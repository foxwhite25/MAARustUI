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
    let client = ClientType::YoStarEN;
    let server = Server::US;

    let mut m = MAABuilder::new("/home/fox_white/MAA", "192.168.240.112:5555")
        .with_work_dir("/home/fox_white/LatexProject/MaaAssistantArknights/src/maa_rust_ui/logs")
        .with_incremental_path("/home/fox_white/MAA/resource/global/YoStarEN")
        .with_maa_settings(setting)
        .build()
        .await
        .unwrap();

    let _start_up = StartUp::new()
        .set_client_type(client)
        .append_in(&mut m)
        .unwrap()
        .start();

    let _fight = Fight::new()
        .use_medicine(1)
        .server(server)
        .client_type(client)
        .append_in(&mut m)
        .unwrap()
        .start();

    let _recruit = Recruit::new()
        .refresh(true)
        .times(16)
        .server(server)
        .append_in(&mut m)
        .unwrap()
        .start();

    let _mall = Mall::new()
        .shopping(true)
        .buy_first(vec![ShopItem::LMD, ShopItem::RecruitmentPermit])
        .blacklist(vec![ShopItem::CarbonStick, ShopItem::FurniturePart])
        .force_buy_when_full(true)
        .append_in(&mut m)
        .unwrap()
        .start();

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
