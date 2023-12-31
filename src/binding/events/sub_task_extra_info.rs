use log::{info, trace, warn};
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
    #[serde(default)]
    tags: Vec<String>,
    level: Option<i64>,
    #[serde(default)]
    result: Vec<Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub level: i64,
    pub opers: Vec<Operator>,
    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operator {
    pub level: i64,
    pub name: String,
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
    match async_call_info.class.as_str() {
        "asst::StageDropsTaskPlugin" => {
            info!(
                "Finished battle with {} star at stage {}...",
                async_call_info.details.stars.unwrap(),
                async_call_info.details.stage.as_ref().unwrap().stage_code
            );
            info!("Dropped items:");
            for drop in async_call_info.details.drops.unwrap() {
                info!("{} x {}", drop.item_name, drop.quantity);
            }
            info!("");
            info!("Total items:");
            for stat in async_call_info.details.stats {
                info!("{} x {}", stat.item_name, stat.quantity);
            }
        }
        "asst::AutoRecruitTask" if !async_call_info.details.result.is_empty() => {
            let tags_str = async_call_info
                .details
                .tags
                .iter()
                .map(|tag| tag.as_str())
                .collect::<Vec<&str>>()
                .join(", ");
            info!("Recruit tags: {}", tags_str);
            let recruit_star_level = async_call_info.details.level.unwrap();
            if recruit_star_level >= 5 {
                warn!("Good star level: {}", recruit_star_level);
                let max_star_level = async_call_info
                    .details
                    .result
                    .iter()
                    .max_by_key(|result| result.level)
                    .unwrap()
                    .level;

                for result in async_call_info
                    .details
                    .result
                    .iter()
                    .filter(|result| result.level == max_star_level)
                {
                    let opers_str = result
                        .opers
                        .iter()
                        .map(|oper| oper.name.as_str())
                        .collect::<Vec<&str>>()
                        .join(", ");
                    let tags_str = result
                        .tags
                        .iter()
                        .map(|tag| tag.as_str())
                        .collect::<Vec<&str>>()
                        .join(", ");
                    warn!("Tags Combo: [{}] | Operators: [{}]", tags_str, opers_str)
                }
            } else {
                info!("Recruit star level: {}", recruit_star_level);
            }
        }
        _ => {
            trace!("sub_task_extra_info: {:?}", async_call_info)
        }
    }
}
