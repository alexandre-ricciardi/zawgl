use super::store::*;
use super::properties_repository::*;
use super::super::model::*;
use super::super::repository::index::b_tree::*;
use self::records::*;
use std::collections::HashMap;
use std::collections::HashSet;
use super::super::graph::traits::*;
use super::super::graph::*;

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

    pub fn fetch_nodes_ids_with_labels(&mut self, labels: &Vec<String>) -> HashSet<u64> {
        let mut res = HashSet::new();
        for label in labels {
            let ids = self.nodes_labels_index.search(label);
            if let Some(node_ids) = &ids {
                res.extend(node_ids.iter());
            }
        }
        res
    }

    pub fn create(&mut self, pgraph: &PropertyGraph) -> Option<()> {
        let mut map_nodes = HashMap::new();
        let mut node_index = 0;
        let mut node_records = Vec::new();
        for node in pgraph.get_nodes() {
            let nr = NodeRecord::new();
            let nid = self.nodes_store.create(&nr)?;
            for label in &node.labels {
                self.nodes_labels_index.insert(label, nid);
            }
            map_nodes.insert(node_index, nid);
            node_records.push((nid, nr));
            node_index += 1;
        }

        let mut rel_index: usize = 0;
        let mut map_rel = HashMap::new();
        let mut rel_records = Vec::new();
        for rel in pgraph.get_edges() {
            let rr = RelationshipRecord::new(*map_nodes.get(&rel.source.get_index())?,
             *map_nodes.get(&rel.target.get_index())?);
            let rid = self.relationships_store.create(&rr)?;
            map_rel.insert(rel_index, rid);
            rel_records.push((rid, rr));
            rel_index += 1;
        }

        let mut nr_index = 0;
        for nr in &mut node_records {
            let vertex = pgraph.get_inner_graph().get_vertex(NodeIndex::new(nr_index));
            let in_edge_index = vertex.get_first_inbound_edge()?;
            nr.1.first_inbound_edge = *map_rel.get(&in_edge_index.get_index())?;
            let out_edge_index = vertex.get_first_outbound_edge()?;
            nr.1.first_outbound_edge = *map_rel.get(&out_edge_index.get_index())?;
            self.nodes_store.save(nr.0, &nr.1)?;
            nr_index += 1;
        }

        let mut rr_index = 0;
        for rr in &mut rel_records {
            let edge = pgraph.get_inner_graph().get_edge_data(EdgeIndex::new(rr_index));
            rr.1.next_outbound_edge = *map_rel.get(&edge.get_next_outbound_edge()?.get_index())?;
            rr.1.next_inbound_edge = *map_rel.get(&edge.get_next_inbound_edge()?.get_index())?;
            self.relationships_store.save(rr.0, &rr.1)?;
            rr_index += 1;
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