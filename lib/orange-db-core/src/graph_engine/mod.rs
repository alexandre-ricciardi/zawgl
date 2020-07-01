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

    pub fn match_pattern(&mut self, pattern: &PropertyGraph) {
        let mut graph_proxy = GraphProxy::new(&self.repository, extract_nodes_labels(pattern));
        
        //sub_graph_isomorphism();
    }

   

    pub fn retrieve_graph() {

    }

    pub fn sync() {
        
    }
}



#[cfg(test)]
mod test_cache {
    use super::*;
    use super::super::conf::*;
    #[test]
    fn test_add_prop_graphs() {
        
    }

}