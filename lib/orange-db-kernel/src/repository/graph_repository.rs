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

    pub fn retrieve_node_by_id(&mut self, node_id: u64) -> Option<(Node, DbVertexData)> {
        let nr = self.nodes_store.load(node_id)?;
        let mut node = Node::new();
        node.set_id(Some(node_id));
        let mut vertex = DbVertexData::new();
        if nr.first_inbound_edge != 0 {
            vertex.first_inbound_edge = Some(nr.first_inbound_edge);
        }
        if nr.first_outbound_edge != 0 {
            vertex.first_outbound_edge = Some(nr.first_outbound_edge);
        }
        Some((node, vertex))
    }

    pub fn retrieve_relationship_by_id(&mut self, rel_id: u64) -> Option<(Relationship, DbEdgeData)> {
        let rr = self.relationships_store.load(rel_id)?;
        let mut rel = Relationship::new();
        rel.set_id(Some(rel_id));
        let mut edge = DbEdgeData::new(rr.source, rr.target);
        if rr.next_inbound_edge != 0 {
            edge.next_inbound_edge = Some(rr.next_inbound_edge);
        }
        if rr.next_outbound_edge != 0 {
            edge.next_outbound_edge = Some(rr.next_outbound_edge);
        }
        Some((rel, edge))
    }

    pub fn retrieve_sub_graph_around(&mut self, node_id: u64) -> Option<PropertyGraph> {
        let mut pg = PropertyGraph::new();
        let mut map_nodes = HashMap::new();
        let nr = self.nodes_store.load(node_id)?;
        let mut node = Node::new();
        node.set_id(Some(node_id));
        map_nodes.insert(node_id, pg.add_node(node));

        if nr.first_outbound_edge != 0 {
            let mut curr_rel_id = nr.first_outbound_edge;
            loop {
                if curr_rel_id == 0 {
                    break;
                }
                let rr = self.relationships_store.load(curr_rel_id)?;
                let mut rel = Relationship::new();
                rel.set_id(Some(curr_rel_id));

                let nr_target = self.nodes_store.load(rr.target)?;
                let mut target = Node::new();
                target.set_id(Some(rr.target));
                map_nodes.insert(rr.target, pg.add_node(target));
                pg.add_relationship(rel, map_nodes[&node_id], map_nodes[&rr.target]);
                curr_rel_id = rr.next_outbound_edge;
            }
        }
        
        if nr.first_inbound_edge != 0 {
            let mut curr_rel_id = nr.first_inbound_edge;
            loop {
                if curr_rel_id == 0 {
                    break;
                }
                let rr = self.relationships_store.load(curr_rel_id)?;
                let mut rel = Relationship::new();
                rel.set_id(Some(curr_rel_id));

                let nr_source = self.nodes_store.load(rr.source)?;
                let mut source = Node::new();
                source.set_id(Some(rr.source));
                map_nodes.insert(rr.source, pg.add_node(source));
                pg.add_relationship(rel, map_nodes[&node_id], map_nodes[&rr.source]);
                curr_rel_id = rr.next_inbound_edge;
            }
        }

        Some(pg)
    }

    pub fn create(&mut self, pgraph: &PropertyGraph) -> Option<()> {
        let mut map_nodes = HashMap::new();
        let mut node_index = 0;
        let mut node_records = Vec::new();
        for node in pgraph.get_nodes() {
            let nr = NodeRecord::new();
            let nid = self.nodes_store.create(&nr)?;
            for label in node.get_labels_ref() {
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
            let in_edge_index = vertex.get_first_inbound_edge();
            if let Some(in_edge) = in_edge_index {
                nr.1.first_inbound_edge = *map_rel.get(&in_edge.get_index())?;
            }
            
            let out_edge_index = vertex.get_first_outbound_edge();
            if let Some(out_edge) = out_edge_index {
                nr.1.first_outbound_edge = *map_rel.get(&out_edge.get_index())?;
            }
            
            self.nodes_store.save(nr.0, &nr.1)?;
            nr_index += 1;
        }

        let mut rr_index = 0;
        for rr in &mut rel_records {
            let edge = pgraph.get_inner_graph().get_edge_data(EdgeIndex::new(rr_index));
            if let Some(out_edge) = &edge.get_next_outbound_edge() {
                rr.1.next_outbound_edge = *map_rel.get(&out_edge.get_index())?;
            }
            if let Some(in_edge) = &edge.get_next_inbound_edge() {
                rr.1.next_inbound_edge = *map_rel.get(&in_edge.get_index())?;
            }
            
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

pub struct DbVertexData {
    pub first_inbound_edge: Option<u64>,
    pub first_outbound_edge: Option<u64>,
}

impl DbVertexData {
    fn new() -> Self {
        DbVertexData{first_inbound_edge: None, first_outbound_edge: None}
    }
}

pub struct DbEdgeData {
    pub source: u64,
    pub target: u64,
    pub next_outbound_edge: Option<u64>,
    pub next_inbound_edge: Option<u64>,
}

impl DbEdgeData {
    fn new(source: u64, target: u64) -> Self {
        DbEdgeData{source: source, target: target, next_outbound_edge: None, next_inbound_edge: None}
    }
}