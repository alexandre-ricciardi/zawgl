use std::path::Path;
use std::env;

use config::{Config, ConfigError, File};
use serde::Deserialize;

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
        let mut s = Config::new();
        let mut current_dir = env::current_dir().map_err(|err| ConfigError::Message(err.to_string()))?;
        let config_file_path = Path::new(CONFIG_FILE_PATH);
        current_dir.push(config_file_path);
        s.merge(File::from(current_dir.as_path()))?;
        s.try_into()
    }
}