mod model;

use std::cell::RefCell;
use std::rc::Rc;
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

    pub fn add_graph(&mut self, graph: &PropertyGraph) -> Option<()> {
        self.repository.borrow_mut().create(graph)
    }

    pub fn match_pattern(&mut self, pattern: &PropertyGraph) -> Option<Vec<PropertyGraph>> {
        let mut graph_proxy = GraphProxy::new(self.repository.clone(), extract_nodes_labels(pattern));
        let mut res = Vec::new();
        sub_graph_isomorphism(pattern, &mut graph_proxy, 
        |n0, n1| {
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
                let proxy_index = map0[&index];
                let proxy_node = proxy.get_node_ref(&proxy_index)?.clone();
                res_match.add_node(proxy_node);
            }
            for prel in pattern.get_relationships_and_edges() {
                let psource_id = &prel.1.source;
                let ptarget_id = &prel.1.target;
                let proxy_source_id = map0[psource_id];
                let proxy_target_id = map0[ptarget_id];
                for rel_id in proxy.out_edges(&proxy_source_id) {
                    let target_id = proxy.get_target_index(&rel_id);
                    if target_id == proxy_target_id {
                        let rel = proxy.get_relationship_ref(&rel_id)?;
                        if compare_relationships(prel.0, rel) {
                            res_match.add_relationship(rel.clone(), *psource_id, *ptarget_id);
                        }
                    }
                }
            }
            res.push(res_match);
            Some(true)
        });
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