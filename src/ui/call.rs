use eframe::egui::{Ui, TextEdit};
use crate::ui::state::{AppState, UiState, CallEndReason};

pub fn draw_call(ui: &mut Ui, state: &mut AppState) {
    if let UiState::OutgoingCall { callee_id } = &state.ui_state {
        let target = callee_id.clone();
        ui.label(format!("Звонок на {} инициирован…", target));
        if ui.button("Отменить").clicked() {
            state.ui_state = UiState::Dashboard;
        }
        return;
    }
    if let UiState::IncomingCall { caller_id } = &state.ui_state {
        let caller = caller_id.clone();
        ui.label(format!("Входящий звонок от {}", caller));
        ui.horizontal(|ui| {
            if ui.button("Принять").clicked() {
                state.ui_state = UiState::InCall { peer_id: caller.clone(), is_video: false };
            }
            if ui.button("Отклонить").clicked() {
                state.ui_state = UiState::Dashboard;
            }
        });
        return;
    }
    if let UiState::InCall { peer_id, is_video } = &mut state.ui_state {
        let peer = peer_id.clone();
        let video = *is_video;
        ui.label(format!("Вызов с {} (видео: {})", peer, video));
        ui.horizontal(|ui| {
            if ui.button("Завершить").clicked() {
                state.ui_state = UiState::CallEnded { reason: CallEndReason::UserHungUp };
            }
            if ui.button(if video { "Отключить камеру" } else { "Включить камеру" }).clicked() {
                state.toggle_video();
            }
        });
        return;
    }

    if let UiState::CallEnded { reason } = &state.ui_state {
        ui.label(format!("Звонок завершён: {:?}", reason));
        if ui.button("ОК").clicked() {
            state.ui_state = UiState::Dashboard;
        }
        return;
    }
}

pub fn render_call_input_ui(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.label("Введите ID для звонка:");
        ui.add(TextEdit::singleline(&mut state.temp_callee)
            .hint_text("ID...")
        );
    });
    ui.horizontal_centered(|ui| {
        if ui.button("Позвонить").clicked() && !state.temp_callee.is_empty() {
            state.ui_state = UiState::OutgoingCall { callee_id: state.temp_callee.clone() };
        }
    });
    ui.add_space(10.0);
    ui.horizontal_centered(|ui| {
        ui.label("Ваш ID:");
        ui.monospace(state.user_id.to_string());
    });
}
