use std::ffi::{c_char, c_int, c_void, CStr};
use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{channel, Sender, Receiver};
use serde_json::Value;
use crate::binding::event::{AsstMsg, Events};

lazy_static!(
    pub static ref CALLBACK_CHANNEL: (Arc<Mutex<Sender<Events>>>, Arc<Mutex<Receiver<Events>>>) = {
        let (tx, rx) = channel();
        (Arc::new(Mutex::new(tx)), Arc::new(Mutex::new(rx)))
    };
);

#[allow(unused_variables)]
#[allow(unused_must_use)]
pub unsafe extern "C" fn maa_callback(
    msg: c_int,
    detail_json: *const c_char,
    id: *mut c_void,
) {
    std::panic::catch_unwind(|| {
        let body = CStr::from_ptr(detail_json)
            .to_string_lossy();
        let body: Value = serde_json::from_str(&body).unwrap();
        let type_ = match msg {
            0 => AsstMsg::InternalError,
            1 => AsstMsg::InitFailed,
            2 => AsstMsg::ConnectionInfo,
            3 => AsstMsg::AllTasksCompleted,
            4 => AsstMsg::AsyncCallInfo,
            10000 => AsstMsg::TaskChainError,
            10001 => AsstMsg::TaskChainStart,
            10002 => AsstMsg::TaskChainCompleted,
            10003 => AsstMsg::TaskChainExtraInfo,
            10004 => AsstMsg::TaskChainStopped,
            20000 => AsstMsg::SubTaskError,
            20001 => AsstMsg::SubTaskStart,
            20002 => AsstMsg::SubTaskCompleted,
            20003 => AsstMsg::SubTaskExtraInfo,
            20004 => AsstMsg::SubTaskStopped,
            _ => panic!("Unknown message type: {}", msg),
        };
        let task = Events {
            type_,
            params: body,
        };
        CALLBACK_CHANNEL.0.lock().unwrap().send(task);
    });
}