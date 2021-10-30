extern crate one_graph_server;
extern crate tokio;
extern crate serde;
mod settings;
use one_graph_core::model::init::InitContext;
use settings::Settings;

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("config can't be loaded");
    let ctx = InitContext::new(&settings.server.database_dir).expect("can't create database context");
    one_graph_server::run_server(&settings.server.address, ctx).await;
}