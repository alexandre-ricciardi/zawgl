extern crate one_graph_server;
extern crate tokio;
use one_graph_core::model::init::InitContext;
use one_graph_core::test_utils::*;

#[tokio::main]
async fn main() {
    let main_dir = build_dir_path_and_rm_old("test_main").unwrap();
    let conf = InitContext::new(&main_dir);
    one_graph_server::run_server("127.0.0.1:8182", conf).await;
}