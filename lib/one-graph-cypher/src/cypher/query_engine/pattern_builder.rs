use one_graph_core::{model::PropertyGraph, graph::NodeIndex};
use std::collections::HashMap;

pub fn merge_paths(paths: &Vec<PropertyGraph>) -> Vec<PropertyGraph> {
    
    let mut map_var_paths = HashMap::<String, Vec<(&str, &PropertyGraph, NodeIndex)>>::new();
    for path in paths {
        for nid in path.get_nodes_with_ids() {
            if let Some(var_name) = nid.0.get_var() {
                if map_var_paths.contains_key(var_name) {
                    map_var_paths[var_name].push((var_name, path, nid.1))
                } else {
                    map_var_paths.insert(var_name.to_string(), vec![(var_name, path, nid.1)]);
                }
            }
        }
    }

    let mut belongs_to_existing_pattern = None;
    for n in path.get_nodes() {
        if let Some(var_name) = n.get_var() {
            if map_var_pattern.contains_key(var_name) {
                belongs_to_existing_pattern = map_var_pattern.get(var_name);
                break;
            }
        }
    }

    if let Some(pattern) = belongs_to_existing_pattern {
        for n in path.get_nodes() {
            if let Some(var_name) = n.get_var() {
                if !map_var_pattern.contains_key(var_name) {
                    let id = pattern.0.add_node(n.clone());
                    map_var_pattern.insert(var_name.to_string(), (pattern., id));
                }
            }
        }
        for rne in path.get_edges() {
            let s = path.get_node_ref(&rne.get_source());
            let t = path.get_node_ref(&rne.get_target());
            let r = &rne.relationship;
            if let Some(s_var) = s.get_var() {
                if let Some(t_var) = t.get_var() {
                    pattern.add_relationship(r.clone(), map_var_pattern[s_var].1, map_var_pattern[t_var].1);
                }
            }
        }
    }


}