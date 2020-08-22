extern crate one_graph_core;

use one_graph_core::core::*;

#[test]
fn create_graph() {
    let dir = "tmp/create_graph";
    std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir);
    let mut store = GraphStore::new(dir);
    let res = store.process_cypher_query("CREATE (n:Person:Parent)-[r:FRIEND_OF]->(p:Person) RETURN id(n)").unwrap();
    println!("{}", res);
    let mres = store.process_cypher_query("MATCH (n:Person:Parent)-[r:FRIEND_OF]->(p:Person) RETURN n, r, p").unwrap();
    println!("{}", mres);
}