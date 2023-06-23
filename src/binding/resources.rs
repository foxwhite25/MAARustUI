use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

pub type ItemMap = HashMap<String, Item>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub classify_type: String,
    pub description: Option<String>,
    pub icon: String,
    pub name: String,
    pub sort_id: i64,
    pub usage: Option<String>,
}
