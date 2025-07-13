use cpal::traits::{DeviceTrait, HostTrait};

#[derive(PartialEq)]
pub enum SettingsTab {
    General,
    Audio,
    // Проблемы с библеотекой камеры. Пофиксить
    // Video,
    Advanced,
}

pub struct Settings {
    pub tab: SettingsTab,
    pub general: GeneralSettings,
    pub audio: AudioSettings,
    // Проблемы с библеотекой камеры. Пофиксить
    // pub video: VideoSettings,
    pub advanced: AdvancedSettings,
}

impl Settings {
    // Список доступных аудио‑входов
    pub fn list_input_devices() -> Vec<String> {
        let host = cpal::default_host();
        let devices: Box<dyn Iterator<Item = cpal::Device>> = match host.input_devices() {
            Ok(devices) => Box::new(devices),
            Err(err) => {
                eprintln!("Error listing input devices: {}", err);
                Box::new(std::iter::empty())
            }
        };
        devices.filter_map(|device| device.name().ok()).collect()
    }
    // Список доступных аудио‑выходов
    pub fn list_output_devices() -> Vec<String> {
        let host = cpal::default_host();
        let devices_iter = match host.output_devices() {
            Ok(devices) => devices,
            Err(err) => {
                eprintln!("Error listing output devices: {}", err);
                return Vec::new();
            }
        };
        devices_iter.filter_map(|device| device.name().ok()).collect()
    }
    // Проблемы с библеотекой камеры. Пофиксить
    // Список доступных видеокамер
    // pub fn list_camera_devices() -> Vec<String> {
    //     gstreamer::init().unwrap();
    //     let mut monitor = DeviceMonitor::new();
    //     monitor.add_filter(Some("Video/Source"), None);
    //     monitor.start().unwrap();

    //     let devices: Vec<String> = monitor
    //         .devices()
    //         .iter()
    //         .filter_map(|d: &Device| d.display_name().map(|n| n.to_string()))
    //         .collect();

    //     monitor.stop();
    //     devices
    // }
}


impl Settings {
    // Запускаеться при первом запуске приложения
    pub fn new() -> Self {
        Self {
            tab: SettingsTab::General,
            general: GeneralSettings {
                theme_dark: false,
                language: "Русский".to_string(),
            },
            audio: AudioSettings {
                input_devices: Self::list_input_devices(),
                output_devices: Self::list_output_devices(),
                selected_input: 0,
                selected_output: 0,
                mic_volume: 1.0,
                speaker_volume: 1.0,
                noise_suppression: false,
                echo_cancellation: false,
            },
            // Проблемы с библеотекой. Пофиксить
            // video: VideoSettings {
            //     camera_devices: Self::list_camera_devices(),
            //     selected_camera: 0,
            //     resolution: (1280, 720),
            //     framerate: 30,
            //     brightness: 1.0,
            //     contrast: 1.0,
            //     background_blur: false,
            // },
            advanced: AdvancedSettings {
                enable_logging: false,
                log_level: "Info".to_string(),
                use_hardware_acceleration: true,
            },
        }
    }
}


pub struct GeneralSettings {
    pub theme_dark: bool,
    pub language: String,
}

pub struct AudioSettings {
    pub input_devices: Vec<String>,
    pub output_devices: Vec<String>,
    pub selected_input: usize,
    pub selected_output: usize,
    pub mic_volume: f32,
    pub speaker_volume: f32,
    pub noise_suppression: bool,
    pub echo_cancellation: bool,
}
// Проблемы с библеотекой камеры. Пофиксить
// pub struct VideoSettings {
//     pub camera_devices: Vec<String>,
//     pub selected_camera: usize,
//     pub resolution: (u32, u32),
//     pub framerate: u32,
//     pub brightness: f32,
//     pub contrast: f32,
//     pub background_blur: bool,
// }

pub struct AdvancedSettings {
    pub enable_logging: bool,
    pub log_level: String,
    pub use_hardware_acceleration: bool,
}