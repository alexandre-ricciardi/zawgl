mod model;

use super::model::*;
use super::repository::graph_repository::GraphRepository;
use self::model::*;
use super::matcher::vf2::sub_graph_isomorphism;
use super::graph::traits::*;
use super::graph::NodeIndex;
use std::collections::HashMap;

pub struct GraphEngine {
    repository: GraphRepository,
}

fn extract_nodes_labels(pattern: &PropertyGraph) -> Vec<String> {
    let mut res = Vec::new();
    for node in pattern.get_nodes() {
        node.labels.iter().for_each(|l| res.push(l.to_owned()));
    }
    res
}

fn compare_relationships(r0: &Relationship, r1: &Relationship) -> bool {
    let mut res = true;
    for p0 in &r0.properties {
        if !r1.properties.contains(p0) {
            res = false;
            break;
        }
    }
    res
}

impl GraphEngine {
    pub fn new(ctx: &init::InitContext) -> Self {
        GraphEngine{repository: GraphRepository::new(ctx)}
    }

    pub fn add_graph(&mut self, graph: &PropertyGraph) {
        
    }

    pub fn match_pattern(&mut self, pattern: &PropertyGraph) -> Option<Vec<PropertyGraph>> {
        let mut graph_proxy = GraphProxy::new(&mut self.repository, extract_nodes_labels(pattern));
        let mut res = Vec::new();
        sub_graph_isomorphism(pattern, &mut graph_proxy, |n0, n1| {
            let mut res = true;
            for p0 in &n0.properties {
                if !n1.properties.contains(p0) {
                    res = false;
                    break;
                }
            }
            res
        },
        |e0, e1| {
            let mut res = true;
            for p0 in &e0.properties {
                if !e1.properties.contains(p0) {
                    res = false;
                    break;
                }
            }
            res
        },
        |map0, map1, gpattern, proxy| {
            let mut res_match = PropertyGraph::new();
            for index in gpattern.get_nodes_ids() {
                let proxy_index = map0[&index];
                let proxy_node = proxy.get_node_ref(&proxy_index);
                res_match.add_node(proxy_node.clone());
            }
            for prel in pattern.get_relationships_and_edges() {
                let psource_id = &prel.1.source;
                let ptarget_id = &prel.1.target;
                let proxy_source_id = map0[psource_id];
                let proxy_target_id = map0[ptarget_id];
                for rel_id in proxy.out_edges(&proxy_source_id) {
                    let target_id = proxy.get_target_index(&rel_id);
                    if target_id == &proxy_target_id {
                        let rel = proxy.get_relationship_ref(&rel_id);
                        if compare_relationships(prel.0, rel) {
                            res_match.add_relationship(rel.clone(), *psource_id, *ptarget_id);
                        }
                    }
                }
            }
            res.push(res_match);
            true
        });
        Some(res)
    }

   

    pub fn retrieve_graph() {

    }

    pub fn sync() {
        
    }
}



#[cfg(test)]
mod test_cache {
    use super::*;
    #[test]
    fn test_add_prop_graphs() {
        
    }

}