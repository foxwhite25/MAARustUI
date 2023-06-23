use log::trace;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct SubTaskStart {
    class: String,
    details: SubTaskStartDetails,
    #[serde(default)]
    first: Vec<String>,
    pre_task: Option<String>,
    subtask: String,
    taskchain: String,
    taskid: i64,
    uuid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubTaskStartDetails {
    action: Option<String>,
    algorithm: Option<String>,
    exec_times: Option<i64>,
    max_times: Option<i64>,
    task: Option<String>,
}

pub async fn handle_sub_task_start(params: Value) {
    let async_call_info: SubTaskStart = serde_json::from_value(params).unwrap();
    trace!("sub_task_start: {:?}", async_call_info);
}
