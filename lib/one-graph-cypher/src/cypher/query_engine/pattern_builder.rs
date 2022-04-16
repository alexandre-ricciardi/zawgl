use one_graph_core::model::PropertyGraph;
use one_graph_core::graph::traits::GraphContainerTrait;
use std::collections::HashMap;

pub fn merge_path(pattern: &mut PropertyGraph, path: &PropertyGraph) {
    let mut map_var_pattern = HashMap::new();
    for nid in pattern.get_nodes_with_ids() {
        if let Some(var_name) = nid.0.get_var() {
            map_var_pattern.insert(var_name.to_string(), nid.1);
        }
    }
    for n in path.get_nodes() {
        if let Some(var_name) = n.get_var() {
            if !map_var_pattern.contains_key(var_name) {
                let id = pattern.add_node(n.clone());
                map_var_pattern.insert(var_name.to_string(), id);
            }
        }
    }
    for rne in path.get_edges() {
        let s = path.get_node_ref(&rne.get_source());
        let t = path.get_node_ref(&rne.get_target());
        let r = &rne.relationship;
        if let Some(s_var) = s.get_var() {
            if let Some(t_var) = t.get_var() {
                pattern.add_relationship(r.clone(), map_var_pattern[s_var], map_var_pattern[t_var]);
            }
        }
    }

}