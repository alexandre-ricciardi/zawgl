use super::store::*;
use super::properties_repository::*;
use super::super::model::*;
use super::super::repository::index::b_tree::*;

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

    pub fn save(pgraph: PropertyGraph) {
        //let node_list = pgraph.nodes.iter().map(|n|)
    }
}