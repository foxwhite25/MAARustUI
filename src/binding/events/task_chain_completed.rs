use log::info;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TaskChainCompleted {
    taskchain: String,
    taskid: i64,
    uuid: String,
}

pub async fn handle_task_chain_completed(params: Value) {
    let async_call_info: TaskChainCompleted = serde_json::from_value(params).unwrap();
    info!(
        "Task {}({}) Finished",
        async_call_info.taskid, async_call_info.taskchain
    );
}
