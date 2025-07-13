// src/ui/app.rs
use eframe::{egui, App as EguiApp, Frame};

use crate::ui::style::setup_ctx;
use crate::ui::{
    login::login_user,
    dashboard::draw_dashboard,
    call::draw_call,
    state::main::{AppState, UiState},
};

pub struct MyApp {
    pub state: AppState,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { state: AppState::new() }
    }
}

impl EguiApp for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        setup_ctx(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            match &mut self.state.ui_state {
                UiState::Login => login_user(ui, &mut self.state),
                UiState::Dashboard => draw_dashboard(ui, &mut self.state),
                UiState::OutgoingCall { .. } |
                UiState::IncomingCall { .. } |
                UiState::InCall { .. } |
                UiState::CallEnded { .. } => draw_call(ui, &mut self.state),
                UiState::Settings => {
                    ui.label("Настройки");
                },
                UiState::Error { message } => {
                    ui.colored_label(egui::Color32::RED, message);
                },
            }
        });
    }
}
