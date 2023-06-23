use log::{debug, info};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct SubTaskExtraInfo {
    class: String,
    details: Details,
    #[serde(default)]
    first: Vec<String>,
    pre_task: Option<String>,
    subtask: String,
    taskchain: String,
    taskid: i64,
    uuid: String,
    what: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct Details {
    exec_times: Option<i64>,
    limit_type: Option<String>,
    max_times: Option<i64>,
    task: Option<String>,
    drops: Option<Vec<Drop>>,
    stage: Option<Stage>,
    stars: Option<i64>,
    #[serde(default)]
    stats: Vec<Stat>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Drop {
    drop_type: String,
    item_id: String,
    item_name: String,
    quantity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Stage {
    stage_code: String,
    stage_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Stat {
    add_quantity: i64,
    item_id: String,
    item_name: String,
    quantity: i64,
}

pub async fn handle_sub_task_extra_info(params: Value) {
    let async_call_info: SubTaskExtraInfo = serde_json::from_value(params).unwrap();
    match async_call_info.what.as_str() {
        "asst::StageDropsTaskPlugin" => {
            info!(
                "Finished battle with {} star at stage {}",
                async_call_info.details.stars.unwrap(),
                async_call_info.details.stage.as_ref().unwrap().stage_code
            );
            info!("Dropped items:");
            for drop in async_call_info.details.drops.as_ref().unwrap() {
                info!("{}x{}", drop.item_name, drop.quantity);
            }

            info!("Total items:");
            for stat in async_call_info.details.stats {
                info!("{}x{}", stat.item_name, stat.quantity);
            }
        }
        _ => {
            debug!("sub_task_extra_info: {:?}", async_call_info)
        }
    }
}
