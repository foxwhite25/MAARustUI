use std::collections::HashMap;
use std::sync::Arc;

use log::debug;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct AsyncCallInfo {
    pub uuid: String,
    pub what: String,
    pub async_call_id: i32,
    pub details: AsyncCallInfoDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct AsyncCallInfoDetails {
    pub ret: Value,
    pub cost: i32,
}

fn wake(wakes: &Arc<std::sync::Mutex<HashMap<i32, Value>>>, async_id: i32, ret: Value) {
    let mut map = wakes.lock().unwrap();
    map.insert(async_id, ret);
}

pub async fn handle_async_call_info(wakes: &Arc<std::sync::Mutex<HashMap<i32, Value>>>, params: Value) {
    let async_call_info: AsyncCallInfo = serde_json::from_value(params).unwrap();
    debug!("async_call_info: {:?}", async_call_info);
    wake(
        wakes,
        async_call_info.async_call_id,
        async_call_info.details.ret,
    );
}
