use std::collections::{HashMap, HashSet};
use std::ffi::{c_void, CStr, CString, OsStr, OsString};
use std::path::{Path, PathBuf};
use crate::binding::bind::*;
use anyhow::{anyhow, Result};
use crate::binding::event_handler::{CALLBACK_CHANNEL, maa_callback};
use std::env;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use std::task::{Context, Poll, Waker};
use log::{error, info};
use crate::binding::event::{AsstMsg, Events, handle_async_call_info};

#[derive(Debug, Clone)]
pub struct Task {
    pub id: i32,
    pub type_: String,
    pub params: String,
}

#[derive(Debug)]
pub struct MAAConnection {
    handle: AsstHandle,
    uuid: Option<String>,
    target: String,
    tasks: HashMap<i32, Task>,
    id: i64,
    pub wakes: Arc<Mutex<HashSet<i32>>>,
}

fn find_it<P>(exe_name: P) -> Option<PathBuf>
    where P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).filter_map(|dir| {
            let full_path = dir.join(&exe_name);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        }).next()
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

    fn create_connection(
        call_back: AsstApiCallback,
    ) -> Result<(AsstHandle, i64)> {
        let id = rand::random::<i64>();

        let handle = unsafe { AsstCreateEx(call_back, id as *mut c_void) };
        if handle.is_null() {
            Err(anyhow!("Failed to create handle"))
        } else {
            Ok((handle, id))
        }
    }

    fn connect_with_adb(
        &self,
        handle: AsstHandle,
    ) -> Result<i32> {
        let path = self.adb_path.clone().unwrap_or(find_it("adb").unwrap());
        info!("Adb path: {}", path.display());
        info!("Adb address: {}", self.adb_address);
        info!("Adb config: {}", self.adb_config.unwrap_or("General"));
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
        info!("Loading resources to {}", self.resources_path.display());
        Self::load_resource(&self.resources_path)?;
        if let Some(path) = &self.incremental_path {
            info!("Loading incremental resources to {}", path.display());
            Self::load_resource(path)?;
        }
        if let Some(path) = &self.work_dir {
            info!("Setting working directory to {}", path.display());
            Self::set_working_directory(path)?;
        }
        info!("Creating connection to {}", self.adb_address);
        let id = Self::create_connection(self.callback.unwrap())?;
        let handle = id.0;
        let id = id.1;
        let mut maa = MAAConnection {
            handle,
            uuid: None,
            target: self.adb_address.to_string(),
            tasks: HashMap::new(),
            id,
            wakes: Arc::new(Mutex::new(HashSet::new())),
        };
        maa.start_polling().await;
        let async_id = self.connect_with_adb(handle)?;

        CallbackWatcher {
            id: async_id,
            wakes: maa.wakes.clone(),
        }.await;

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
        let mut future = channel.lock().unwrap();
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
        tokio::spawn(async move {
            loop {
                let Some(resp) = Self::poll() else { break };
                match resp.type_ {
                    AsstMsg::InternalError => {}
                    AsstMsg::InitFailed => {}
                    AsstMsg::ConnectionInfo => {}
                    AsstMsg::AllTasksCompleted => {}
                    AsstMsg::AsyncCallInfo => {
                        handle_async_call_info(&wakes ,resp.params).await
                    }
                    AsstMsg::TaskChainError => {}
                    AsstMsg::TaskChainStart => {}
                    AsstMsg::TaskChainCompleted => {}
                    AsstMsg::TaskChainExtraInfo => {}
                    AsstMsg::TaskChainStopped => {}
                    AsstMsg::SubTaskError => {}
                    AsstMsg::SubTaskStart => {}
                    AsstMsg::SubTaskCompleted => {}
                    AsstMsg::SubTaskExtraInfo => {}
                    AsstMsg::SubTaskStopped => {}
                }
            }
        });
    }
}

struct CallbackWatcher {
    id: i32,
    wakes: Arc<Mutex<HashSet<i32>>>,
}

impl Future for CallbackWatcher {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut wakes = self.wakes.lock().unwrap();
        if wakes.contains(&self.id) {
            wakes.remove(&self.id);
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}