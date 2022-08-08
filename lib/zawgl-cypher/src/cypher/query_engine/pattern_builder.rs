use zawgl_core::{model::PropertyGraph};
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
            full_set.insert(*id);
        }
    }
    

    for id in 0..paths.len() {
        if !full_set.contains(&id) {
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
                let mut s = e.get_source();
                let mut t = e.get_target();
                if !map_path_node_ids.contains_key(&t) {
                    let tn = path.get_node_ref(&t);
                    if let Some(vname) = tn.get_var() {
                        t = merge_vars_map[vname];
                    }
                }
                if !map_path_node_ids.contains_key(&s) {
                    let sn = path.get_node_ref(&s);
                    if let Some(vname) = sn.get_var() {
                        s = merge_vars_map[vname];
                    }
                }
                let r = &e.relationship;
                pattern.add_relationship(r.clone(), map_path_node_ids[&s], map_path_node_ids[&t]);
            }
        }
        res.push(pattern);
    }

    res
}


#[cfg(test)]
mod test_patterns_builder {
    use zawgl_core::model::{Node, Relationship};

    use super::*;

    #[test]
    fn test_same_pattern() {
        let mut p0 = PropertyGraph::new();
        {
            let mut n0 = Node::new();
            n0.set_var("a");
            let mut n1 = Node::new();
            n1.set_var("b");
            let i0 = p0.add_node(n0);
            let i1 = p0.add_node(n1);
            p0.add_relationship(Relationship::new(), i0, i1);
        }
        let mut p1 = PropertyGraph::new();
        {
            let mut n0 = Node::new();
            n0.set_var("a");
            let mut n1 = Node::new();
            n1.set_var("c");
            let i0 = p1.add_node(n0);
            let i1 = p1.add_node(n1);
            p1.add_relationship(Relationship::new(), i0, i1);
        }

        let patterns = merge_paths(&vec![p0, p1]);

        assert_eq!(1, patterns.len());
        let pattern = &patterns[0];
        assert_eq!(3, pattern.get_nodes().len());
    }

    #[test]
    fn test_two_patterns() {
        let mut p0 = PropertyGraph::new();
        {
            let mut n0 = Node::new();
            n0.set_var("a");
            let mut n1 = Node::new();
            n1.set_var("b");
            let i0 = p0.add_node(n0);
            let i1 = p0.add_node(n1);
            p0.add_relationship(Relationship::new(), i0, i1);
        }
        let mut p1 = PropertyGraph::new();
        {
            let mut n0 = Node::new();
            n0.set_var("d");
            let mut n1 = Node::new();
            n1.set_var("c");
            let i0 = p1.add_node(n0);
            let i1 = p1.add_node(n1);
            p1.add_relationship(Relationship::new(), i0, i1);
        }

        let patterns = merge_paths(&vec![p0, p1]);

        assert_eq!(2, patterns.len());
        let pattern0 = &patterns[0];
        assert_eq!(2, pattern0.get_nodes().len());
        let pattern1 = &patterns[1];
        assert_eq!(2, pattern1.get_nodes().len());
    }

    
    #[test]
    fn test_unique_pattern() {
        let mut p0 = PropertyGraph::new();
        {
            let mut n0 = Node::new();
            n0.set_var("a");
            p0.add_node(n0);
        }

        let patterns = merge_paths(&vec![p0]);

        assert_eq!(1, patterns.len());
        let pattern = &patterns[0];
        assert_eq!(1, pattern.get_nodes().len());
    }

}