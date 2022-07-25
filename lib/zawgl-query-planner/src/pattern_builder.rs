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
                let merged_node = merge_nodes(&s_var_name, target_node, nid.0);
                let id = result.add_node(merged_node);
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

    if n0.get_status() == &Status::Match || n1.get_status() == &Status::Match {
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