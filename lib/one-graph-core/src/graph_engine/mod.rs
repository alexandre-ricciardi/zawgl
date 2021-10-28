pub mod model;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::model::*;
use super::repository::graph_repository::GraphRepository;
use self::model::*;
use super::matcher::vf2::sub_graph_isomorphism;
use super::graph::traits::*;

pub struct GraphEngine {
    repository: Rc<RefCell<GraphRepository>>,
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
    
    pub fn create_relationship(&mut self, rel: &Relationship, source_id: u64, target_id: u64) -> Option<Relationship> {
        self.repository.borrow_mut().create_relationship(rel, source_id, target_id)
    }

    pub fn match_pattern(&mut self, pattern: &PropertyGraph) -> Option<Vec<PropertyGraph>> {
        let mut graph_proxy = GraphProxy::new(self.repository.clone(), pattern)?;
        let mut res = Vec::new();
        sub_graph_isomorphism(pattern, &mut graph_proxy, 
        |n0, n1| {
            if n0.get_id() == None && n0.get_labels_ref().is_empty() {
                return true;
            }
            
            if n0.get_id() != None && n0.get_id() == n1.get_id() {
                return true;
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
                        if p1.get_name() == &pred.name {
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
            if e0.get_id() == None && e0.get_labels_ref().is_empty() {
                return true;
            }
            
            if e0.get_id() != None && e0.get_id() == e1.get_id() {
                return true;
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
                        if p1.get_name() == &pred.name {
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
        let mut match_pattern = PropertyGraph::new();
        let mut map_nodes_ids = HashMap::new();
        for nid in pattern.get_nodes_ids() {
            let n = pattern.get_node_ref(&nid);
            if *n.get_status() == Status::Match {
                let pattern_n_index = match_pattern.add_node(n.clone());
                map_nodes_ids.insert(nid, pattern_n_index);
            }
        }
        for re in pattern.get_relationships_and_edges() {
            let source_index = re.1.source;
            let target_index = re.1.target;
            if *re.0.get_status() == Status::Match {
                match_pattern.add_relationship(re.0.clone(), map_nodes_ids[&source_index], map_nodes_ids[&target_index]);
            }
        }

        let mut res = self.match_pattern(&match_pattern)?;

        for matched_graph in &mut res {
            for re in pattern.get_relationships_and_edges() {
                if *re.0.get_status() == Status::Create {
                    let source_index = map_nodes_ids[&re.1.source];
                    let target_index = map_nodes_ids[&re.1.target];
                    let source = matched_graph.get_node_ref(&source_index).get_id()?;
                    let target = matched_graph.get_node_ref(&target_index).get_id()?;
                    let res = self.create_relationship(re.0, source, target)?;
                    matched_graph.add_relationship(res, source_index, target_index);
                }
            }
        }
        Some(res)
    }


    pub fn retrieve_graph(&mut self) -> Option<GraphProxy> {
        GraphProxy::new_full(self.repository.clone())
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