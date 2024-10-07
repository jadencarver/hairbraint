mod app;
use crate::app::*;

fn main() -> eframe::Result<()> {
    let mut app = App::new();

    let mut options = eframe::NativeOptions::default();
    //options.initial_window_size = Some(egui::vec2(1920.0, 1080.0));

    eframe::run_native(
        &app.title(),
        options,
        Box::new(|ctx| Ok(Box::new(app))),
    )
}
