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
                pattern.add_node(n.clone());
            }
        }
    }
    for rne in path.get_relationships_and_edges() {
        let e = rne.1;
        let s = path.get_node_ref(&e.get_source());
        let t = path.get_node_ref(&e.get_target());
        let r = rne.0;
        if let Some(s_var) = s.get_var() {
            let is_ref_s = map_var_pattern.contains_key(s_var);
            if let Some(t_var) = t.get_var() {
                let is_ref_t = map_var_pattern.contains_key(t_var);
                if is_ref_s && is_ref_t {
                    pattern.add_relationship(r.clone(), map_var_pattern[s_var], map_var_pattern[t_var]);
                } else if is_ref_s {
                    let tid = pattern.add_node(t.clone());
                    pattern.add_relationship(r.clone(), map_var_pattern[s_var], tid);
                } else if is_ref_t {
                    let sid = pattern.add_node(s.clone());
                    pattern.add_relationship(r.clone(), sid, map_var_pattern[t_var]);
                } else {
                    let sid = pattern.add_node(s.clone());
                    let tid = pattern.add_node(t.clone());
                    pattern.add_relationship(r.clone(), sid, tid);
                }
            } else if is_ref_s {
                let tid = pattern.add_node(t.clone());
                pattern.add_relationship(r.clone(), map_var_pattern[s_var], tid);
            } else {
                let sid = pattern.add_node(s.clone());
                let tid = pattern.add_node(t.clone());
                pattern.add_relationship(r.clone(), sid, tid);                
            }
        } else if let Some(t_var) = t.get_var() {
            let is_ref_t = map_var_pattern.contains_key(t_var);
            if let Some(s_var) = s.get_var() {
                let is_ref_s = map_var_pattern.contains_key(s_var);
                if is_ref_s && is_ref_t {
                    pattern.add_relationship(r.clone(), map_var_pattern[s_var], map_var_pattern[t_var]);
                } else if is_ref_s {
                    let tid = pattern.add_node(t.clone());
                    pattern.add_relationship(r.clone(), map_var_pattern[s_var], tid);
                } else if is_ref_t {
                    let sid = pattern.add_node(s.clone());
                    pattern.add_relationship(r.clone(), sid, map_var_pattern[t_var]);
                } else {
                    let sid = pattern.add_node(s.clone());
                    let tid = pattern.add_node(t.clone());
                    pattern.add_relationship(r.clone(), sid, tid);
                }
            } else if is_ref_t {
                let sid = pattern.add_node(t.clone());
                pattern.add_relationship(r.clone(), sid, map_var_pattern[t_var]);
            } else {
                let sid = pattern.add_node(s.clone());
                let tid = pattern.add_node(t.clone());
                pattern.add_relationship(r.clone(), sid, tid);                
            }
        }
    }

}