use anyhow::Context;
use clap::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    files: Files,
}

#[derive(Serialize, Deserialize, Clone)]
struct Files {
    file_name: String,
    config_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            files: Files {
                file_name: "watson.json".to_string(),
                config_name: "config.toml".to_string(),
            },
        }
    }
}

impl Config {
    pub(crate) fn load_or_default() -> anyhow::Result<Self, Error> {
        let config = Config::default();
        let config_string = std::fs::read_to_string(config.get_config_name());
        match config_string {
            Ok(config_string) => {
                let config: Config = toml::from_str(&config_string)
                    .context("Failed to parse config.toml")
                    .unwrap();
                Ok(config)
            }
            Err(_) => {
                config.save().context("Failed to save config.toml").unwrap();
                Ok(config)
            }
        }
    }

    fn save(&self) -> anyhow::Result<(), Error> {
        let config_string = toml::to_string(self)
            .context("Failed to serialize config.toml")
            .unwrap();
        std::fs::write(self.get_config_name(), config_string)
            .context("Failed to write config.toml")
            .unwrap();
        Ok(())
    }

    pub fn get_file_name(&self) -> String {
        self.files.file_name.clone()
    }

    pub fn get_config_name(&self) -> String {
        self.files.config_name.clone()
    }
}
