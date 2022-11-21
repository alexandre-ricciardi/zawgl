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

use zawgl_core::{model::{PropertyGraph, Node, Status}, graph::*};
use std::{collections::{HashMap, HashSet}};

pub fn build_pattern(source_pattern: &PropertyGraph, target_pattern: &PropertyGraph) -> PropertyGraph {
    let mut result = PropertyGraph::new();
    let mut var_name_to_target_node_id: HashMap<String, NodeIndex> = HashMap::new();
    for nid in target_pattern.get_nodes_with_ids() {
        if let Some(var_name) = nid.0.get_var() {
            var_name_to_target_node_id.insert(var_name.to_string(), nid.1);
        }
    }
    let mut source_nid_to_result_nid = HashMap::new();
    let mut matching_var_names_to_result_nid = HashMap::new();
    for nid in source_pattern.get_nodes_with_ids() {
        if let Some(s_var_name) = nid.0.get_var() {
            if var_name_to_target_node_id.contains_key(s_var_name) {
                let target_node = target_pattern.get_node_ref(&var_name_to_target_node_id[s_var_name]);
                let merged_node = merge_nodes(s_var_name, target_node, nid.0);
                let id = result.add_node(merged_node);
                source_nid_to_result_nid.insert(nid.1, id);
                matching_var_names_to_result_nid.insert(s_var_name.to_string(), id);
            } else {
                let mut source_node = nid.0.clone();
                if source_node.get_status() == &Status::Empty {
                    source_node.set_status(Status::Match);
                }
                let id = result.add_node(source_node);
                source_nid_to_result_nid.insert(nid.1, id);
            }
        } else {
            let mut source_node = nid.0.clone();
            if source_node.get_status() == &Status::Empty {
                source_node.set_status(Status::Match);
            }
            let id = result.add_node(source_node);
            source_nid_to_result_nid.insert(nid.1, id);
        }
    }

    for e in source_pattern.get_edges() {
        let source = source_nid_to_result_nid[&e.source];
        let target = source_nid_to_result_nid[&e.target];
        let mut source_rel = e.relationship.clone();
        if source_rel.get_status() == &Status::Empty {
            source_rel.set_status(Status::Match);
        }
        
        result.add_relationship(source_rel, source, target);
    }

    let mut target_nid_to_result_nid = HashMap::new();
    for nid in target_pattern.get_nodes_with_ids() {
        if let Some(t_var_name) = nid.0.get_var() {
            if matching_var_names_to_result_nid.contains_key(t_var_name) {
                target_nid_to_result_nid.insert(nid.1, matching_var_names_to_result_nid[t_var_name]);
            } else {
                let id = result.add_node(nid.0.clone());
                target_nid_to_result_nid.insert(nid.1, id);
            }
        } else {
            let id = result.add_node(nid.0.clone());
            target_nid_to_result_nid.insert(nid.1, id);
        }
    }

    for e in target_pattern.get_edges() {
        let source = target_nid_to_result_nid[&e.source];
        let target = target_nid_to_result_nid[&e.target];
        result.add_relationship(e.relationship.clone(), source, target);
    }
    result
}

fn merge_nodes(var_name: &str, n0: &Node, n1: &Node) -> Node {
    let mut labels_set = HashSet::new();
    
    for l0 in n0.get_labels_ref() {
        labels_set.insert(l0.to_string());
    }
    for l1 in n1.get_labels_ref() {
        labels_set.insert(l1.to_string());
    }

    let mut res = Node::new();
    res.set_var(var_name);

    if n0.get_status() == &Status::Match || n1.get_status() == &Status::Match || n0.get_status() == &Status::Empty || n1.get_status() == &Status::Empty {
        res.set_status(Status::Match);
    } else {
        res.set_status(Status::Create);
    }

    for l in labels_set {
        res.get_labels_mut().push(l);
    }

    if let Some(nid) = n0.get_id() {
        res.set_id(Some(nid));
    }

    if let Some(nid) = n1.get_id() {
        res.set_id(Some(nid));
    }

    let mut props_set = HashSet::new();

    for p in n0.get_properties_ref() {
        props_set.insert(p);
    }
    for p in n1.get_properties_ref() {
        props_set.insert(p);
    }

    for p in props_set {
        res.get_properties_mut().push(p.clone());
    }

    res
}

pub fn merge_patterns(patterns: &Vec<&PropertyGraph>) -> PropertyGraph {
    let mut result = PropertyGraph::new();
    let mut source_pattern_to_result_nid = HashMap::new();
    let mut map_var_name_to_pattern_nid = HashMap::new();
    let mut pattern_id = 0;
    for p in patterns {
        let mut source_pattern_nid_to_result_nid = HashMap::new();
        for nid in p.get_nodes_with_ids() {
            let n = nid.0;
            if let Some(var_name) = n.get_var() {
                if !map_var_name_to_pattern_nid.contains_key(var_name) {
                    map_var_name_to_pattern_nid.insert(var_name, (pattern_id, nid.1));
                } else {
                    continue;
                }
            }
            let id = result.add_node(n.clone());
            source_pattern_nid_to_result_nid.insert(nid.1, id);
        }
        source_pattern_to_result_nid.insert(pattern_id, source_pattern_nid_to_result_nid);
        pattern_id += 1;
    }
    pattern_id = 0;
    for p in patterns {
        for e in p.get_edges() {
            let mut source_id = e.source;
            let mut target_id = e.target;
            let mut tmp_pattern_id = pattern_id;
            if !source_pattern_to_result_nid[&pattern_id].contains_key(&e.source) {
                if let Some(var_name) = patterns[pattern_id].get_node_ref(&e.source).get_var() {
                    tmp_pattern_id = map_var_name_to_pattern_nid[var_name].0;
                    source_id = map_var_name_to_pattern_nid[var_name].1;
                }
            }
            if !source_pattern_to_result_nid[&pattern_id].contains_key(&e.target) {
                if let Some(var_name) = patterns[pattern_id].get_node_ref(&e.target).get_var() {
                    tmp_pattern_id = map_var_name_to_pattern_nid[var_name].0;
                    target_id = map_var_name_to_pattern_nid[var_name].1;
                }
            }
            let source = source_pattern_to_result_nid[&tmp_pattern_id][&source_id];
            let target = source_pattern_to_result_nid[&tmp_pattern_id][&target_id];
            result.add_relationship(e.relationship.clone(), source, target);
        }
        pattern_id += 1;
    }
    result
}