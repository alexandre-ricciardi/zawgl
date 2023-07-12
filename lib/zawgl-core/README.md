# zawgl-core
Zawgl graph core library

## Usage

Sample usage:
```rust
use zawgl_core::{model::{init::InitContext, Node, Property, PropertyValue, Relationship}, repository::graph_repository::GraphRepository};

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    let ctx: InitContext = InitContext::new("zawgl-db").expect("can't create database context");
    let mut gr = GraphRepository::new(ctx);
    let mut node = Node::new();
    node.set_labels(vec!["Person".to_string()]);
    node.set_properties(vec![Property::new("age".to_string(), PropertyValue::PInteger(42))]);
    let node_with_id = gr.create_node(&node).unwrap();
    gr.sync();
 }
```