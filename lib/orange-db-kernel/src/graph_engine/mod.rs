mod model;

use super::model::*;
use super::repository::graph_repository::GraphRepository;
use self::model::*;
use super::matcher::vf2::sub_graph_isomorphism;


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

impl GraphEngine {
    pub fn new(ctx: &init::InitContext) -> Self {
        GraphEngine{repository: GraphRepository::new(ctx)}
    }

    pub fn add_graph(&mut self, graph: &PropertyGraph) {
        
    }

    pub fn match_pattern(&mut self, pattern: &PropertyGraph) -> Option<Vec<PropertyGraph>> {
        let graph_proxy = GraphProxy::new(&self.repository, extract_nodes_labels(pattern));
        let mut res = Vec::new();
        sub_graph_isomorphism(pattern, &graph_proxy, |n0, n1| {
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
        |map0, map1| {
            let mut res_match = PropertyGraph::new();
            for node in pattern.get_nodes() {

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