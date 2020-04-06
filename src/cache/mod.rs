mod model;

use super::model::*;
use super::repository::graph_repository::GraphRepository;
use self::model::*;
use std::collections::HashMap;

pub struct Cache {
    model: CacheGraph,
    repository: GraphRepository,
    map_store_to_cache_node_ids: HashMap<StoreId, CacheId>,
}

impl Cache {
    pub fn new(ctx: &init::InitContext) -> Self {
        Cache{ model: CacheGraph::new(), repository: GraphRepository::new(ctx),
            map_store_to_cache_node_ids: HashMap::new() }
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
    pub fn add_graph(&mut self, graph: &mut PropertyGraph) {
        let mut node_cache_ids = Vec::new();
        for node in graph.get_nodes() {
            if self.check_node_exists(&node) {
                let existing_node_cache_id = node.id.and_then(|id|self.map_store_to_cache_node_ids.get(&id));
                if let Some(id) = existing_node_cache_id {
                    node_cache_ids.push(*id);
                }
            } else {
                let cache_id = self.model.add_node(&node);
                node_cache_ids.push(cache_id);
            }
        }

        let mut rel_cache_ids = Vec::new();
        for edge_data in graph.get_inner_graph().get_edges() {
            let source_ids: Ids;
            let target_ids: Ids;
            {
                let source = self.model.get_node_ref(node_cache_ids[edge_data.source]);
                source_ids = source.id;
            }
            {
                let target = self.model.get_node_ref(node_cache_ids[edge_data.target]);
                target_ids = target.id;
            }
            let rel_cache_id = self.model.add_relationship(source_ids, target_ids);
            rel_cache_ids.push(rel_cache_id);
        }

        for node_id in 0..node_cache_ids.len() {
            let mut outbound_edges = Vec::new();
            for edge_id in graph.get_inner_graph().out_degrees(node_id) {
                outbound_edges.push(edge_id);
            }
            let mut prev_rel_id = Ids::new_empty();
            for outbound_edge_id in &outbound_edges {
                let rel_cache_id = rel_cache_ids[*outbound_edge_id];
                let cache_rel = self.model.get_relationship_mut(rel_cache_id);
                cache_rel.first_prev_rel_id = prev_rel_id;
                prev_rel_id = cache_rel.id;
            }
            outbound_edges.reverse();
            let mut next_rel_id = Ids::new_empty();
            for outbound_edge_id in &outbound_edges {
                let rel_cache_id = rel_cache_ids[*outbound_edge_id];
                let cache_rel = self.model.get_relationship_mut(rel_cache_id);
                cache_rel.first_next_rel_id = next_rel_id;
                next_rel_id = cache_rel.id;
            }
            let mut inbound_edges = Vec::new();
            for edge_id in graph.get_inner_graph().in_degrees(node_id) {
                inbound_edges.push(edge_id);
            }
            for inbound_edge_id in &inbound_edges {
                let rel_cache_id = rel_cache_ids[*inbound_edge_id];
                let cache_rel = self.model.get_relationship_mut(rel_cache_id);
                cache_rel.second_prev_rel_id = prev_rel_id;
                prev_rel_id = cache_rel.id;
            }
            inbound_edges.reverse();
            for inbound_edge_id in &inbound_edges {
                let rel_cache_id = rel_cache_ids[*inbound_edge_id];
                let cache_rel = self.model.get_relationship_mut(rel_cache_id);
                cache_rel.second_next_rel_id = next_rel_id;
                next_rel_id = cache_rel.id;
            }
            {
                let node_cache_id = node_cache_ids[node_id];
                let current_cache_node = self.model.get_node_mut(node_cache_id);
                current_cache_node.next_rel_id = next_rel_id;
            }
        }

    }
    fn check_node_exists(&self, node: &Node) -> bool {
        if let Some(id) = node.id {
            self.map_store_to_cache_node_ids.contains_key(&id)
        } else {
            false
        }
    }
    fn reserve_node_id(&mut self) -> u64 {
        0   
    }

    pub fn retrieve_graph() {

    }

    pub fn sync_to_disk() {

    }
}