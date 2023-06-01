// MIT License
//
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::path::Path;
use std::env;

use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};
use log::*;

const CONFIG_DIR: &str = ".zawgl";
const CONFIG_FILE_NAME: &str = "Settings.toml";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    pub address: String,
    pub database_dir: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub log: Log,
}

impl Settings {
    pub fn new() -> Self {
        let s = Config::builder();
        let mut current_dir = env::current_dir().map_err(|err| ConfigError::Message(err.to_string())).expect("current directory");
        let conf_dir = Path::new(CONFIG_DIR);
        current_dir.push(conf_dir);
        let mut full_path = current_dir.clone();
        let config_file = Path::new(CONFIG_FILE_NAME);
        full_path.push(config_file);
        if let Some(file_path) = full_path.as_path().to_str() {
            if let Ok(config) = s.add_source(config::File::with_name(file_path))
                .build() {
                if let Ok(settings) = config.try_deserialize() {
                    settings
                } else {
                    panic!("Failed parsing config file .zawgl/Settings.toml")
                }
            } else {
                let server = Server { address: "0.0.0.0:8182".to_string(), database_dir: "zawgl-db".to_string() };
                let log = Log { level: "info".to_string() };
                let settings = Settings { server, log };
                std::fs::create_dir_all(&current_dir).expect("Failed creating configuration dir");
                std::fs::write(
                    &full_path,
                    toml::to_string(&settings).unwrap()
                )
                .expect("Failed to create configuration file");
                println!("Created configuration file {}", file_path);
                settings
            }
        } else {
            panic!("Failed to resolve current directory")
        }
        
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
