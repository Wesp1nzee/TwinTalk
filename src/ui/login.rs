use eframe::egui::{Ui, TextEdit};
use crate::ui::state::{AppState, UiState};

pub fn login_user(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.label("Введите своё имя:");
        ui.add(TextEdit::singleline(&mut state.user_name)
            .hint_text("Иван")
        );
    });
    ui.horizontal_centered(|ui| {
        if ui.button("Далее").clicked() && !state.temp_callee.is_empty() {
            state.ui_state = UiState::OutgoingCall { callee_id: state.temp_callee.clone() };
        }
    });
}