mod app;

use eframe::egui;
use app::IronmanGg;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 1080.0)),
        ..Default::default()
    };
    eframe::run_native(
            "Ironman gg",
        options,
        Box::new(|_cc| Box::new(IronmanGg::default())),
    )
}