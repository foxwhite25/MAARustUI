use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::{c_void, CStr, CString};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

use anyhow::{anyhow, Result};
use futures::Future;
use log::{debug, error, info, trace};
use serde_json::Value;
use tokio::sync::Mutex;

use crate::binding::bind::*;
use crate::binding::event_handler::{maa_callback, CALLBACK_CHANNEL};
use crate::binding::events::*;
use crate::binding::options::MAAOption;
use crate::binding::resources::ItemMap;
use crate::binding::tasks::StoppedTask;

#[derive(Debug)]
pub struct MAAConnection {
    handle: AsstHandle,
    uuid: Arc<Mutex<Option<String>>>,
    target: String,
    id: i64,
    pub wakes: Arc<Mutex<HashMap<i32, Value>>>,
    finished: Arc<Mutex<bool>>,
    item_map: ItemMap,
}

fn find_it<P>(exe_name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&exe_name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    })
}

pub struct MAABuilder<'a> {
    resources_path: PathBuf,
    adb_address: &'a str,
    incremental_path: Option<PathBuf>,
    adb_path: Option<PathBuf>,
    work_dir: Option<PathBuf>,
    callback: Option<AsstApiCallback>,
    adb_config: Option<&'a str>,
    maa_settings: MAAOption,
}

impl<'a> MAABuilder<'a> {
    pub fn new<P: AsRef<Path>>(resources_path: P, adb_address: &'a str) -> Self {
        Self {
            resources_path: resources_path.as_ref().to_path_buf(),
            adb_address,
            incremental_path: None,
            adb_path: None,
            work_dir: None,
            callback: Some(Some(maa_callback)),
            adb_config: None,
            maa_settings: MAAOption::default(),
        }
    }

    pub fn with_incremental_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.incremental_path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn with_adb_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.adb_path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn with_work_dir<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.work_dir = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn with_callback(mut self, callback: Option<AsstApiCallback>) -> Self {
        self.callback = callback;
        self
    }

    pub fn with_adb_config(mut self, config: Option<&'a str>) -> Self {
        self.adb_config = config;
        self
    }

    pub fn with_maa_settings(mut self, settings: MAAOption) -> Self {
        self.maa_settings = settings;
        self
    }

    fn load_resource<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref().as_os_str().as_os_str_bytes();
        let ret = unsafe {
            let path = CString::new(path)?;
            AsstLoadResource(path.as_ptr())
        };
        match ret {
            1 => Ok(()),
            _ => Err(anyhow!("Unknown Error: {ret}")),
        }
    }

    fn set_working_directory<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref().as_os_str().as_os_str_bytes();
        let ret = unsafe {
            let path = CString::new(path)?;
            AsstSetUserDir(path.as_ptr())
        };
        match ret {
            1 => Ok(()),
            _ => Err(anyhow!("Unknown Error: {ret}")),
        }
    }

    fn create_connection(call_back: AsstApiCallback) -> Result<(AsstHandle, i64)> {
        let id = rand::random::<i64>();

        let handle = unsafe { AsstCreateEx(call_back, id as *mut c_void) };
        if handle.is_null() {
            Err(anyhow!("Failed to create handle"))
        } else {
            Ok((handle, id))
        }
    }

    fn connect_with_adb(&self, handle: AsstHandle) -> Result<i32> {
        let path = self.adb_path.clone().unwrap_or(find_it("adb").unwrap());
        debug!("Adb path: {}", path.display());
        debug!("Adb address: {}", self.adb_address);
        debug!("Adb config: {}", self.adb_config.unwrap_or("General"));
        unsafe {
            let c_adb_path = CString::new(path.as_os_str().as_os_str_bytes())?;
            let c_address = CString::new(self.adb_address)?;
            let c_cfg_ptr = CString::new(self.adb_config.unwrap_or("General"))?;
            let async_id = AsstAsyncConnect(
                handle,
                c_adb_path.as_ptr(),
                c_address.as_ptr(),
                c_cfg_ptr.as_ptr(),
                1,
            );
            if async_id != 0 {
                Ok(async_id)
            } else {
                Err(anyhow!("Unknown Error: {async_id}"))
            }
        }
    }

    pub async fn build(&self) -> Result<MAAConnection> {
        if let Some(path) = &self.work_dir {
            info!("Setting working directory to {}", path.display());
            Self::set_working_directory(path)?;
        }

        info!("Loading resources to {}", self.resources_path.display());
        Self::load_resource(&self.resources_path)?;

        let item_map = self.resources_path.join("item_index.json");
        if !item_map.is_file() {
            error!("Item index not found");
            return Err(anyhow!("Item index not found"));
        }
        let item_map = std::fs::read_to_string(item_map)?;
        let item_map: ItemMap = serde_json::from_str(&item_map)?;

        if let Some(path) = &self.incremental_path {
            info!("Loading incremental resources to {}", path.display());
            Self::load_resource(path)?;
        }

        info!("Creating connection to {}", self.adb_address);
        let id = Self::create_connection(self.callback.unwrap())?;
        let handle = id.0;
        let id = id.1;
        let mut maa = MAAConnection {
            handle,
            uuid: Arc::new(Mutex::new(None)),
            target: self.adb_address.to_string(),
            id,
            wakes: Arc::new(Mutex::new(HashMap::new())),
            finished: Arc::new(Mutex::new(false)),
            item_map,
        };
        let settings = self.maa_settings.to_map();
        for (k, v) in settings {
            maa.set_option(k as AsstInstanceOptionKey, &v)?;
        }
        maa.start_polling().await;
        let async_id = self.connect_with_adb(handle)?;

        let k: Value = CallbackWatcher {
            id: async_id,
            wakes: maa.wakes.clone(),
        }
        .await;
        match k {
            Value::Bool(b) => {
                if b {
                    info!("Connected to MAA");
                } else {
                    return Err(anyhow!("Failed to connect to MAA"));
                }
            }
            _ => return Err(anyhow!("Unknown Return Value From Callback: {k}")),
        }

        Ok(maa)
    }
}

impl MAAConnection {
    pub fn get_version(&self) -> Result<String> {
        unsafe {
            let c = AsstGetVersion();
            let ret = CStr::from_ptr(c).to_str()?.to_string();
            Ok(ret)
        }
    }

    fn set_option(&mut self, option: AsstInstanceOptionKey, value: &str) -> Result<()> {
        let c_option_value = CString::new(value)?;
        let ret = unsafe { AsstSetInstanceOption(self.handle, option, c_option_value.as_ptr()) };
        match ret {
            1 => Ok(()),
            _ => Err(anyhow!("Unknown Error: {ret}")),
        }
    }

    fn poll() -> Option<Events> {
        let channel = CALLBACK_CHANNEL.1.clone();
        let future = channel.lock().unwrap();
        match future.recv() {
            Ok(res) => Some(res),
            Err(env) => {
                error!("Recv error: {}", env);
                None
            }
        }
    }

    async fn start_polling(&mut self) {
        let wakes = self.wakes.clone();
        let uuid = self.uuid.clone();
        let finish = self.finished.clone();
        tokio::spawn(async move {
            info!("Polling started");
            loop {
                let Some(resp) = Self::poll() else { break; };
                let finished = finish.lock().await;
                if *finished {
                    break;
                }
                match resp.type_ {
                    AsstMsg::InternalError => trace!("Received: {:?}", resp),
                    AsstMsg::InitFailed => handle_init_failed(resp.params).await,
                    AsstMsg::ConnectionInfo => handle_connection_info(&uuid, resp.params).await,
                    AsstMsg::AllTasksCompleted => trace!("Received: {:?}", resp),
                    AsstMsg::AsyncCallInfo => handle_async_call_info(&wakes, resp.params).await,
                    AsstMsg::TaskChainError => trace!("Received: {:?}", resp),
                    AsstMsg::TaskChainStart => trace!("Received: {:?}", resp),
                    AsstMsg::TaskChainCompleted => trace!("Received: {:?}", resp),
                    AsstMsg::TaskChainExtraInfo => trace!("Received: {:?}", resp),
                    AsstMsg::TaskChainStopped => trace!("Received: {:?}", resp),
                    AsstMsg::SubTaskError => trace!("Received: {:?}", resp),
                    AsstMsg::SubTaskStart => handle_sub_task_start(resp.params).await,
                    AsstMsg::SubTaskCompleted => trace!("Received: {:?}", resp),
                    AsstMsg::SubTaskExtraInfo => handle_sub_task_extra_info(resp.params).await,
                    AsstMsg::SubTaskStopped => trace!("Received: {:?}", resp),
                }
            }
        });
    }

    pub fn append_task<'a>(&mut self, task: impl StoppedTask<'a>) -> Result<i32> {
        let id = CString::new(task.name())?;
        let c_task = CString::new(task.to_json())?;
        debug!("Appending task: {}", task.name());
        let ret = unsafe { AsstAppendTask(self.handle, id.as_ptr(), c_task.as_ptr()) };
        Ok(ret)
    }

    pub fn start(&self) -> Result<()> {
        info!("Starting MAA");
        let ret = unsafe { AsstStart(self.handle) };
        match ret {
            1 => Ok(()),
            _ => Err(anyhow!("Unknown Error: {ret}")),
        }
    }

    pub fn stop(&self) {
        unsafe {
            AsstStop(self.handle);
        }
    }

    pub fn is_running(&self) -> bool {
        unsafe {
            let ret = AsstRunning(self.handle);
            matches!(ret, 1)
        }
    }

    pub fn destroy(self) {
        self._destroy();
    }

    fn _destroy(&self) {
        let mut finish = self.finished.blocking_lock();
        *finish = true;
        let channel = CALLBACK_CHANNEL.0.clone();
        let future = channel.lock().unwrap();
        future
            .send(Events {
                type_: AsstMsg::InternalError,
                params: Default::default(),
            })
            .expect("Failed to send destroy event");
        unsafe {
            AsstDestroy(self.handle);
        }
    }
}

impl Drop for MAAConnection {
    fn drop(&mut self) {
        self._destroy()
    }
}

struct CallbackWatcher {
    id: i32,
    wakes: Arc<Mutex<HashMap<i32, Value>>>,
}

impl Future for CallbackWatcher {
    type Output = Value;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut wakes = self.wakes.blocking_lock();
        if wakes.contains_key(&self.id) {
            let value = wakes.get(&self.id).unwrap().clone();
            wakes.remove(&self.id);
            Poll::Ready(value)
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
