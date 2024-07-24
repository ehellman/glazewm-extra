use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowRule {
    pub command: String,
    pub match_process_name: Option<String>,
    pub match_class_name: Option<String>,
    pub match_title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    window_rules: Option<Vec<WindowRule>>,
    focused_window_rules: Option<Vec<WindowRule>>,
    unfocused_window_rules: Option<Vec<WindowRule>>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            window_rules: Some(vec![
                WindowRule {
                    command: "set title false".to_string(),
                    match_process_name: Some(".*".to_string()),
                    match_class_name: None,
                    match_title: None,
                },
                WindowRule {
                    command: "set rounded false".to_string(),
                    match_process_name: Some(".*".to_string()),
                    match_class_name: None,
                    match_title: None,
                },
            ]),
            focused_window_rules: Some(vec![WindowRule {
                command: "set translucent 255".to_string(),
                match_process_name: Some(".*".to_string()),
                match_class_name: None,
                match_title: None,
            }]),
            unfocused_window_rules: Some(vec![WindowRule {
                command: "set translucent 220".to_string(),
                match_process_name: Some(".*".to_string()),
                match_class_name: None,
                match_title: None,
            }]),
        }
    }
}

pub fn parse_config() -> AppConfig {
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push(".config");
    config_path.push("glazewm-extra.toml");

    match fs::metadata(&config_path) {
        Ok(_) => {
            let config_str = fs::read_to_string(&config_path).unwrap();

            if let Ok(app_config) = toml::from_str::<AppConfig>(&config_str) {
                app_config
            } else {
                AppConfig::default()
            }
        }
        Err(_) => {
            let config_str = toml::to_string(&AppConfig::default()).unwrap();
            let _ = fs::write(&config_path, config_str);
            AppConfig::default()
        }
    }
}
