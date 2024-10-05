# zawgl-core
Zawgl graph core library

## Usage

Sample usage:
```rust
use log::LevelFilter;
use zawgl_core::{model::{init::InitContext, Node, Property, Relationship}, repository::graph_repository::GraphRepository};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    let ctx: InitContext = InitContext::new("zawgl-db").expect("can't create database context");
    let mut gr = GraphRepository::new(ctx);
    let mut alice = Node::new();
    alice.set_labels(vec!["Person".to_string()]);
    alice.set_properties(vec![Property::new_integer("age", 42)]);
    let alice_with_id = gr.create_node(&alice).unwrap();
    let mut bob = Node::new();
    bob.set_labels(vec!["Person".to_string()]);
    bob.set_properties(vec![Property::new_integer("age", 36)]);
    let bob_with_id = gr.create_node(&bob).unwrap();
    let mut is_friend_rel = Relationship::new();
    is_friend_rel.set_labels(vec!["IS_FRIEND_OF".to_string()]);
    gr.create_relationship(&is_friend_rel, alice_with_id.get_id().unwrap(), bob_with_id.get_id().unwrap());
    gr.sync();

}
```