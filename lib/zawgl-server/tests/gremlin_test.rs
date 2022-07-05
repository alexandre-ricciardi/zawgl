extern crate gremlin_client;
use gremlin_client::{GremlinClient, process::traversal::traversal};


fn gremlin() {
    let client = GremlinClient::connect("localhost").unwrap();
    let g = traversal().with_remote(client);
    let res = g.v(()).has_label("person").has(("name","Jon")).next();
    println!("{:?}", res);
}