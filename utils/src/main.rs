use std::collections::HashSet;

use one_graph_core::graph::traits::GrowableGraphTrait;
use one_graph_core::graph_engine::model::{GraphProxy, ProxyNodeId};
use one_graph_core::test_utils::*;
use one_graph_core::model::init::InitContext;
use one_graph_core::graph_engine::GraphEngine;
use one_graph_core::graph::traits::*;

fn main() {
    let main_dir = get_tmp_dir_path("test_main");
    let conf = InitContext::new(&main_dir);
    let mut graph_engine = GraphEngine::new(&conf);
    let mut full_graph = graph_engine.retrieve_graph().unwrap();
    println!("{:?}", full_graph.get_nodes_ids());
    depth_first_search(&mut full_graph);
    println!("full_graph {{");
    for e in full_graph.get_edges() {
        println!("{:?} -> {:?}", e.source.store_id, e.target.store_id);
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