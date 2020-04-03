use super::super::model::*;
pub struct CachedNode {
    pub node: Node,
    pub is_stored: bool,
    pub store_next_rel_id: u64,
}

pub struct CachedRelationship {
    pub relationship: Relationship,
    pub is_stored: bool,
    pub first_node: u64,
    pub second_node: u64,
    pub relationship_type: u64,
    pub first_prev_rel_id: u64,
    pub first_next_rel_id: u64,
    pub second_prev_rel_id: u64,
    pub second_next_rel_id: u64,
}

pub struct CacheGraph {
    nodes: Vec<CachedNode>,
    relationships: Vec<CachedRelationship>,
}

impl CacheGraph {
    pub fn new() -> Self {
        CacheGraph{ nodes: Vec::new(), relationships: Vec::new() }
    }

    pub fn add_graph(graph: PropertyGraph) {
        for node in graph.get_nodes() {
            
        }
    }

}