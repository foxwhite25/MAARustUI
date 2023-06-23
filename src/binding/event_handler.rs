use std::ffi::{c_char, c_int, c_void, CStr};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use serde_json::Value;

use crate::binding::events::{AsstMsg, Events};

type ArcMutexSender = Arc<Mutex<Sender<Events>>>;
type ArcMutexReceiver = Arc<Mutex<Receiver<Events>>>;

lazy_static! {
    pub static ref CALLBACK_CHANNEL: (ArcMutexSender, ArcMutexReceiver) = {
        let (tx, rx) = channel();
        (Arc::new(Mutex::new(tx)), Arc::new(Mutex::new(rx)))
    };
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
/// # Safety
///
/// This function is a callback from the C library, and should not be called directly.
pub unsafe extern "C" fn maa_callback(msg: c_int, detail_json: *const c_char, id: *mut c_void) {
    std::panic::catch_unwind(|| {
        let body = CStr::from_ptr(detail_json).to_string_lossy();
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
