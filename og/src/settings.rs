use std::path::Path;
use std::env;

use config::{Config, ConfigError, File};
use serde::Deserialize;
use log::*;

const CONFIG_FILE_PATH: &str = ".config/Settings";

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub address: String,
    pub database_dir: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub log: Log,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder();
        let mut current_dir = env::current_dir().map_err(|err| ConfigError::Message(err.to_string()))?;
        let config_file_path = Path::new(CONFIG_FILE_PATH);
        current_dir.push(config_file_path);
        s.add_source(config::File::with_name(current_dir.as_path().to_str().expect("config file path")))
        .build().expect("settings").try_deserialize()
    }


    pub fn get_log_level(&self) -> LevelFilter {
        let log_level = match self.log.level.as_str() {
            "info" => LevelFilter::Info,
            "error" => LevelFilter::Error,
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "warn" => LevelFilter::Warn,
            _ => LevelFilter::Off,
        };
        log_level
    }
}