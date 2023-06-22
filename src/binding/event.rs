use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use serde_json::Value;

use serde::Deserialize;
use serde::Serialize;

pub enum AsstMsg {
    // 内部错误
    InternalError = 0,
    // 初始化失败
    InitFailed = 1,
    // 连接相关信息
    ConnectionInfo = 2,
    // 全部任务完成
    AllTasksCompleted = 3,
    // 外部异步调用信息
    AsyncCallInfo = 4,

    /* TaskChain Info */
    // 任务链执行/识别错误
    TaskChainError = 10000,
    // 任务链开始
    TaskChainStart = 10001,
    // 任务链完成
    TaskChainCompleted = 10002,
    // 任务链额外信息
    TaskChainExtraInfo = 10003,
    // 任务链手动停止
    TaskChainStopped = 10004,

    /* SubTask Info */
    // 原子任务执行/识别错误
    SubTaskError = 20000,
    // 原子任务开始
    SubTaskStart = 20001,
    // 原子任务完成
    SubTaskCompleted = 20002,
    // 原子任务额外信息
    SubTaskExtraInfo = 20003,
    // 原子任务手动停止
    SubTaskStopped = 20004,
}

pub struct Events {
    pub type_: AsstMsg,
    pub params: Value,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AsyncCallInfo {
    pub uuid: String,
    pub what: String,
    pub async_call_id: i32,
    pub details: AsyncCallInfoDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AsyncCallInfoDetails {
    pub ret: bool,
    pub cost: i32,
}

pub fn wake(wakes: &Arc<Mutex<HashSet<i32>>>, async_id: i32) {
    let mut map = wakes.lock().unwrap();
    map.insert(async_id);
}


pub async fn handle_async_call_info(wakes: &Arc<Mutex<HashSet<i32>>>, params: Value) {
    let async_call_info: AsyncCallInfo = serde_json::from_value(params).unwrap();
    wake(wakes, async_call_info.async_call_id);
    println!("async_call_info: {:?}", async_call_info);
}
