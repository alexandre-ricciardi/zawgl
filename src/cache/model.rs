use super::super::model::*;
use std::collections::HashMap;

pub struct CachedNode {
    pub id: Ids,
    pub is_stored: bool,
    pub next_rel_id: Ids,
}

impl CachedNode {
    pub fn new() -> Self {
        CachedNode {id: Ids::new_empty(), is_stored: false, next_rel_id: Ids::new_empty()}
    }
}

pub type CacheId = usize;
pub type StoreId = u64;
#[derive(Copy, Clone, PartialEq)]
pub struct Ids {
    pub store: Option<StoreId>,
    pub cache: Option<CacheId>,
}

impl Ids {
    pub fn new_empty() -> Self {
        Ids {store: None, cache: None}
    }
    pub fn new(s_id: StoreId, c_id: CacheId) -> Self {
        Ids {store: Some(s_id), cache: Some(c_id)}
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
        CachedRelationship {id: Ids::new_empty(), is_stored: false, first_node: Ids::new_empty(), second_node: Ids::new_empty(), 
        relationship_type: Ids::new_empty(), first_prev_rel_id: Ids::new_empty(), first_next_rel_id: Ids::new_empty(), second_prev_rel_id: Ids::new_empty(), second_next_rel_id: Ids::new_empty()}
    }
}

pub struct CacheGraph {
    nodes: Vec<CachedNode>,
    relationships: Vec<CachedRelationship>,
}

impl CacheGraph {
    pub fn new() -> Self {
        CacheGraph{nodes: Vec::new(), relationships: Vec::new()}
    }

    pub fn add_node(&mut self, store_id: Option<StoreId>) -> CacheId {
        let size = self.nodes.len();
        let mut cn = CachedNode::new();
        cn.id.store = store_id;
        cn.id.cache = Some(size);
        self.nodes.push(cn);
        size
    }

    pub fn get_node_ref(&self, cache_id: CacheId) -> &CachedNode {
        &self.nodes[cache_id]
    }

    pub fn get_node_mut(&mut self, cache_id: CacheId) -> &mut CachedNode {
        &mut self.nodes[cache_id]
    }
    
    pub fn add_relationship(&mut self, source: Ids, target: Ids, store_id: Option<StoreId>) -> CacheId {
        let index = self.relationships.len();
        {
            let mut cr = CachedRelationship::new();
            cr.id.store = store_id;
            cr.id.cache = Some(index);
            cr.first_node = source;
            cr.second_node = target;
            self.relationships.push(cr);
        }
        index
    }

    pub fn get_relationship_ref(&self, cache_id: CacheId) -> &CachedRelationship {
        &self.relationships[cache_id]
    }
    pub fn get_relationship_mut(&mut self, cache_id: CacheId) -> &mut CachedRelationship {
        &mut self.relationships[cache_id]
    }

}


#[cfg(test)]
mod test_cache_model {
    use super::*;
    fn test_add_prop_graphs() {
    }

}