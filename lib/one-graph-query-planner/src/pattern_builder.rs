use one_graph_core::{model::PropertyGraph, graph::*};
use std::{collections::{HashMap}};

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
                let id = result.add_node(target_pattern.get_node_ref(&var_name_to_target_node_id[s_var_name]).clone());
                source_nid_to_result_nid.insert(nid.1, id);
                matching_var_names_to_result_nid.insert(s_var_name.to_string(), id);
            } else {
                let id = result.add_node(nid.0.clone());
                source_nid_to_result_nid.insert(nid.1, id);
            }
        } else {
            let id = result.add_node(nid.0.clone());
            source_nid_to_result_nid.insert(nid.1, id);
        }
    }

    for e in source_pattern.get_edges() {
        let source = source_nid_to_result_nid[&e.source];
        let target = source_nid_to_result_nid[&e.target];
        result.add_relationship(e.relationship.clone(), source, target);
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

pub fn merge_patterns(patterns: &Vec<&PropertyGraph>) -> PropertyGraph {
    let mut result = PropertyGraph::new();
    let mut source_pattern_to_result_nid = HashMap::new();
    let mut pattern_id = 0;
    for p in patterns {
        let mut source_pattern_nid_to_result_nid = HashMap::new();
        for nid in p.get_nodes_with_ids() {
            let id = result.add_node(nid.0.clone());
            source_pattern_nid_to_result_nid.insert(nid.1, id);
        }
        source_pattern_to_result_nid.insert(pattern_id, source_pattern_nid_to_result_nid);
        pattern_id += 1;
    }
    pattern_id = 0;
    for p in patterns {
        for e in p.get_edges() {
            let source = source_pattern_to_result_nid[&pattern_id][&e.source];
            let target = source_pattern_to_result_nid[&pattern_id][&e.target];
            result.add_relationship(e.relationship.clone(), source, target);
        }
        pattern_id += 1;
    }
    result
}