use std::collections::HashMap;

use config::{Config,File,FileFormat};

pub struct EmuConfig {
    pub screen_size: u8,
    pub font_path: String
}

impl EmuConfig {
    pub fn new() -> EmuConfig {
        EmuConfig {
            screen_size: 1,
            font_path: "./nesfont.bmp".to_string()
        }
    }

    pub fn from_path(path: &String) -> EmuConfig {
        let mut result = EmuConfig::new();
        let config = match Config::default().merge(File::new(path, FileFormat::Toml)) {
            Ok(conf) => {
                info!("Config loaded successfully");
                conf.to_owned()
            },
            Err(e) => {
                error!("Config could not be loaded from {}", path);
                error!("{}", e);
                Config::default()
            }
        };

        let deserialized = config.try_into::<HashMap<String, String>>().unwrap();
        if deserialized.contains_key("screen_size") {
            result.screen_size = match deserialized.get("screen_size").unwrap().parse::<u8>() {
                Ok(val) => val,
                Err(_) => {
                    error!("Invalid value for screen_size, use an integer. Defaulting to 1.");
                    1
                }
            };
        }

        if deserialized.contains_key("font_path") {
            result.font_path = deserialized.get("font_path").unwrap().to_string();
        }
        
        result
    }
}
