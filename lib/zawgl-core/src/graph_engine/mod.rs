// MIT License
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

pub mod model;

use std::collections::HashMap;

use super::model::*;
use super::repository::graph_repository::GraphRepository;
use self::model::*;
use super::matcher::vf2::sub_graph_isomorphism;
use super::graph::traits::*;

pub type MutableGraphRepository = GraphRepository;

pub struct GraphEngine {
    repository: MutableGraphRepository,
}

fn compare_relationships(r0: &Relationship, r1: &Relationship) -> bool {
    let mut res = true;
    for p0 in r0.get_properties_ref() {
        if !r1.get_properties_ref().contains(p0) {
            res = false;
            break;
        }
    }
    res
}

impl GraphEngine {
    pub fn new(ctx: init::InitContext) -> Self {
        GraphEngine{repository: GraphRepository::new(ctx)}
    }

    pub fn create_graph(&mut self, graph: &PropertyGraph) -> Option<PropertyGraph> {
        self.repository.create_graph(graph)
    }

    pub fn create_node(&mut self, node: &Node) -> Option<Node> {
        self.repository.create_node(node)
    }
    
    pub fn create_relationship(&mut self, rel: &Relationship, source_id: u64, target_id: u64) -> Option<Relationship> {
        self.repository.create_relationship(rel, source_id, target_id)
    }

    pub fn match_pattern(&mut self, pattern: &PropertyGraph) -> Option<Vec<PropertyGraph>> {
        let mut graph_proxy = GraphProxy::new(&mut self.repository, pattern)?;
        let mut res = Vec::new();
        sub_graph_isomorphism(pattern, &mut graph_proxy, 
        |n0, n1| {
            if n0.get_id().is_none() && n0.get_labels_ref().is_empty() {
                return true;
            }
            
            if n0.get_id().is_some() && n0.get_id() != n1.get_id() {
                return false;
            }

            let mut match_labels = true;
            for label in n0.get_labels_ref() {
                if !n1.get_labels_ref().contains(label) {
                    match_labels = false;
                    break;
                }
            }
            let mut match_properties = true;
            for pred in n0.get_predicates_ref() {
                if match_properties {
                    for p1 in n1.get_properties_ref() {
                        if p1.get_name() == pred.name {
                            match_properties = pred.predicate.eval(p1.get_value());
                            if !match_properties {
                                break;
                            }
                        }
                    }
                }
            }
            match_labels && match_properties
        },
        |e0, e1| {
            if e0.get_id().is_none() && e0.get_labels_ref().is_empty() {
                return true;
            }
            
            if e0.get_id().is_some() && e0.get_id() != e1.get_id() {
                return false;
            }

            let mut match_labels = true;
            for label in e0.get_labels_ref() {
                if !e1.get_labels_ref().contains(label) {
                    match_labels = false;
                    break;
                }
            }
            let mut match_properties = true;
            for pred in e0.get_predicates_ref() {
                if match_properties {
                    for p1 in e1.get_properties_ref() {
                        if p1.get_name() == pred.name {
                            match_properties = pred.predicate.eval(p1.get_value());
                            if !match_properties {
                                break;
                            }
                        }
                    }
                }
            }
            match_labels && match_properties
        },
        |map0, _map1, gpattern, proxy: &mut GraphProxy| {
            let mut res_match = PropertyGraph::new();
            for index in gpattern.get_nodes_ids() {
                let pattern_node = gpattern.get_node_ref(&index);
                let proxy_index = map0[&index];
                let mut proxy_node = proxy.get_node_ref(&proxy_index)?.clone();
                proxy_node.set_option_var(pattern_node.get_var());
                res_match.add_node(proxy_node);
            }
            for prel in gpattern.get_relationships_and_edges() {
                let psource_id = &prel.source;
                let ptarget_id = &prel.target;
                let proxy_source_id = map0[psource_id];
                let proxy_target_id = map0[ptarget_id];
                for (_rel_id, target_id, rel) in proxy.out_edges(&proxy_source_id)? {
                    if target_id == proxy_target_id && compare_relationships(&prel.relationship, &rel) {
                        let mut rel_clone = rel.clone();
                        rel_clone.set_option_var(prel.relationship.get_var());
                        res_match.add_relationship(rel_clone, *psource_id, *ptarget_id);
                    }
                }
            }
            res.push(res_match);
            Some(true)
        });
        Some(res)
    }

    pub fn match_patterns_and_create(&mut self, patterns: &Vec<PropertyGraph>) -> Option<Vec<Vec<PropertyGraph>>> {
        let mut matched_patterns = Vec::new();

        for pattern in patterns {
            let mut map_nodes_ids = HashMap::new();
            let mut nodes_db_ids_to_pattern_id = HashMap::new();
            let mut match_pattern = PropertyGraph::new();
            for nid in pattern.get_nodes_ids() {
                let n = pattern.get_node_ref(&nid);
                if let Some(db_id) = n.get_id() {
                    if let std::collections::hash_map::Entry::Vacant(e) = nodes_db_ids_to_pattern_id.entry(db_id) {
                        e.insert(nid);
                    } else {
                        let prev_id = nodes_db_ids_to_pattern_id[&db_id];
                        map_nodes_ids.insert(nid, prev_id);
                        continue;
                    }
                }
                if *n.get_status() == Status::Match {
                    let pattern_n_index = match_pattern.add_node(n.clone());
                    map_nodes_ids.insert(nid, pattern_n_index);
                }
            }
            for re in pattern.get_relationships_and_edges() {
                let source_index = re.source;
                let target_index = re.target;
                if *re.relationship.get_status() == Status::Match {
                    let mut sid = source_index;
                    let mut tid = target_index;
                    if !map_nodes_ids.contains_key(&source_index) {
                        let source = pattern.get_node_ref(&source_index);
                        if let Some(source_db_id) = source.get_id() {
                            sid = nodes_db_ids_to_pattern_id[&source_db_id]; 
                        }                     
                    }
                    if !map_nodes_ids.contains_key(&target_index) {
                        let target = pattern.get_node_ref(&target_index);
                        if let Some(target_db_id) = target.get_id() {
                            tid = nodes_db_ids_to_pattern_id[&target_db_id]; 
                        }                     
                    }
                    match_pattern.add_relationship(re.relationship.clone(), sid, tid);
                }
            }

            let res = self.match_pattern(&match_pattern)?;

            matched_patterns.push((map_nodes_ids, res, pattern));
        }

        let mut results = Vec::new();
        for mut matched in matched_patterns {
            let mut matched_graphs = Vec::new();
            for mut matched_graph in matched.1 {
                for nid in matched.2.get_nodes_with_ids() {
                    if *nid.0.get_status() == Status::Create {
                        let node = self.create_node(nid.0)?;
                        let node_id = matched_graph.add_node(node);
                        matched.0.insert(node_id, nid.1);
                    }
                }
                for re in  matched.2.get_relationships_and_edges() {
                    if *re.relationship.get_status() == Status::Create {
                        let source_index = matched.0[&re.source];
                        let target_index = matched.0[&re.target];
                        let source = matched_graph.get_node_ref(&source_index).get_id()?;
                        let target = matched_graph.get_node_ref(&target_index).get_id()?;
                        let res = self.create_relationship(&re.relationship, source, target)?;
                        matched_graph.add_relationship(res, source_index, target_index);
                    }
                }
                matched_graphs.push(matched_graph);
            }
            results.push(matched_graphs);
        }
        
        Some(results)
    }


    pub fn retrieve_graph(&mut self) -> Option<GraphProxy> {
        GraphProxy::new_full(&mut self.repository)
    }

    pub fn sync(&mut self) {
        self.repository.sync();
    }

    pub fn clear(&mut self) {
        self.repository.clear();
    }
}



#[cfg(test)]
mod test_graph_engine_match {
    use crate::{model::{PropertyGraph, Node, Relationship, init::InitContext}, test_utils::build_dir_path_and_rm_old};
    use std::time::Instant;

    use super::GraphEngine;

    #[test]
    fn test_match() {
        let main_dir = build_dir_path_and_rm_old("test_match_graph_engine").expect("db path");
        {
            let mut graph = PropertyGraph::new();
            let mut n1 = Node::new();
            n1.set_labels(vec!["Label1".to_string()]);
            let id1 = graph.add_node(n1);
            let mut n2 = Node::new();
            n2.set_labels(vec!["Label2".to_string()]);
            let id2 = graph.add_node(n2);
            let mut n3 = Node::new();
            n3.set_labels(vec!["Label3".to_string()]);
            let id3 = graph.add_node(n3);
            let mut r12 = Relationship::new();
            r12.set_labels(vec!["Type12".to_string()]);
            graph.add_relationship(r12, id1, id2);
            let mut r32 = Relationship::new();
            r32.set_labels(vec!["Type32".to_string()]);
            graph.add_relationship(r32, id3, id2);
            let conf = InitContext::new(&main_dir).expect("can't create context");
            let mut ge = GraphEngine::new(conf);
            ge.create_graph(&graph);
            ge.sync();

        }

        let conf = InitContext::new(&main_dir).expect("can't create context");
        let mut ge_load = GraphEngine::new(conf);

        let mut pattern = PropertyGraph::new();
        let mut n2 = Node::new();
        n2.set_labels(vec!["Label2".to_string()]);
        let id2 = pattern.add_node(n2);
        let mut n3 = Node::new();
        n3.set_labels(vec!["Label3".to_string()]);
        let id3 = pattern.add_node(n3);
        let mut r32 = Relationship::new();
        r32.set_labels(vec!["Type32".to_string()]);
        pattern.add_relationship(r32, id3, id2);

        let res = ge_load.match_pattern(&pattern).expect("graphs");

        assert_eq!(1, res.len())
    }

    
    #[test]
    fn test_match_self_relationship() {
        let main_dir = build_dir_path_and_rm_old("test_match_graph_engine_self").expect("db path");
        {
            let mut graph = PropertyGraph::new();
            let mut n1 = Node::new();
            n1.set_labels(vec!["Label1".to_string()]);
            let id1 = graph.add_node(n1);
            let mut n2 = Node::new();
            n2.set_labels(vec!["Label2".to_string()]);
            let id2 = graph.add_node(n2);
            let mut n3 = Node::new();
            n3.set_labels(vec!["Label3".to_string()]);
            let id3 = graph.add_node(n3);
            let mut r12 = Relationship::new();
            r12.set_labels(vec!["Type12".to_string()]);
            graph.add_relationship(r12, id1, id2);
            let mut r32 = Relationship::new();
            r32.set_labels(vec!["Type32".to_string()]);
            graph.add_relationship(r32, id3, id2);
            let mut r33 = Relationship::new();
            r33.set_labels(vec!["Type33".to_string()]);
            graph.add_relationship(r33, id3, id3);
            let conf = InitContext::new(&main_dir).expect("can't create context");
            let mut ge = GraphEngine::new(conf);
            ge.create_graph(&graph);
            ge.sync();

        }

        let conf = InitContext::new(&main_dir).expect("can't create context");
        let mut ge_load = GraphEngine::new(conf);

        let mut pattern = PropertyGraph::new();
        let mut n3 = Node::new();
        n3.set_labels(vec!["Label3".to_string()]);
        let id3 = pattern.add_node(n3);
        let mut r32 = Relationship::new();
        r32.set_labels(vec!["Type33".to_string()]);
        pattern.add_relationship(r32, id3, id3);

        let res = ge_load.match_pattern(&pattern).expect("graphs");

        assert_eq!(1, res.len())
    }

    
    #[test]
    fn test_match_merge_same_node_self_relationship() {
        let main_dir = build_dir_path_and_rm_old("test_match_merge_same_node_self_relationship").expect("db path");
        {
            let mut graph = PropertyGraph::new();
            let mut n1 = Node::new();
            n1.set_labels(vec!["Label1".to_string()]);
            let id1 = graph.add_node(n1);
            let mut n2 = Node::new();
            n2.set_labels(vec!["Label2".to_string()]);
            let id2 = graph.add_node(n2);
            let mut n3 = Node::new();
            n3.set_labels(vec!["Label3".to_string()]);
            let id3 = graph.add_node(n3);
            let mut r12 = Relationship::new();
            r12.set_labels(vec!["Type12".to_string()]);
            graph.add_relationship(r12, id1, id2);
            let mut r32 = Relationship::new();
            r32.set_labels(vec!["Type32".to_string()]);
            graph.add_relationship(r32, id3, id2);
            let mut r33 = Relationship::new();
            r33.set_labels(vec!["Type33".to_string()]);
            graph.add_relationship(r33, id3, id3);
            let conf = InitContext::new(&main_dir).expect("can't create context");
            let mut ge = GraphEngine::new(conf);
            ge.create_graph(&graph);
            ge.sync();

        }

        let conf = InitContext::new(&main_dir).expect("can't create context");
        let mut ge_load = GraphEngine::new(conf);

        let mut pattern = PropertyGraph::new();
        let mut n3 = Node::new();
        n3.set_labels(vec!["Label3".to_string()]);
        let id3 = pattern.add_node(n3);
        let mut r32 = Relationship::new();
        r32.set_labels(vec!["Type33".to_string()]);
        pattern.add_relationship(r32, id3, id3);

        let res = ge_load.match_pattern(&pattern).expect("graphs");

        assert_eq!(1, res.len())
    }

    #[test]
    fn test_bench_create_nodes() {
        let main_dir = build_dir_path_and_rm_old("test_bench_create_nodes").expect("db path");
        let conf = InitContext::new(&main_dir).expect("can't create context");
        let mut ge = GraphEngine::new(conf);
        let mut n = Node::new();
        n.set_labels(vec!["Label1".to_string()]);
        let start = Instant::now();
        for _ in 0..1000 {
            ge.create_node(&n).expect("node created");
        }
        ge.sync();
    
        let duration = start.elapsed();
        println!("Time to create 1000 nodes: {:?}", duration)
    }
}