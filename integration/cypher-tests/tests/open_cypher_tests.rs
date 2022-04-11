use log::LevelFilter;
use one_graph_core::{model::init::InitContext, test_utils::build_dir_path_and_rm_old};
use simple_logger::SimpleLogger;
use log::*;
use og_client::Client;
use std::{future::Future, rc::Rc};
use bson::{Bson, Document, doc};

#[tokio::test]
async fn test_cypher() {
    SimpleLogger::new().with_level(LevelFilter::Debug).init().unwrap();
    run_test("first_test", test_cypher_requests).await;
    run_test("create_path_test", test_create_path).await;
    test_double_create_issue().await;
}

async fn run_test<F, T>(db_name: &str, lambda: F) where F : FnOnce() -> T, T : Future<Output = ()> + Send {
    let db_dir = build_dir_path_and_rm_old(db_name).expect("error");
    
    let ctx = InitContext::new(&db_dir).expect("can't create database context");
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let server = one_graph_server::run_server("localhost:8182", ctx, || {
        if let Err(_) = tx.send(()) {
            error!("starting database");
        }
    });

    let error_cb = || async {
        assert!(false, "error server");
    };

    let trigger = || async {
            match rx.await {
                Ok(_) => lambda().await,
                Err(_) => error_cb().await,
            }
        };
    tokio::select! {
        _ = server => 0,
        _ = trigger()  => 0
    };
   
}

async fn test_cypher_requests() {
    let mut client = Client::new("ws://localhost:8182").await;
    let r = client.execute_cypher_request("create (n:Person) return n").await;
    if let Ok(d) = r {
        debug!("{}", d.to_string())
    }
    let r = client.execute_cypher_request("create (n:Movie) return n").await;
    if let Ok(d) = r {
        debug!("{}", d.to_string())
    }

    
    let r = client.execute_cypher_request("create (n:Person) return n").await;
    if let Ok(d) = r {
        debug!("{}", d.to_string())
    }
    let r = client.execute_cypher_request("create (n:Movie) return n").await;
    if let Ok(d) = r {
        debug!("{}", d.to_string())
    }
    
    let r = client.execute_cypher_request("match (n:Person) return n").await;
    if let Ok(d) = r {
        let res = d.get_document("result").expect("result");
        let graphs = res.get_array("graphs").expect("graphs");
        assert_eq!(graphs.len(), 2);
        for g in graphs {
            let graph = g.as_document().expect("a graph");
            let nodes = graph.get_array("nodes").expect("nodes");
            assert_eq!(nodes.len(), 1);
        }
    } else {
        assert!(false, "no response")
    }
    
    let r = client.execute_cypher_request("match (n:Movie) return n").await;
    if let Ok(d) = r {
        let res = d.get_document("result").expect("result");
        let graphs = res.get_array("graphs").expect("graphs");
        assert_eq!(graphs.len(), 2);
        for g in graphs {
            let graph = g.as_document().expect("a graph");
            let nodes = graph.get_array("nodes").expect("nodes");
            assert_eq!(nodes.len(), 1);
        }
    } else {
        assert!(false, "no response")
    }
    
    let r = client.execute_cypher_request("match (p:Person), (m:Movie) create (m)<-[r:Played]-(p) return m, r, p").await;
    if let Ok(d) = r {
        debug!("{}", d.to_string());
        let res = d.get_document("result").expect("result");
        let graphs = res.get_array("graphs").expect("graphs");
        assert_eq!(graphs.len(), 4);
        for g in graphs {
            let graph = g.as_document().expect("a graph");
            let nodes = graph.get_array("nodes").expect("nodes");
            let relationships = graph.get_array("relationships").expect("relationships");
            assert_eq!(nodes.len(), 2);
            assert_eq!(relationships.len(), 1);
        }
    }
}

async fn test_double_create_issue() {
    run_test("another_test", move || async {
        let mut client = Client::new("ws://localhost:8182").await;
        let r3 = client.execute_cypher_request("create (n:Movie)<-[r:Played]-(p:Person) return n, r, p").await;
        if let Ok(d) = r3 {
            debug!("{}", d.to_string());
            let res = d.get_document("result").expect("result");
            let graphs = res.get_array("graphs").expect("graphs");
            for g in graphs {
                let graph = g.as_document().expect("a graph");
                let nodes = graph.get_array("nodes").expect("nodes");
                assert_eq!(nodes.len(), 2);
            }
        } else {
            assert!(false, "no response")
        }
    }).await;
}

async fn test_create_path() {
    let mut client = Client::new("ws://localhost:8182").await;
     let r = client.execute_cypher_request("create (n:Movie)<-[r:Played]-(p:Person) return n, r, p").await;
    if let Ok(d) = r {
        debug!("{}", d.to_string());
        let res = d.get_document("result").expect("result");
        let graphs = res.get_array("graphs").expect("graphs");
        assert_eq!(graphs.len(), 1);
        for g in graphs {
            let graph = g.as_document().expect("a graph");
            let nodes = graph.get_array("nodes").expect("nodes");
            let relationships = graph.get_array("relationships").expect("relationships");
            assert_eq!(nodes.len(), 2);
            assert_eq!(relationships.len(), 1);
        }
    } else {
        assert!(false, "no response")
    }
}