use super::store::*;
use super::properties_repository::*;
use super::super::model::*;
use super::super::repository::index::b_tree::*;
use self::records::*;
use std::collections::HashMap;
use super::super::graph::traits::*;

pub struct GraphRepository {
    nodes_store: nodes_store::NodesStore,
    relationships_store: relationships_store::RelationshipsStore,
    properties_repository: PropertiesRespository,
    nodes_labels_index: BTreeIndex,
}

impl GraphRepository {
    pub fn new(init_ctx: &init::InitContext) -> Self {
        GraphRepository {nodes_store: nodes_store::NodesStore::new(&init_ctx.get_nodes_store_path().unwrap()),
            relationships_store: relationships_store::RelationshipsStore::new(&init_ctx.get_relationships_store_path().unwrap()),
            properties_repository: PropertiesRespository::new(&init_ctx.get_properties_store_path().unwrap(), &init_ctx.get_dynamic_store_path().unwrap()),
            nodes_labels_index: BTreeIndex::new(&init_ctx.get_nodes_labels_index_path().unwrap())}
    }

    pub fn fetch_nodes_ids_with_labels(&mut self, labels: &Vec<String>) -> Option<Vec<u64>> {
        let mut res = Vec::new();
        for label in labels {
            let ids = self.nodes_labels_index.search(label)?;
            res.extend(ids.iter());
        }
        Some(res)
    }

    pub fn save(&mut self, pgraph: &PropertyGraph) -> Option<()> {
        let mut map_nodes = HashMap::new();
        let mut node_index = 0;
        for node in pgraph.get_nodes() {
            let nr = NodeRecord::new();
            let nid = self.nodes_store.create(&nr)?;
            for label in &node.labels {
                self.nodes_labels_index.insert(label, nid)?;
            }
            map_nodes.insert(node_index, nid);
            node_index += 1;
        }

        for rel in pgraph.get_relationships_and_edges() {
            let rr = RelationshipRecord::new(*map_nodes.get(&rel.1.source.get_index())?,
             *map_nodes.get(&rel.1.target.get_index())?);
            let rid = self.relationships_store.create(&rr)?;

        }
        Some(())
    }

    pub fn sync(&mut self) {
        self.nodes_labels_index.sync();
        self.relationships_store.sync();
        self.nodes_store.sync();
        self.properties_repository.sync();
    }
}