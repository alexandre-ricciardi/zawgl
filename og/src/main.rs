extern crate one_graph_server;
extern crate tokio;
extern crate serde;
mod settings;
use log::info;
use one_graph_core::model::init::InitContext;
use settings::Settings;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("config can't be loaded");
    let log_level = settings.get_log_level();
    SimpleLogger::new().with_level(log_level).init().unwrap();
    let ctx = InitContext::new(&settings.server.database_dir).expect("can't create database context");
    tokio::select! {
        _ = one_graph_server::run_server(&settings.server.address, ctx, || {
            info!("database started");
        }) => 0,
        _ = tokio::signal::ctrl_c() => 0
    };
}
