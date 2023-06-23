use log::{error, info};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TaskChainError {
    taskchain: String,
    taskid: i64,
    uuid: String,
}

pub async fn handle_task_chain_error(params: Value) {
    let async_call_info: TaskChainError = serde_json::from_value(params).unwrap();
    error!("Task {} Errored: {}", async_call_info.taskid, async_call_info.taskchain);
}
