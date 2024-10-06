# zawgl-core
Zawgl graph core library

## Usage

Sample usage:
```rust
use log::LevelFilter;
use zawgl_core::{graph_engine::GraphEngine, model::{init::InitContext, Node, Property, PropertyGraph, Relationship}, repository::graph_repository::GraphRepository, test_utils::build_dir_path_and_rm_old};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    let db_dir = build_dir_path_and_rm_old("simple_test").expect("error");
    let ctx: InitContext = InitContext::new(&db_dir).expect("can't create database context");
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


    let mut pattern = PropertyGraph::new();
    let source_index = pattern.add_node(Node::new());
    let target_index = pattern.add_node(Node::new());
    pattern.add_relationship(Relationship::new(), source_index, target_index);

    let mut ge = GraphEngine::from_repository(gr);
    let results = ge.match_pattern(&pattern);

    if let Some(res) = results {
        assert_eq!(res.len(), 1);
    } else {
        assert!(false);
    }
}
```