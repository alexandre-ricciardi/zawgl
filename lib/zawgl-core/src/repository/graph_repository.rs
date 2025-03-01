// MIT License
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::index::model::Key;
use super::store::*;
use super::properties_repository::*;
use super::super::model::*;
use super::super::repository::index::b_tree::*;
use self::records::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;
use super::super::graph::traits::*;

fn parse_labels(labels: &str) -> Option<Vec<String>> {
    Some(labels.split(':').map(String::from).collect())
}

/// A graph repository enabling to store a directed property graph on the disk 
pub struct GraphRepository {
    nodes_store: nodes_store::NodesStore,
    relationships_store: relationships_store::RelationshipsStore,
    properties_repository: PropertiesRespository,
    nodes_labels_index: BTreeIndex,
    relationships_labels_index: BTreeIndex,
    labels_store: dynamic_store::DynamicStore,
    nodes_ids: Option<Vec<u64>>,
    labels: HashMap<String, u64>,
}

impl GraphRepository {

    /// Builds a GraphRepository with its context
    pub fn new(init_ctx: init::DatabaseInitContext) -> Self {
        let mut nodes_store = nodes_store::NodesStore::new(&init_ctx.get_nodes_store_path().unwrap());
        let nodes_ids = nodes_store.retrieve_all_nodes_ids();
        let mut labels_store = dynamic_store::DynamicStore::new(&init_ctx.get_labels_store_path().unwrap());
        let all_labels = labels_store.retrieve_all_string();
        let mut labels = HashMap::new();
        if let Some(stored_labels) = all_labels {
            for (label, id) in stored_labels {
                labels.insert(label, id);
            } 
        }
        GraphRepository {nodes_store, nodes_ids,
            relationships_store: relationships_store::RelationshipsStore::new(&init_ctx.get_relationships_store_path().unwrap()),
            properties_repository: PropertiesRespository::new(&init_ctx.get_properties_store_path().unwrap(), &init_ctx.get_dynamic_store_path().unwrap()),
            nodes_labels_index: BTreeIndex::new(&init_ctx.get_nodes_labels_index_path().unwrap()),
            relationships_labels_index: BTreeIndex::new(&init_ctx.get_relationships_types_index_path().unwrap()),
            labels_store, labels
        }
    }

    pub fn get_node_ids(&self) -> &Option<Vec<u64>> {
        &self.nodes_ids
    }

    /// Retrieve nodes IDs by labels
    pub fn fetch_nodes_ids_with_labels(&mut self, labels: &Vec<String>) -> HashSet<u64> {
        let mut res = HashSet::new();
        for label in labels {
            let ids = self.nodes_labels_index.search(&Key::from_str(label));
            if let Some(node_ids) = &ids {
                res.extend(node_ids.iter());
            }
        }
        res
    }
    
    /// Retrieve an node with its ID
    pub fn retrieve_node_by_id(&mut self, node_id: u64) -> Option<(Node, DbVertexData)> {
        let nr = self.nodes_store.load(node_id)?;
        let mut node = Node::new();
        node.set_id(Some(node_id));
        node.set_properties(self.properties_repository.retrieve_list(nr.next_prop_id)?);
        if nr.node_type != 0 {
            let labels = self.labels_store.load_string(nr.node_type)?;
            node.set_labels(parse_labels(&labels)?);
        }
        let mut vertex = DbVertexData::new();
        if nr.first_inbound_edge != 0 {
            vertex.first_inbound_edge = Some(nr.first_inbound_edge);
        }
        if nr.first_outbound_edge != 0 {
            vertex.first_outbound_edge = Some(nr.first_outbound_edge);
        }
        Some((node, vertex))
    }

    /// Retrieve a vertex with its ID
    pub fn retrieve_vertex_data_by_id(&mut self, node_id: u64) -> Option<DbVertexData> {
        let nr = self.nodes_store.load(node_id)?;
        let mut vertex = DbVertexData::new();
        if nr.first_inbound_edge != 0 {
            vertex.first_inbound_edge = Some(nr.first_inbound_edge);
        }
        if nr.first_outbound_edge != 0 {
            vertex.first_outbound_edge = Some(nr.first_outbound_edge);
        }
        Some(vertex)
    }

    /// Retrieve an relationship with its ID
    pub fn retrieve_relationship_by_id(&mut self, rel_id: u64) -> Option<(Relationship, DbEdgeData)> {
        let rr = self.relationships_store.load(rel_id)?;
        let mut rel = Relationship::new();
        rel.set_id(Some(rel_id));
        rel.set_properties(self.properties_repository.retrieve_list(rr.next_prop_id)?);
        if rr.relationship_type != 0 {
            let labels_data = self.labels_store.load_string(rr.relationship_type)?;
            rel.set_labels(parse_labels(&labels_data)?);
        }
        let mut edge = DbEdgeData::new(rr.source, rr.target);
        if rr.next_inbound_edge != 0 {
            edge.next_inbound_edge = Some(rr.next_inbound_edge);
        }
        if rr.next_outbound_edge != 0 {
            edge.next_outbound_edge = Some(rr.next_outbound_edge);
        }
        Some((rel, edge))
    }

    /// Retrieve an edge with its ID
    pub fn retrieve_edge_data_by_id(&mut self, rel_id: u64) -> Option<DbEdgeData> {
        let rr = self.relationships_store.load(rel_id)?;
        let mut edge = DbEdgeData::new(rr.source, rr.target);
        if rr.next_inbound_edge != 0 {
            edge.next_inbound_edge = Some(rr.next_inbound_edge);
        }
        if rr.next_outbound_edge != 0 {
            edge.next_outbound_edge = Some(rr.next_outbound_edge);
        }
        Some(edge)
    }

    /// Create a node and index its labels
    pub fn create_node(&mut self, node: &Node) -> Option<Node> {
        let node = self.create_node_with_properties(node)?;
        for label in node.get_labels_ref() {
            self.nodes_labels_index.insert(&Key::from_str(label), node.get_id()?);
        }
        Some(node)
    }
    
    /// Create a node with properties
    pub fn create_node_with_properties(&mut self, node: &Node) -> Option<Node> {
        let mut nr = NodeRecord::new();
        let mut res = node.clone();
        nr.next_prop_id = self.properties_repository.create_list(res.get_properties_mut())?;
        if !node.get_labels_ref().is_empty() {
            let full_label = node.get_labels_ref().join(":");
            if let Entry::Vacant(e) = self.labels.entry(full_label.to_string()) {
                let label_id = self.labels_store.save_data(full_label.as_bytes())?;
                e.insert(label_id);
                nr.node_type = label_id;
            } else {
                nr.node_type = self.labels[&full_label];
            }
        }
        let nid = self.nodes_store.create(&nr)?;
        if let Some(nids) = &mut self.nodes_ids {
            nids.push(nid);
        }
        res.set_id(Some(nid));
        Some(res)
    }
    
    /// Create a relationship with properties
    pub fn create_relationship_with_properties(&mut self, rel: &Relationship, source: u64, target: u64) -> Option<Relationship> {
        let mut source_record = self.nodes_store.load(source)?;
        let mut target_record = self.nodes_store.load(target)?;
        let mut rr = RelationshipRecord::new(source, target);
        rr.next_outbound_edge = source_record.first_outbound_edge;
        rr.next_inbound_edge = target_record.first_inbound_edge;
        let mut res = rel.clone();
        rr.next_prop_id = self.properties_repository.create_list(res.get_properties_mut())?;
        if !rel.get_labels_ref().is_empty() {
            let full_label = rel.get_labels_ref().join(":");
            if let Entry::Vacant(e) = self.labels.entry(full_label.to_string()) {
                let label_id = self.labels_store.save_data(full_label.as_bytes())?;
                e.insert(label_id);
                rr.relationship_type = label_id;
            } else {
                rr.relationship_type = self.labels[&full_label];
            }
        }
        let rid = self.relationships_store.create(&rr)?;
       
        res.set_id(Some(rid));

        if source == target {
            source_record.first_outbound_edge = rid;
            source_record.first_outbound_edge = rid;
            self.nodes_store.save(source, &source_record)?;
        } else {
            source_record.first_outbound_edge = rid;
            target_record.first_inbound_edge = rid;
            self.nodes_store.save(source, &source_record)?;
            self.nodes_store.save(target, &target_record)?;
        }
        Some(res)
    }

    /// Create a relationship and index its labels
    pub fn create_relationship(&mut self, rel: &Relationship, source: u64, target: u64) -> Option<Relationship> {
        let res = self.create_relationship_with_properties(rel, source, target)?;
        for label in rel.get_labels_ref() {
            self.relationships_labels_index.insert(&Key::from_str(label), res.get_id()?);
        }
        Some(res)
    }

    /// Create a graph and index its labels
    pub fn create_graph(&mut self, pgraph: &PropertyGraph) -> Option<PropertyGraph> {
        let mut res = pgraph.clone();
        let mut map_nodes = HashMap::new();
        let mut node_records = Vec::new();
        for (node_index, node) in res.get_nodes_mut().iter_mut().enumerate() {
            let cnode = self.create_node(node)?;
            map_nodes.insert(node_index, cnode.get_id()?);
            node_records.push(cnode);
        }

        let mut map_rel = HashMap::new();
        let mut rel_records = Vec::new();
        for (rel_index, edge) in res.get_edges_mut().iter_mut().enumerate() {
            let crel = self.create_relationship(&edge.relationship, 
                *map_nodes.get(&edge.source.get_index())?,
                *map_nodes.get(&edge.target.get_index())?)?;
            map_rel.insert(rel_index, crel.get_id()?);
            rel_records.push(crel);
        }
        
        for (n_index, n) in res.get_nodes_mut().iter_mut().enumerate() {
            n.set_id(Some(map_nodes[&n_index]));
        }

        for (r_index, r)  in res.get_relationships_mut().iter_mut().enumerate() {
            r.set_id(Some(map_rel[&r_index]));
        }

        Some(res)
    }

    /// Write data on disk
    pub fn sync(&mut self) {
        self.nodes_labels_index.sync();
        self.relationships_store.sync();
        self.nodes_store.sync();
        self.properties_repository.sync();
        self.labels_store.sync();
    }

    /// Clears the caches
    pub fn clear(&mut self) {
        self.nodes_labels_index.clear();
        self.relationships_store.clear();
        self.nodes_store.clear();
        self.properties_repository.clear();
        self.labels_store.clear();
    }

    pub fn rebuild_index(&mut self) -> Option<()> {
        self.nodes_labels_index.reset();
        let ids = self.nodes_store.retrieve_all_nodes_ids()?;
        for id in ids {
            let (node, _v) = self.retrieve_node_by_id(id)?;
            for label in node.get_labels_ref() {
                self.nodes_labels_index.insert(&Key::from_str(label), node.get_id()?);
            }
        }
        Some(())
    }
}

#[derive(Copy, Clone)]
pub struct DbVertexData {
    pub first_inbound_edge: Option<u64>,
    pub first_outbound_edge: Option<u64>,
}

impl DbVertexData {
    fn new() -> Self {
        DbVertexData{first_inbound_edge: None, first_outbound_edge: None}
    }
}


#[derive(Copy, Clone)]
pub struct DbEdgeData {
    pub source: u64,
    pub target: u64,
    pub next_outbound_edge: Option<u64>,
    pub next_inbound_edge: Option<u64>,
}

impl DbEdgeData {
    fn new(source: u64, target: u64) -> Self {
        DbEdgeData{source, target, next_outbound_edge: None, next_inbound_edge: None}
    }
}