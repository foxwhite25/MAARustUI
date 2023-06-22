use std::sync::{Arc, Mutex};
use log::{debug, error, info, warn};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

enum ConnectionInfoWhat {
    ConnectFailed,
    Connected,
    UuidGot,
    UnsupportedResolution,
    ResolutionError,
    Reconnecting,
    Reconnected,
    Disconnect,
    ScreencapFailed,
    TouchModeNotAvailable,
    ResolutionGot,
    Unknown
}

impl From<&str> for ConnectionInfoWhat {
    fn from(s: &str) -> Self {
        match s {
            "ConnectFailed" => ConnectionInfoWhat::ConnectFailed,
            "Connected" => ConnectionInfoWhat::Connected,
            "UuidGot" => ConnectionInfoWhat::UuidGot,
            "UnsupportedResolution" => ConnectionInfoWhat::UnsupportedResolution,
            "ResolutionError" => ConnectionInfoWhat::ResolutionError,
            "Reconnecting" => ConnectionInfoWhat::Reconnecting,
            "Reconnected" => ConnectionInfoWhat::Reconnected,
            "Disconnect" => ConnectionInfoWhat::Disconnect,
            "ScreencapFailed" => ConnectionInfoWhat::ScreencapFailed,
            "TouchModeNotAvailable" => ConnectionInfoWhat::TouchModeNotAvailable,
            "ResolutionGot" => ConnectionInfoWhat::ResolutionGot,
            _ => {
                warn!("Unknown ConnectionInfoWhat: {:?}", s);
                ConnectionInfoWhat::Unknown
            },
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConnectionInfo {
    pub what: String,
    pub why: Option<String>,
    pub uuid: String,
    pub details: ConnectionInfoDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConnectionInfoDetails {
    pub adb: String,
    pub address: String,
    pub config: String,
    pub height: Option<i64>,
    pub width: Option<i64>,
}

pub async fn handle_connection_info(uuid: &Arc<Mutex<Option<String>>>, params: Value) {
    let async_call_info: ConnectionInfo = serde_json::from_value(params).unwrap();
    let connection_info_what = ConnectionInfoWhat::from(async_call_info.what.as_str());
    match connection_info_what {
        ConnectionInfoWhat::ConnectFailed => {
            error!("Connection Failed: {:?}", async_call_info.why)
        }
        ConnectionInfoWhat::Connected => {
            debug!("Connected: {:?}", async_call_info.why)
        }
        ConnectionInfoWhat::UuidGot => {
            let mut uuid = uuid.lock().unwrap();
            debug!("Got UUID: {}", async_call_info.uuid);
            *uuid = Some(async_call_info.uuid);
        }
        ConnectionInfoWhat::UnsupportedResolution => {
            error!("Unsupported Resolution: {:?}", async_call_info.why)
        }
        ConnectionInfoWhat::ResolutionError => {
            error!("Resolution Error: {:?}", async_call_info.why)
        }
        ConnectionInfoWhat::Reconnecting => {
            info!("Reconnecting: {:?}", async_call_info.why)
        }
        ConnectionInfoWhat::Reconnected => {
            info!("Reconnected: {:?}", async_call_info.why)
        }
        ConnectionInfoWhat::Disconnect => {
            info!("Disconnected: {:?}", async_call_info.why)
        }
        ConnectionInfoWhat::ScreencapFailed => {
            error!("Screencap Failed: {:?}", async_call_info.why)
        }
        ConnectionInfoWhat::TouchModeNotAvailable => {
            error!("Touch Mode Not Available: {:?}", async_call_info.why)
        }
        ConnectionInfoWhat::ResolutionGot => {
            info!("Device Resolution: {}x{}", async_call_info.details.width.unwrap(), async_call_info.details.height.unwrap())
        }
        ConnectionInfoWhat::Unknown => {
            warn!("Unknown ConnectionInfoWhat: {}", async_call_info.what)
        }
    }
}
