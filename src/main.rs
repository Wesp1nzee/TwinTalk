mod ui;
use eframe::{NativeOptions, Result};
use ui::app::MyApp;

fn main() -> Result<()> {
    let options = NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([800.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My VoIP",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
