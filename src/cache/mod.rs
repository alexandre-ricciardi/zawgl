mod model;

use super::model::*;
use super::repository::graph_repository::GraphRepository;
use self::model::*;
use std::collections::HashMap;

pub struct Cache {
    model: CacheGraph,
    repository: GraphRepository,
    map_store_to_cache_node_ids: HashMap<StoreId, CacheId>,
    map_store_to_cache_relationship_ids: HashMap<StoreId, CacheId>,
}

impl Cache {
    pub fn new(ctx: &init::InitContext) -> Self {
        Cache{ model: CacheGraph::new(), repository: GraphRepository::new(ctx),
            map_store_to_cache_node_ids: HashMap::new(), map_store_to_cache_relationship_ids: HashMap::new() }
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
        let mut node_cache_ids = Vec::new();
        for node in graph.get_nodes() {
            let existing_node_cache_id = node.id.and_then(|id|self.map_store_to_cache_node_ids.get(&id));
            if let Some(id) = existing_node_cache_id {
                node_cache_ids.push(*id);
            } else if let Some(id) = node.id {
                let node_cache_id = self.model.add_node(node.id);
                self.map_store_to_cache_node_ids.insert(id, node_cache_id);
                node_cache_ids.push(node_cache_id);
            } else {
                let node_store_id = self.repository.gen_node_id();
                let node_cache_id = self.model.add_node(Some(node_store_id));
                self.map_store_to_cache_node_ids.insert(node_store_id, node_cache_id);
                node_cache_ids.push(node_cache_id);
            }
        }

        let mut rel_cache_ids = Vec::new();
        for rel in graph.get_relationships_and_edges() {

            let source = self.model.get_node_ref(node_cache_ids[rel.1.source]);
            let source_ids = source.id;
            
            let target = self.model.get_node_ref(node_cache_ids[rel.1.target]);
            let target_ids = target.id;
            

            let existing_rel_cache_id = rel.0.id.and_then(|id|self.map_store_to_cache_relationship_ids.get(&id));
            if let Some(id) = existing_rel_cache_id {
                rel_cache_ids.push(*id);
            } else if let Some(id) = rel.0.id {
                let rel_cache_id = self.model.add_relationship(source_ids, target_ids, rel.0.id);
                self.map_store_to_cache_relationship_ids.insert(id, rel_cache_id);
                rel_cache_ids.push(rel_cache_id);
            } else {
                let rel_store_id = self.repository.gen_relationship_id();
                let rel_cache_id = self.model.add_relationship( source_ids, target_ids, Some(rel_store_id));
                self.map_store_to_cache_relationship_ids.insert(rel_store_id, rel_cache_id);
                rel_cache_ids.push(rel_cache_id);
            }
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
        let mut pgraph = PropertyGraph::new();
        pgraph.add_node();
        pgraph.add_node();
        pgraph.add_node();
        pgraph.add_node();

        pgraph.add_relationship(0, 1);
        pgraph.add_relationship(0, 2);
        pgraph.add_relationship(1, 3);
        pgraph.add_relationship(2, 3);
        let ctx = init::InitContext {
            conf: Conf {
                db_dir: String::from("C:\\Temp"),
                stores: Stores {
                    nodes_store: String::from("test_add_prop_graphs_nodes.db"),
                    relationships_store: String::from("test_add_prop_graphs_rels.db"),
                    properties_store: String::from("test_add_prop_graphs_props.db"),
                    dynamic_store: String::from("test_add_prop_graphs_dyn.db")
                }
            }
        };
        let mut cgraph = Cache::new(&ctx);
        cgraph.add_graph(&pgraph);

        let n0 = cgraph.model.get_node_ref(0);
        assert_eq!(n0.id.cache, Some(0));
        assert_eq!(n0.next_rel_id.cache, Some(1));

        let n1 = cgraph.model.get_node_ref(1);
        assert_eq!(n1.id.cache, Some(1));
        assert_eq!(n1.next_rel_id.cache, Some(0));

        let r0 = cgraph.model.get_relationship_ref(n0.next_rel_id.cache.unwrap());
        assert_eq!(r0.first_node.cache, Some(0));
        assert_eq!(r0.second_node.cache, Some(2));

        let r1 = cgraph.model.get_relationship_ref(r0.first_next_rel_id.cache.unwrap());
        assert_eq!(r1.first_node.cache, Some(0));
        assert_eq!(r1.second_node.cache, Some(1));
        assert_eq!(r1.first_next_rel_id.cache, None);
        assert_eq!(r1.first_prev_rel_id.cache, Some(1));
    }

}