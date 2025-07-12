use eframe::egui::Ui;
use crate::ui::state::AppState;

pub fn draw_dashboard(ui: &mut Ui, state: &mut AppState) {
    // TODO: Релизовать панельку
    ui.heading("Главная");
    ui.separator();
    ui.add_space(16.0);
    if ui.button("Настройки").clicked() {
        state.ui_state = crate::ui::state::UiState::Settings;
    }
}