use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{App, Manager};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub translucent_window: Option<TranslucentWindowConfig>,
    pub title_bar: Option<TitleBarConfig>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            translucent_window: Some(TranslucentWindowConfig::default()),
            title_bar: Some(TitleBarConfig::default()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslucentWindowConfig {
    pub enable: Option<bool>,
    pub alpha: Option<u8>,
}

impl Default for TranslucentWindowConfig {
    fn default() -> Self {
        Self {
            enable: Some(true),
            alpha: Some(220),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleBarConfig {
    pub enable: Option<bool>,
}

impl Default for TitleBarConfig {
    fn default() -> Self {
        Self { enable: Some(true) }
    }
}

pub fn setup_store(app: &mut App) {
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push(".config");
    config_path.push("glazewm-extra.toml");

    match fs::metadata(&config_path) {
        Ok(_) => {
            let config_str = fs::read_to_string(&config_path).unwrap();

            if let Ok(app_config) = toml::from_str::<AppConfig>(&config_str) {
                app.manage(app_config);
            } else {
                app.manage(AppConfig::default());
            }
        }
        Err(_) => {
            let config_str = toml::to_string(&AppConfig::default()).unwrap();
            let _ = fs::write(&config_path, config_str);
            app.manage(AppConfig::default());
        }
    }
}

#[tauri::command]
pub fn get_app_config(state: tauri::State<AppConfig>) -> &AppConfig {
    state.inner()
}