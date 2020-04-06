mod model;

use super::model::*;
use self::model::*;

pub struct Cache {
    cache_model: CacheGraph,
}

impl Cache {
    pub fn new() -> Self {
        Cache{ cache_model: CacheGraph::new() }
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
    pub fn add_graph(&mut self, graph: PropertyGraph) {
       
    }

    pub fn retrieve_graph() {

    }

    pub fn sync_to_disk() {

    }
}