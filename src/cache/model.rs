use super::super::model::*;
use std::collections::HashMap;

pub struct CachedNode {
    pub id: Ids,
    pub is_stored: bool,
    pub next_rel_id: Ids,
}

impl CachedNode {
    pub fn new() -> Self {
        CachedNode {id: Ids::new(), is_stored: false, next_rel_id: Ids::new()}
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Ids {
    db: Option<u64>,
    local: Option<usize>,
}

impl Ids {
    fn new() -> Self {
        Ids {db: None, local: None}
    }
}

pub struct CachedRelationship {
    pub id: Ids,
    pub is_stored: bool,
    pub first_node: Ids,
    pub second_node: Ids,
    pub relationship_type: Ids,
    pub first_prev_rel_id: Ids,
    pub first_next_rel_id: Ids,
    pub second_prev_rel_id: Ids,
    pub second_next_rel_id: Ids,
}

impl CachedRelationship {
    pub fn new() -> Self {
        CachedRelationship {id: Ids::new(), is_stored: false, first_node: Ids::new(), second_node: Ids::new(), 
        relationship_type: Ids::new(), first_prev_rel_id: Ids::new(), first_next_rel_id: Ids::new(), second_prev_rel_id: Ids::new(), second_next_rel_id: Ids::new()}
    }
}

pub struct CacheGraph {
    nodes: Vec<CachedNode>,
    relationships: Vec<CachedRelationship>,
    map_db_to_cache_node_ids: HashMap<u64, usize>,
    map_cache_to_db_node_ids: HashMap<usize, u64>,
}

impl CacheGraph {
    pub fn new() -> Self {
        CacheGraph{ nodes: Vec::new(), relationships: Vec::new() }
    }

    pub fn add_node(&mut self, node: &Node) -> usize {
        let size = self.nodes.len();
        let mut cn = CachedNode::new();
        cn.id.db = node.id;
        cn.id.local = Some(size);
        self.nodes.push(cn);
        size
    }

    fn get_node_from_db_id(&self, db_id: u64) -> &CachedNode {
        &self.nodes[self.map_db_to_cache_node_ids[&db_id]]
    }

    pub fn add_relationship(&mut self, source: &mut CachedNode, target: &mut CachedNode) -> usize {
        let index = self.relationships.len();
        {
            let mut cr = CachedRelationship::new();
            cr.id.local = Some(index);
            cr.first_node = source.id;
            cr.second_node = target.id;
            self.relationships.push(cr);
        }

        index
    }

    fn check_node_exists(&self, node: &Node) -> bool {
        if let Some(id) = node.id {
            self.map_db_to_cache_node_ids.contains_key(&id)
        } else {
            false
        }
    }

    pub fn add_graph(&mut self, graph: PropertyGraph) {
        let mut node_ids = Vec::new();
        for node in graph.get_nodes() {
            if self.check_node_exists(&node) {
                let existing_node_cache_id = node.id.and_then(|id|self.map_db_to_cache_node_ids.get(&id));
                if let Some(id) = existing_node_cache_id {
                    node_ids.push(*id);
                }
            } else {
                let cache_id = self.add_node(&node);
                node_ids.push(cache_id);
            }
        }

        let mut count_rel = 0;
        for rel in graph.get_relationships() {
            let pgraph_edge = graph.get_inner_graph().get_edge(count_rel);
            let source = &mut self.nodes[node_ids[pgraph_edge.source]];
            let target = &mut self.nodes[node_ids[pgraph_edge.target]];
            let rel_cache_id = self.add_relationship(source, target);
            
            count_rel += 1;
        }
    }

}