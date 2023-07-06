use maa_rust_ui::gui::WrapApp;

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some([1280.0, 1024.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "egui demo app",
        options,
        Box::new(|cc| Box::new(WrapApp::new(cc))),
    )
}