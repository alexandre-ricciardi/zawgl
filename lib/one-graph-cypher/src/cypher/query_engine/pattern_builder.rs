use one_graph_core::{model::PropertyGraph};
use std::{collections::{HashMap, HashSet}};


pub fn merge_paths(paths: &Vec<PropertyGraph>) -> Vec<PropertyGraph> {
    
    let mut var_paths: HashMap<String, HashSet<usize>> = HashMap::new();
    let mut path_id: usize = 0;
    for path in paths {
        for n in path.get_nodes() {
            if let Some(var_name) = n.get_var() {
                let set = var_paths.get_mut(var_name);
                if let Some(s) = set {
                    s.insert(path_id);
                } else {
                    var_paths.insert(var_name.to_string(), HashSet::from([path_id]));
                }
            }
        }
        path_id += 1;
    }

    let mut paths_set = Vec::<HashSet<usize>>::new();
    for path_id_set in var_paths.values() {
        if path_id_set.len() > 1 {
            let mut found= false;
            for set in paths_set.iter_mut() {
                if !set.is_disjoint(path_id_set) {
                    found = true;
                    for id in path_id_set {
                        set.insert(*id);
                    }
                }
            }
            if !found {
                paths_set.push(path_id_set.clone());
            }
        }
    }

    let mut full_set = HashSet::new();
    for set in paths_set.iter() {
        for id in set {
            full_set.insert(id);
        }
    }

    let mut all_paths = HashSet::new();
    for id in 0..paths.len() {
        all_paths.insert(id);
    }

    for id in 0..paths.len() {
        if !all_paths.contains(&id) {
            let mut set = HashSet::new();
            set.insert(id);
            paths_set.push(set);
        }
    }

    let mut res = Vec::new();
    for set in paths_set {
        let mut pattern = PropertyGraph::new();
        let mut merge_vars_map = HashMap::new();
        let mut map_path_node_ids = HashMap::new();
        for path_id in set {
            let path = &paths[path_id];
            for node in path.get_nodes_with_ids() {
                if let Some(var_name) = node.0.get_var() {
                    if !merge_vars_map.contains_key(var_name) {
                        let nid = pattern.add_node(node.0.clone());
                        map_path_node_ids.insert(node.1, nid);
                        merge_vars_map.insert(var_name, nid);
                    }
                } else {
                    let nid = pattern.add_node(node.0.clone());
                    map_path_node_ids.insert(node.1, nid);
                }
            }

            for e in path.get_edges() {
                let s = e.get_source();
                let t = e.get_target();
                let r = &e.relationship;
                pattern.add_relationship(r.clone(), map_path_node_ids[&s], map_path_node_ids[&t]);
            }
        }
        res.push(pattern);
    }

    res
}