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