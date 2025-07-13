use eframe::egui::{Ui, ComboBox, Slider};
use crate::ui::state::main::{AppState, UiState};
use crate::ui::state::settings::{SettingsTab, GeneralSettings, AudioSettings, AdvancedSettings};

pub fn draw_dashboard(ui: &mut Ui, state: &mut AppState) {
    // TODO: Релизовать панельку
    ui.heading("Главная");
    ui.separator();
    if ui.button("Настройки").clicked() {
        state.ui_state = UiState::Settings;
    }
}

pub fn draw_settings(ui: &mut Ui, state: &mut AppState) {
    ui.heading("Настройки");
    ui.separator();

    if ui.button("← Назад").clicked() {
        state.ui_state = UiState::Dashboard;
        return;
    }
    ui.separator();

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label("Общие");
            if ui.selectable_label(state.settings.tab == SettingsTab::General, "Общие").clicked() {
                state.settings.tab = SettingsTab::General;
            }
            ui.selectable_label(state.settings.tab == SettingsTab::Audio, "Аудио").on_hover_text("Настройки аудио");
            if ui.selectable_label(state.settings.tab == SettingsTab::Audio, "Аудио").clicked() {
                state.settings.tab = SettingsTab::Audio;
            }
            // Проблемы с библеотекой камеры. Пофиксить

            // if ui.selectable_label(state.settings.tab == SettingsTab::Video, "Видео").clicked() {
            //     state.settings.tab = SettingsTab::Video;
            // }
            if ui.selectable_label(state.settings.tab == SettingsTab::Advanced, "Дополнительно").clicked() {
                state.settings.tab = SettingsTab::Advanced;
            }
        });

        ui.separator();

        ui.vertical(|ui| match state.settings.tab {
            SettingsTab::General => draw_general(ui, &mut state.settings.general),
            SettingsTab::Audio => draw_audio(ui, &mut state.settings.audio),
            // SettingsTab::Video => draw_video(ui, &mut state.settings.video),
            SettingsTab::Advanced => draw_advanced(ui, &mut state.settings.advanced),
        });
    });
}

fn draw_general(ui: &mut Ui, s: &mut GeneralSettings) {
    ui.checkbox(&mut s.theme_dark, "Темная тема");
    ui.horizontal(|ui| {
        ui.label("Язык:");
        ComboBox::from_id_salt("lang_select")
            .selected_text(&s.language)
            .show_ui(ui, |ui| {
                for lang in ["Русский", "English", "Español"].iter() {
                    ui.selectable_value(&mut s.language, lang.to_string(), *lang);
                }
            });
    });
}

fn draw_audio(ui: &mut Ui, s: &mut AudioSettings) {
    ui.horizontal(|ui| {
        ui.label("Входное устройство:");
        ComboBox::from_id_salt("audio_in")
            .selected_text(&s.input_devices[s.selected_input].clone())
            .show_ui(ui, |ui| {
                for (i, dev) in s.input_devices.iter().enumerate() {
                    ui.selectable_value(&mut s.selected_input, i, dev);
                }
            });
    });
    ui.add(Slider::new(&mut s.mic_volume, 0.0..=1.0).text("Громкость микрофона"));

    ui.horizontal(|ui| {
        ui.label("Выходное устройство:");
        ComboBox::from_id_salt("audio_out")
            .selected_text(&s.output_devices[s.selected_output].clone())
            .show_ui(ui, |ui| {
                for (i, dev) in s.output_devices.iter().enumerate() {
                    ui.selectable_value(&mut s.selected_output, i, dev);
                }
            });
    });
    ui.add(Slider::new(&mut s.speaker_volume, 0.0..=1.0).text("Громкость колонок"));

    ui.checkbox(&mut s.noise_suppression, "Шумоподавление");
    ui.checkbox(&mut s.echo_cancellation, "Эхо-подавление");
}
// Проблемы с библеотекой камеры. Пофиксить
// fn draw_video(ui: &mut Ui, s: &mut VideoSettings) {
//     ui.horizontal(|ui| {
//         ui.label("Камера:");
//         ComboBox::from_id_salt("video_cam")
//             .selected_text(&s.camera_devices[s.selected_camera].clone())
//             .show_ui(ui, |ui| {
//                 for (i, cam) in s.camera_devices.iter().enumerate() {
//                     ui.selectable_value(&mut s.selected_camera, i, cam);
//                 }
//             });
//     });
//     ui.horizontal(|ui| {
//         ui.label("Разрешение:");
//         ComboBox::from_id_salt("res_select")
//             .selected_text(&format!("{}x{}", s.resolution.0, s.resolution.1))
//             .show_ui(ui, |ui| {
//                 for &(w, h) in &[(640, 480), (1280, 720), (1920, 1080)] {
//                     ui.selectable_value(&mut s.resolution, (w, h), format!("{}x{}", w, h));
//                 }
//             });
//     });
//     ui.add(Slider::new(&mut s.framerate, 15..=60).text("FPS"));
//     ui.add(Slider::new(&mut s.brightness, 0.0..=1.0).text("Яркость"));
//     ui.add(Slider::new(&mut s.contrast, 0.0..=1.0).text("Контрастность"));
//     ui.checkbox(&mut s.background_blur, "Размытие фона");
// }

fn draw_advanced(ui: &mut Ui, s: &mut AdvancedSettings) {
    ui.checkbox(&mut s.enable_logging, "Включить логирование");
    ui.horizontal(|ui| {
        ui.label("Уровень логов:");
        ComboBox::from_id_salt("log_level")
            .selected_text(&s.log_level)
            .show_ui(ui, |ui| {
                for lvl in ["Error", "Warn", "Info", "Debug", "Trace"].iter() {
                    ui.selectable_value(&mut s.log_level, lvl.to_string(), *lvl);
                }
            });
    });
    ui.checkbox(&mut s.use_hardware_acceleration, "Аппаратное ускорение");
}
