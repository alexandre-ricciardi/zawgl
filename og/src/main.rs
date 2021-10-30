extern crate one_graph_server;
extern crate tokio;
extern crate serde;
mod settings;
use one_graph_core::model::init::InitContext;
use settings::Settings;
use log::*;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("config can't be loaded");
    let log_level = get_log_level(&settings);
    SimpleLogger::new().with_level(log_level).init().unwrap();
    let ctx = InitContext::new(&settings.server.database_dir).expect("can't create database context");
    
    tokio::select! {
        _ = one_graph_server::run_server(&settings.server.address, ctx) => 0,
        _ = tokio::signal::ctrl_c() => 0
    };
}

fn get_log_level(settings: &Settings) -> LevelFilter {
    let log_level = match settings.log.level.as_str() {
        "info" => LevelFilter::Info,
        "error" => LevelFilter::Error,
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "warn" => LevelFilter::Warn,
        _ => LevelFilter::Off,
    };
    log_level
}