// MIT License
//
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::collections::HashSet;

use zawgl_core::graph::traits::GrowableGraphTrait;
use zawgl_core::graph_engine::model::{GraphProxy, ProxyNodeId};
use zawgl_core::model::{Node, Property, PropertyValue, Relationship};
use zawgl_core::model::init::InitContext;
use zawgl_core::graph_engine::GraphEngine;
use zawgl_core::graph::traits::*;
use zawgl_core::test_utils::{build_dir_path_and_rm_old};

fn main() {
    let main_dir = build_dir_path_and_rm_old("zawgl-db").expect("db dir path");
    build_test_graph(&main_dir);
    let conf = InitContext::new(&main_dir).expect("can't create context");
    let mut graph_engine = GraphEngine::new(conf);
    let mut full_graph = graph_engine.retrieve_graph().unwrap();
    println!("{:?}", full_graph.get_nodes_ids());
    depth_first_search(&mut full_graph);
    println!("digraph full_graph {{");
    for nid in &full_graph.get_nodes_ids() {
        let n = full_graph.get_node_ref(nid);
        if let Some(node) = n {
            print!("    {}", node.get_id().expect("nid"));
            print!(" [label=\"");
            for l in node.get_labels_ref() {
                print!("{} ", l);
            }
            for p in node.get_properties_ref() {
                print!("{}__{:?} ", p.get_name(), p.get_value());
            }
            print!("\"]");
            println!("");
        }
    }
    for e in full_graph.get_edges_with_relationships() {
        println!("    {} -> {}", e.0.source.store_id, e.0.target.store_id);
    }
    println!("}}");
    for e in full_graph.get_edges_with_relationships() {
        let src = full_graph.get_node_ref(&e.0.source).expect("source").get_labels_ref().join(":");
        let trg = full_graph.get_node_ref(&e.0.target).expect("target").get_labels_ref().join(":");
        println!("    {:?}[{:?}]--{:?}[{:?}](in:{:?}, out:{:?})-->{:?}[{:?}]", 
        e.0.source.store_id, src, e.1.get_id().unwrap(), e.1.get_labels_ref().join(":"), e.0.next_inbound_edge, e.0.next_outbound_edge,
        e.0.target.store_id, trg);
    }
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
    for (e_in, in_v, rel) in graph.in_edges(&id) {
        println!("{:?}", rel);
        if !labeled.contains(&in_v) {
            //iterate_adjacent_nodes(labeled, graph, &in_v);
        }
    }
    for (e_in, out_v, rel) in graph.out_edges(&id) {
        println!("{:?}", rel);
        if !labeled.contains(&out_v) {
            //iterate_adjacent_nodes(labeled, graph, &out_v);
        }
    }
}

fn build_test_graph(main_dir: &str) {
    let conf = InitContext::new(&main_dir).expect("can't create context");
    let mut graph_engine = GraphEngine::new(conf);
    let mut n0 = Node::new();
    n0.set_labels(vec!["Test".to_string(), "Label".to_string()]);
    n0.set_properties(vec![Property::new("name".to_string(), PropertyValue::PBool(true))]);
    let nid0 = graph_engine.create_node(&n0).expect("node").get_id().expect("nid");
    let mut n1 = Node::new();
    n1.set_labels(vec!["Test".to_string(), "Label".to_string()]);
    n1.set_properties(vec![Property::new("name".to_string(), PropertyValue::PFloat(214.2))]);
    let nid1 = graph_engine.create_node(&n1).expect("node").get_id().expect("nid");
    let mut r = Relationship::new();
    r.set_labels(vec!["Relationship".to_string(), "Label".to_string()]);
    r.set_properties(vec![Property::new("name".to_string(), PropertyValue::PFloat(12.3))]);
    graph_engine.create_relationship(&r, nid0, nid1);
    graph_engine.sync();
}