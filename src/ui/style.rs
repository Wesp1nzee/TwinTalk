use eframe::egui::{self, Visuals};

pub fn setup_ctx(ctx: &egui::Context) {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let mut style = (*ctx.style()).clone();
        style.spacing.button_padding = egui::vec2(16.0, 12.0);
        style.spacing.item_spacing   = egui::vec2(12.0, 8.0);
        style.text_styles
             .get_mut(&egui::TextStyle::Button).unwrap().size = 20.0;
        style.text_styles
             .get_mut(&egui::TextStyle::Body).unwrap().size   = 18.0;
        ctx.set_visuals(Visuals::light());
        ctx.set_style(style);
    });
}
