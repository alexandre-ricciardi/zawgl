use std::{thread, time};

use log::LevelFilter;
use one_graph_core::{model::init::InitContext, test_utils::build_dir_path_and_rm_old};
use simple_logger::SimpleLogger;
use log::*;
use one_graph_client::Client;

#[tokio::test]
async fn test_cypher() {
    let db_dir = build_dir_path_and_rm_old("test_cypher").expect("error");
    SimpleLogger::new().with_level(LevelFilter::Trace).init().unwrap();
    let ctx = InitContext::new(&db_dir).expect("can't create database context");
    let server = one_graph_server::run_server("localhost:8182", ctx);

    thread::sleep(time::Duration::from_millis(1000));

    tokio::select!{
        _ = server => 0,
        _ = test_cypher_requests() => 0,
    };
}

async fn test_cypher_requests() {
    let mut client = Client::new("ws://localhost:8182").await;
    let res0 = client.execute_cypher_request("create (n:Person) return n").await;
    if let Ok(d) = res0 {
        debug!("{}", d.to_string())
    }
   
    let res1 = client.execute_cypher_request("match (n:Person) return n").await;
    if let Ok(d) = res1 {
        debug!("{}", d.to_string())
    }
}