use super::model::*;
use super::repository::store::records::*;
pub struct GraphCache {
    node_records: Vec<NodeRecord>,
    relationship_records: Vec<RelationshipRecord>,
    properties_records: Vec<PropertyRecord>
}

impl GraphCache {
    // pub fn new() -> Self {
    //     GraphCache{}
    // }

    pub fn add_graph(&mut self, graph: PropertyGraph) {
        //compute item ids
        //feed graph cache ?
        //store in mem records and models
        //grpah representation of cache
        //async save to disk invalidation graph cache or remove links in mem ?
        //compute cache size and give a size limit
        //map record id id in cache...
    }

    pub fn retrieve_graph() {

    }

    pub fn sync_to_disk() {

    }
}