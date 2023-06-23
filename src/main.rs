use std::thread;

use log::info;

use maa_rust_ui::binding::connection::MAABuilder;
use maa_rust_ui::binding::options::{MAAOption, TouchMode};
use maa_rust_ui::binding::tasks::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
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
        .use_medicine(1)
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

    Mall::new_paused()
        .set_shopping(true)
        .set_buy_first(vec![ShopItem::LMD, ShopItem::RecruitmentPermit])
        .set_blacklist(vec![ShopItem::FurniturePart, ShopItem::FurniturePart])
        .set_force_shopping_if_credit_full(true)
        .append_in(&mut m)
        .unwrap();

    info!(
        "You are running MAA version: {}; MAARustUI: v{}, enjoy!",
        m.get_version().unwrap(),
        VERSION
    );

    m.start().unwrap();

    while m.is_running() {
        thread::sleep(std::time::Duration::from_secs(1));
    }
    info!("MAA have stopped");
    std::process::exit(0);
}
