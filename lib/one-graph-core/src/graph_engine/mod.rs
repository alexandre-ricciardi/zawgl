mod model;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::graph::EdgeIndex;
use crate::graph::NodeIndex;

use super::model::*;
use super::repository::graph_repository::GraphRepository;
use self::model::*;
use super::matcher::vf2::sub_graph_isomorphism;
use super::graph::traits::*;

pub struct GraphEngine {
    repository: Rc<RefCell<GraphRepository>>,
}

fn extract_nodes_labels(pattern: &PropertyGraph) -> Vec<String> {
    let mut res = Vec::new();
    for node in pattern.get_nodes() {
        node.get_labels_ref().iter().for_each(|l| res.push(l.to_owned()));
    }
    res
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
    pub fn new(ctx: &init::InitContext) -> Self {
        GraphEngine{repository: Rc::new(RefCell::new(GraphRepository::new(ctx)))}
    }

    pub fn create_graph(&mut self, graph: &PropertyGraph) -> Option<PropertyGraph> {
        self.repository.borrow_mut().create_graph(graph)
    }

    pub fn create_node(&mut self, node: &Node) -> Option<Node> {
        self.repository.borrow_mut().create_node(node)
    }

    pub fn match_pattern(&mut self, pattern: &PropertyGraph) -> Option<Vec<PropertyGraph>> {
        let mut graph_proxy = GraphProxy::new(self.repository.clone(), extract_nodes_labels(pattern));
        let mut res = Vec::new();
        sub_graph_isomorphism(pattern, &mut graph_proxy, 
        |n0, n1| {
            if n0.get_id() == n1.get_id() {
                return true;
            }
            let mut res = true;
            for p0 in n0.get_properties_ref() {
                if !n1.get_properties_ref().contains(p0) {
                    res = false;
                    break;
                }
            }
            res
        },
        |e0, e1| {
            if e0.get_id() == e1.get_id() {
                return true;
            }
            let mut res = true;
            for p0 in e0.get_properties_ref() {
                if !e1.get_properties_ref().contains(p0) {
                    res = false;
                    break;
                }
            }
            res
        },
        |map0, _map1, gpattern, proxy| {
            let mut res_match = PropertyGraph::new();
            for index in gpattern.get_nodes_ids() {
                let pattern_node = gpattern.get_node_ref(&index);
                let proxy_index = map0[&index];
                let mut proxy_node = proxy.get_node_ref(&proxy_index)?.clone();
                proxy_node.set_option_var(pattern_node.get_var());
                res_match.add_node(proxy_node);
            }
            for prel in gpattern.get_relationships_and_edges() {
                let psource_id = &prel.1.source;
                let ptarget_id = &prel.1.target;
                let proxy_source_id = map0[psource_id];
                let proxy_target_id = map0[ptarget_id];
                for rel_id in proxy.out_edges(&proxy_source_id) {
                    let target_id = proxy.get_target_index(&rel_id);
                    if target_id == proxy_target_id {
                        let rel = proxy.get_relationship_ref(&rel_id)?;
                        if compare_relationships(prel.0, rel) {
                            let mut rel_clone = rel.clone();
                            rel_clone.set_option_var(prel.0.get_var());
                            res_match.add_relationship(rel_clone, *psource_id, *ptarget_id);
                        }
                    }
                }
            }
            res.push(res_match);
            Some(true)
        });
        Some(res)
    }

    pub fn match_pattern_and_create(&mut self, pattern: &PropertyGraph) -> Option<Vec<PropertyGraph>> {
        let mut graph_proxy = GraphProxy::new(self.repository.clone(), extract_nodes_labels(pattern));

        let mut match_pattern = PropertyGraph::new();
        let mut map_nodes_ids = HashMap::new();
        let mut n_index = 0;
        for n in pattern.get_nodes() {
            if *n.get_status() == Status::Match {
                let pattern_n_index = match_pattern.add_node(n.clone());
                map_nodes_ids.insert(NodeIndex::new(n_index), pattern_n_index);
            }
            n_index += 1;
        }
        let mut r_index = 0;
        for r in pattern.get_relationships() {
            let e_index = EdgeIndex::new(r_index);
            let source_index = pattern.get_source_index(&e_index);
            let target_index = pattern.get_source_index(&e_index);
            if *r.get_status() == Status::Match {
                match_pattern.add_relationship(r.clone(), map_nodes_ids[&source_index], map_nodes_ids[&target_index]);
            }
            r_index += 1;
        }

        let res = self.match_pattern(pattern)?;

        for matched_graph in &res {
            for index in matched_graph.get_nodes_ids() {
            }
        }
        Some(res)
    }


    pub fn retrieve_graph() {

    }

    pub fn sync(&mut self) {
        self.repository.borrow_mut().sync();
    }
}



#[cfg(test)]
mod test_cache {
    use super::*;
    #[test]
    fn test_add_prop_graphs() {
        
    }

}