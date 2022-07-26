use std::collections::HashSet;

use zawgl_core::graph::traits::GrowableGraphTrait;
use zawgl_core::graph_engine::model::{GraphProxy, ProxyNodeId};
use zawgl_core::test_utils::*;
use zawgl_core::model::init::InitContext;
use zawgl_core::graph_engine::GraphEngine;
use zawgl_core::graph::traits::*;

fn main() {
    let main_dir = get_tmp_dir_path("test_mutliple_match");
    let conf = InitContext::new(&main_dir).expect("can't create context");
    let mut graph_engine = GraphEngine::new(&conf);
    let mut full_graph = graph_engine.retrieve_graph().unwrap();
    println!("{:?}", full_graph.get_nodes_ids());
    depth_first_search(&mut full_graph);
    println!("full_graph {{");
    for e in full_graph.get_edges_with_relationships() {
        println!("{:?} --{:?}--> {:?}", e.0.source.store_id, e.1.get_id().unwrap(), e.0.target.store_id);
    }
    println!("}}");
}

fn depth_first_search(graph: &mut GraphProxy) {
    let mut labeled = HashSet::new();
    for id in graph.get_nodes_ids() {
        if !labeled.contains(&id) {
            iterate_adjacent_nodes(&mut labeled, graph, &id);
        }
    }
}

fn iterate_adjacent_nodes(labeled: &mut HashSet<ProxyNodeId>, graph: &mut GraphProxy, id: &ProxyNodeId) {
    labeled.insert(*id);
    println!("{:?}", graph.get_node_ref(&id));
    for e_in in graph.in_edges(&id) {
        println!("{:?}", graph.get_relationship_ref(&e_in));
        let in_v = graph.get_source_index(&e_in);
        if !labeled.contains(&in_v) {
            iterate_adjacent_nodes(labeled, graph, &in_v);
        }
    }
    for e_out in graph.out_edges(&id) {
        println!("{:?}", graph.get_relationship_ref(&e_out));
        let out_v = graph.get_target_index(&e_out);
        if !labeled.contains(&out_v) {
            iterate_adjacent_nodes(labeled, graph, &out_v);
        }
    }
}