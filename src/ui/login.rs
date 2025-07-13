use eframe::egui::{Ui, TextEdit};
use crate::ui::state::main::{AppState, UiState};

pub fn login_user(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.label("Введите своё имя:");
        ui.add(TextEdit::singleline(&mut state.user_name)
            .hint_text("Иван")
        );
    });
    ui.horizontal_centered(|ui| {
        if ui.button("Далее").clicked(){
            state.ui_state = UiState::Dashboard;
        }
    });
}