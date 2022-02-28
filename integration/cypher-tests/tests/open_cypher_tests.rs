use log::LevelFilter;
use one_graph_core::{model::init::InitContext, test_utils::build_dir_path_and_rm_old};
use simple_logger::SimpleLogger;

use one_graph_client::Client;

#[tokio::test]
async fn test_cypher() {
    let db_dir = build_dir_path_and_rm_old("test_cypher").expect("error");
    SimpleLogger::new().with_level(LevelFilter::Trace).init().unwrap();
    let ctx = InitContext::new(&db_dir).expect("can't create database context");
    let server = one_graph_server::run_server("localhost:8182", ctx);
    
    let client = Client::new("ws://localhost:8182");

    tokio::select!{
        _ = server => 0,
        _ = client.execute_cypher_request("match (n:Person) return n") => 0,
    };
}