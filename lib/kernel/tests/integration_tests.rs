extern crate kernel;

use kernel::kernel::*;

#[test]
fn create_graph() {
    let dir = "C:\\Temp\\create_graph";
    std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir);
    let mut kernel = DbKernel::new(dir);
    kernel.process_cypher_query("CREATE (n:Person:Parent)-[r:FRIEND_OF]->(p:Person) RETURN id(n)");
    kernel.process_cypher_query("MATCH (n:Person:Parent)-[r:FRIEND_OF]->(p:Person)");
}