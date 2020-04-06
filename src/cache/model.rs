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

pub type CacheId = usize;
pub type StoreId = u64;
#[derive(Copy, Clone, PartialEq)]
pub struct Ids {
    store: Option<StoreId>,
    cache: Option<CacheId>,
}

impl Ids {
    fn new() -> Self {
        Ids {store: None, cache: None}
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
    map_store_to_cache_node_ids: HashMap<StoreId, CacheId>,
    map_store_to_cache_raltionship_ids: HashMap<StoreId, CacheId>,
}

impl CacheGraph {
    pub fn new() -> Self {
        CacheGraph{ nodes: Vec::new(), relationships: Vec::new(), 
            map_store_to_cache_node_ids: HashMap::new(),
            map_store_to_cache_raltionship_ids: HashMap::new() }
    }

    pub fn add_node(&mut self, node: &Node) -> CacheId {
        let size = self.nodes.len();
        let mut cn = CachedNode::new();
        cn.id.store = node.id;
        cn.id.cache = Some(size);
        self.nodes.push(cn);
        size
    }

    pub fn get_node_ref(&self, cache_id: CacheId) -> &CachedNode {
        &self.nodes[cache_id]
    }

    fn get_node_from_db_id(&self, db_id: u64) -> &CachedNode {
        &self.nodes[self.map_store_to_cache_node_ids[&db_id]]
    }

    pub fn add_relationship(&mut self, source: Ids, target: Ids) -> CacheId {
        let index = self.relationships.len();
        {
            let mut cr = CachedRelationship::new();
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

    fn check_node_exists(&self, node: &Node) -> bool {
        if let Some(id) = node.id {
            self.map_store_to_cache_node_ids.contains_key(&id)
        } else {
            false
        }
    }

    pub fn add_graph(&mut self, graph: PropertyGraph) {
        let mut node_cache_ids = Vec::new();
        for node in graph.get_nodes() {
            if self.check_node_exists(&node) {
                let existing_node_cache_id = node.id.and_then(|id|self.map_store_to_cache_node_ids.get(&id));
                if let Some(id) = existing_node_cache_id {
                    node_cache_ids.push(*id);
                }
            } else {
                let cache_id = self.add_node(&node);
                node_cache_ids.push(cache_id);
            }
        }

        let mut rel_cache_ids = Vec::new();
        for edge_data in graph.get_inner_graph().get_edges() {
            let source_ids: Ids;
            let target_ids: Ids;
            {
                let source = &self.nodes[node_cache_ids[edge_data.source]];
                source_ids = source.id;
            }
            {
                let target = &self.nodes[node_cache_ids[edge_data.target]];
                target_ids = target.id;
            }
            let rel_cache_id = self.add_relationship(source_ids, target_ids);
            rel_cache_ids.push(rel_cache_id);
        }

        for node_id in 0..node_cache_ids.len() {
            let mut outbound_edges = Vec::new();
            for edge_id in graph.get_inner_graph().out_degrees(node_id) {
                outbound_edges.push(edge_id);
            }
            let mut prev_rel_id = Ids::new();
            for outbound_edge_id in &outbound_edges {
                let rel_cache_id = rel_cache_ids[*outbound_edge_id];
                let cache_rel = &mut self.relationships[rel_cache_id];
                cache_rel.first_prev_rel_id = prev_rel_id;
                prev_rel_id = cache_rel.id;
            }
            outbound_edges.reverse();
            let mut next_rel_id = Ids::new();
            for outbound_edge_id in &outbound_edges {
                let rel_cache_id = rel_cache_ids[*outbound_edge_id];
                let cache_rel = &mut self.relationships[rel_cache_id];
                cache_rel.first_next_rel_id = next_rel_id;
                next_rel_id = cache_rel.id;
            }
            let mut inbound_edges = Vec::new();
            for edge_id in graph.get_inner_graph().in_degrees(node_id) {
                inbound_edges.push(edge_id);
            }
            for inbound_edge_id in &inbound_edges {
                let rel_cache_id = rel_cache_ids[*inbound_edge_id];
                let cache_rel = &mut self.relationships[rel_cache_id];
                cache_rel.second_prev_rel_id = prev_rel_id;
                prev_rel_id = cache_rel.id;
            }
            inbound_edges.reverse();
            for inbound_edge_id in &inbound_edges {
                let rel_cache_id = rel_cache_ids[*inbound_edge_id];
                let cache_rel = &mut self.relationships[rel_cache_id];
                cache_rel.second_next_rel_id = next_rel_id;
                next_rel_id = cache_rel.id;
            }
            {
                let node_cache_id = node_cache_ids[node_id];
                let current_cache_node = &mut self.nodes[node_cache_id];
                current_cache_node.next_rel_id = next_rel_id;
            }
        }
    }

}


#[cfg(test)]
mod test_cache_model {
    use super::*;
    #[test]
    fn test_add_prop_graphs() {
        let mut pgraph = PropertyGraph::new();
        pgraph.add_node();
        pgraph.add_node();
        pgraph.add_node();
        pgraph.add_node();

        pgraph.add_relationship(0, 1);
        pgraph.add_relationship(0, 2);
        pgraph.add_relationship(1, 3);
        pgraph.add_relationship(2, 3);

        let mut cgraph = CacheGraph::new();
        cgraph.add_graph(pgraph);

        let n0 = cgraph.get_node_ref(0);
        assert_eq!(n0.id.cache, Some(0));
        assert_eq!(n0.next_rel_id.cache, Some(1));

        let n1 = cgraph.get_node_ref(1);
        assert_eq!(n1.id.cache, Some(1));
        assert_eq!(n1.next_rel_id.cache, Some(0));

        let r0 = cgraph.get_relationship_ref(n0.next_rel_id.cache.unwrap());
        assert_eq!(r0.first_node.cache, Some(0));
        assert_eq!(r0.second_node.cache, Some(2));

        let r1 = cgraph.get_relationship_ref(r0.first_next_rel_id.cache.unwrap());
        assert_eq!(r1.first_node.cache, Some(0));
        assert_eq!(r1.second_node.cache, Some(1));
        assert_eq!(r1.first_next_rel_id.cache, None);
        assert_eq!(r1.first_prev_rel_id.cache, Some(1));
    }

}