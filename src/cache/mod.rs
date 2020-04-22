mod model;

use super::model::*;
use super::repository::graph_repository::GraphRepository;
use self::model::*;
use std::collections::HashMap;
use super::matcher::vf2::*;

pub struct Cache {
    model: GraphProxy<Node, Relationship>,
    repository: GraphRepository,
}

impl Cache {
    pub fn new(ctx: &init::InitContext) -> Self {
        Cache{ model: GraphProxy::new(ctx), repository: GraphRepository::new(ctx)}
    }

    ///compute item ids
    ///
    ///feed graph cache ?
    /// 
    ///store in mem records and models
    /// 
    ///grpah representation of cache
    /// 
    ///async save to disk invalidation graph cache or remove links in mem ?
    /// 
    ///compute cache size and give a size limit
    /// 
    ///map record id id in cache...
    pub fn add_graph(&mut self, graph: &PropertyGraph) {

    }

    pub fn match_pattern(pattern: &PropertyGraph) {
        //sub_graph_isomorphism();
    }

    pub fn retrieve_graph() {

    }

    pub fn sync_to_disk() {
        
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