use super::store::*;
use super::properties_repository::*;
use super::super::model::*;
use super::super::conf::Stores;
use super::super::graph::*;

pub struct GraphRepository {
    nodes_store: nodes_store::NodesStore,
    relationships_store: relationships_store::RelationshipsStore,
    properties_repository: PropertiesRespository,
}

impl GraphRepository {
    pub fn new(init_ctx: init::InitContext) -> Self {
        GraphRepository {nodes_store: nodes_store::NodesStore::new(&init_ctx.get_nodes_store_path().unwrap()),
            relationships_store: relationships_store::RelationshipsStore::new(&init_ctx.get_relationships_store_path().unwrap()),
            properties_repository: PropertiesRespository::new(&init_ctx.get_properties_store_path().unwrap(), &init_ctx.get_dynamic_store_path().unwrap())}
    }

    pub fn save(pgraph: PropertyGraph) {
        let node_list = pgraph.nodes.iter().map(|n|)
    }
}