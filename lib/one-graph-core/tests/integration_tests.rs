extern crate one_graph_core;

use one_graph_core::core::*;
use one_graph_core::test_utils::*;

#[test]
fn create_graph() {
    let db_dir = build_dir_path_and_rm_old("create_graph").unwrap();
    let mut store = GraphStore::new(&db_dir);
    let res = store.process_cypher_query("CREATE (n:Person:Parent)-[r:FRIEND_OF]->(p:Person) RETURN id(n)").unwrap();
    println!("{}", res);
    let mres = store.process_cypher_query("MATCH (n:Person:Parent)-[r:FRIEND_OF]->(p:Person) RETURN n, r, p").unwrap();
    println!("{}", mres);
}