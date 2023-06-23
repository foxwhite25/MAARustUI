use log::error;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub use async_call_info::*;
pub use connection_info::*;

mod async_call_info;
mod connection_info;

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct Events {
    pub type_: AsstMsg,
    pub params: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitFailed {
    pub what: String,
    pub why: String,
    pub details: Value,
}

pub async fn handle_init_failed(params: Value) {
    let init_failed: InitFailed = serde_json::from_value(params).unwrap();
    error!("init_failed: {:?}", init_failed);
}
