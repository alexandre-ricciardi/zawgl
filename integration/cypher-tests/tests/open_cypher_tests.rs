use log::LevelFilter;
use one_graph_core::{model::init::InitContext, test_utils::build_dir_path_and_rm_old};
use simple_logger::SimpleLogger;
use log::*;
use og_client::Client;

#[tokio::test]
async fn test_cypher() {
    let db_dir = build_dir_path_and_rm_old("test_cypher").expect("error");
    SimpleLogger::new().with_level(LevelFilter::Debug).init().unwrap();
    let ctx = InitContext::new(&db_dir).expect("can't create database context");
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let server = one_graph_server::run_server("localhost:8182", ctx, || {
        if let Err(_) = tx.send(()) {
            error!("starting database");
        }
    });
    tokio::spawn(server);
    match rx.await {
        Ok(_) => test_cypher_requests().await,
        Err(_) => error!("starting database"),
    }
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

    let r2 = client.execute_cypher_request("create (n:Movie) return n").await;
    if let Ok(d) = r2 {
        debug!("{}", d.to_string())
    }

    
    let r3 = client.execute_cypher_request("create (n:Movie)<-[r:Played]-(p:Person) return n, r, p").await;
    if let Ok(d) = r3 {
        debug!("{}", d.to_string())
    }

    
    let r4 = client.execute_cypher_request("match (p:Person), (m:Movie) create (m)<-[r:Played]-(p) return m, r, p").await;
    if let Ok(d) = r4 {
        debug!("{}", d.to_string())
    }

}