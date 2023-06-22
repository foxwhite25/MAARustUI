
use std::{thread};
use log::info;
use RustUI::binding::connection::{MAABuilder};

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();
    let m = MAABuilder::new("/home/fox_white/MAA", "192.168.240.112:5555")
        .with_work_dir("/home/fox_white/LatexProject/MaaAssistantArknights/src/RustUI/logs")
        .with_incremental_path("/home/fox_white/MAA/resource/global/YoStarEN")
        .build().await.unwrap();

    info!("{}", m.get_version().unwrap());
    thread::sleep(std::time::Duration::from_secs(10));
}
