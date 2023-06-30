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

use super::store::*;
use super::properties_repository::*;
use super::super::model::*;
use super::super::repository::index::b_tree::*;
use self::records::*;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::collections::HashSet;
use super::super::graph::traits::*;

fn parse_labels(labels: &str) -> Option<Vec<String>> {
    Some(labels.split(":").map(|s| String::from(s)).collect())
}

pub struct GraphRepository {
    nodes_store: nodes_store::NodesStore,
    relationships_store: relationships_store::RelationshipsStore,
    properties_repository: PropertiesRespository,
    nodes_labels_index: BTreeIndex,
    relationships_labels_index: BTreeIndex,
    labels_store: dynamic_store::DynamicStore,
    nodes_ids: Option<Vec<u64>>,
}

impl GraphRepository {
    pub fn new(init_ctx: init::InitContext) -> Self {
        let mut nodes_store = nodes_store::NodesStore::new(&init_ctx.get_nodes_store_path().unwrap());
        let nodes_ids = nodes_store.borrow_mut().retrieve_all_nodes_ids();
        GraphRepository {nodes_store: nodes_store, nodes_ids,
            relationships_store: relationships_store::RelationshipsStore::new(&init_ctx.get_relationships_store_path().unwrap()),
            properties_repository: PropertiesRespository::new(&init_ctx.get_properties_store_path().unwrap(), &init_ctx.get_dynamic_store_path().unwrap()),
            nodes_labels_index: BTreeIndex::new(&init_ctx.get_nodes_labels_index_path().unwrap()),
            relationships_labels_index: BTreeIndex::new(&init_ctx.get_relationships_types_index_path().unwrap()),
            labels_store: dynamic_store::DynamicStore::new(&init_ctx.get_labels_store_path().unwrap()),
        }
    }

    pub fn get_node_ids(&self) -> &Option<Vec<u64>> {
        &self.nodes_ids
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

    pub fn create_node(&mut self, node: &Node) -> Option<Node> {
        let mut nr = NodeRecord::new();
        let mut res = node.clone();
        nr.next_prop_id = self.properties_repository.create_list(res.get_properties_mut())?;
        if !node.get_labels_ref().is_empty() {
            nr.node_type = self.labels_store.save_data(node.get_labels_ref().join(":").as_bytes())?;
        }
        let nid = self.nodes_store.create(&nr)?;
        for label in node.get_labels_ref() {
            self.nodes_labels_index.insert(label, nid);
        }
        if let Some(nids) = &mut self.nodes_ids {
            nids.push(nid);
        }
        res.set_id(Some(nid));
        Some(res)
    }
    

    pub fn create_relationship(&mut self, rel: &Relationship, source: u64, target: u64) -> Option<Relationship> {
        let mut source_record = self.nodes_store.load(source)?;
        let mut target_record = self.nodes_store.load(target)?;
        let mut rr = RelationshipRecord::new(source, target);
        rr.next_outbound_edge = source_record.first_outbound_edge;
        rr.next_inbound_edge = target_record.first_inbound_edge;
        let mut res = rel.clone();
        rr.next_prop_id = self.properties_repository.create_list(res.get_properties_mut())?;
        if !rel.get_labels_ref().is_empty() {
            rr.relationship_type = self.labels_store.save_data(rel.get_labels_ref().join(":").as_bytes())?;
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
        
        for label in rel.get_labels_ref() {
            self.relationships_labels_index.insert(label, rid);
        }

        Some(res)
    }

    pub fn create_graph(&mut self, pgraph: &PropertyGraph) -> Option<PropertyGraph> {
        let mut res = pgraph.clone();
        let mut map_nodes = HashMap::new();
        let mut node_index = 0;
        let mut node_records = Vec::new();
        for node in res.get_nodes_mut() {
            let cnode = self.create_node(node)?;
            map_nodes.insert(node_index, cnode.get_id()?);
            node_records.push(cnode);
            node_index += 1;
        }

        let mut rel_index: usize = 0;
        let mut map_rel = HashMap::new();
        let mut rel_records = Vec::new();
        for edge in res.get_edges_mut() {
            let crel = self.create_relationship(&edge.relationship, 
                *map_nodes.get(&edge.source.get_index())?,
                *map_nodes.get(&edge.target.get_index())?)?;
            map_rel.insert(rel_index, crel.get_id()?);
            rel_records.push(crel);
            rel_index += 1;
        }
        
        let mut n_index = 0;
        for n in res.get_nodes_mut() {
            n.set_id(Some(map_nodes[&n_index]));
            n_index += 1;
        }

        let mut r_index = 0;
        for r  in res.get_relationships_mut() {
            r.set_id(Some(map_rel[&r_index]));
            r_index += 1;
        }

        Some(res)
    }

    pub fn sync(&mut self) {
        self.nodes_labels_index.sync();
        self.relationships_store.sync();
        self.nodes_store.sync();
        self.properties_repository.sync();
        self.labels_store.sync();
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
        DbEdgeData{source: source, target: target, next_outbound_edge: None, next_inbound_edge: None}
    }
}