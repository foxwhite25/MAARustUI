#[derive(Default)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct About {}

impl super::Demo for About {
    fn name(&self) -> &'static str {
        "About MAA Rust UI"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .default_width(320.0)
            .open(open)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl super::View for About {
    fn ui(&mut self, ui: &mut egui::Ui) {
        use egui::special_emojis::{OS_APPLE, OS_LINUX, OS_WINDOWS};

        ui.heading("MAA Rust UI");
        ui.label(format!(
            "MAA Rust UI is a MAA wrapper that, thanks to egui, runs natively on {}, {}, {}.",
            OS_APPLE, OS_LINUX, OS_WINDOWS,
        ));
        ui.label("MAA Rust UI is designed to be easy to use, portable, and fast.");

        ui.add_space(12.0); // ui.separator();
        ui.heading("Reason To Exist");
        ui.label("The Official Client only support native windows, which I don't use. \nOn the other hand, MAAX plan to support linux as it is just written in Vue, but it doesn't right now, also it doesn't support global server.");

        ui.add_space(12.0); // ui.separator();
        ui.heading("Links");
        links(ui);
    }
}

fn links(ui: &mut egui::Ui) {
    use egui::special_emojis::GITHUB;
    ui.hyperlink_to(
        format!("{} MAA on GitHub", GITHUB),
        "https://github.com/MaaAssistantArknights/MaaAssistantArknights",
    );
    ui.hyperlink_to(
        format!("{} MAA Rust UI", GITHUB),
        "https://github.com/foxwhite25/MAARustUI",
    );
}
