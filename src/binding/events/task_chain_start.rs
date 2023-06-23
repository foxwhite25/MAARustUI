use log::info;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TaskChainStart {
    taskchain: String,
    taskid: i64,
    uuid: String,
}

pub async fn handle_task_chain_start(params: Value) {
    let async_call_info: TaskChainStart = serde_json::from_value(params).unwrap();
    info!("Start Task {}: {}", async_call_info.taskid, async_call_info.taskchain);
}
