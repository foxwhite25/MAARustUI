use std::thread;

use log::info;

use maa_rust_ui::binding::connection::MAABuilder;
use maa_rust_ui::binding::options::{MAAOption, TouchMode};
use maa_rust_ui::binding::tasks::*;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();
    let setting = MAAOption::default().with_touch_mode(TouchMode::MAATouch);

    let mut m = MAABuilder::new("/home/fox_white/MAA", "192.168.240.112:5555")
        .with_work_dir("/home/fox_white/LatexProject/MaaAssistantArknights/src/maa_rust_ui/logs")
        .with_incremental_path("/home/fox_white/MAA/resource/global/YoStarEN")
        .with_maa_settings(setting)
        .build()
        .await
        .unwrap();

    StartUp::new()
        .set_client_type(ClientType::YoStarEN)
        .append_in(&mut m)
        .unwrap();

    Fight::new()
        .server(Server::US)
        .client_type(ClientType::YoStarEN)
        .append_in(&mut m)
        .unwrap();

    Recruit::new()
        .refresh(true)
        .times(16)
        .server(Server::US)
        .append_in(&mut m)
        .unwrap();

    m.start().unwrap();

    info!("You are running MAA version: {}", m.get_version().unwrap());
    while m.is_running() {
        thread::sleep(std::time::Duration::from_secs(1));
    }
    info!("MAA is stopped");
}
