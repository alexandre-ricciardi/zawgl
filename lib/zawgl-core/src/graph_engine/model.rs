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

use super::super::model::*;
use super::super::graph::traits::*;
use super::super::repository::graph_repository::*;

use std::collections::hash_map::Entry;
use std::hash::{Hash, Hasher};
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug)]
pub struct ProxyNodeId {
    pub mem_id: Option<usize>,
    pub store_id: u64,
}

impl PartialEq for ProxyNodeId {
    fn eq(&self, other: &Self) -> bool {
        self.store_id == other.store_id
    }
}
impl Eq for ProxyNodeId {}
impl Hash for ProxyNodeId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.store_id.hash(state);
    }
}

impl ProxyNodeId {

    fn new_db(db_id: u64) -> Self {
        ProxyNodeId{mem_id: None, store_id: db_id}
    }
    fn new(mem_id: usize, db_id: u64) -> Self {
        ProxyNodeId{mem_id: Some(mem_id), store_id: db_id}
    }
    fn get_store_id(&self) -> u64 {
        self.store_id
    }
    fn get_index(&self) -> Option<usize> {
        self.mem_id
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct ProxyRelationshipId {
    mem_id: Option<usize>,
    store_id: u64,
}

impl ProxyRelationshipId {
    fn new_db(db_id: u64) -> Self {
        ProxyRelationshipId{mem_id: None, store_id: db_id}
    }
    fn new(mem_id: usize, db_id: u64) -> Self {
        ProxyRelationshipId{mem_id: Some(mem_id), store_id: db_id}
    }
    fn get_store_id(&self) -> u64 {
        self.store_id
    }
    fn get_index(&self) -> Option<usize> {
        self.mem_id
    }
}

#[derive(Clone)]
pub struct InnerVertexData<EID> {
    first_outbound_edge: Option<EID>,
    first_inbound_edge: Option<EID>,
    node: Option<Node>,
}

#[derive(Clone)]
pub struct InnerEdgeData<NID, EID> {
    pub source: NID,
    pub target: NID,
    pub next_outbound_edge: Option<EID>,
    pub next_inbound_edge: Option<EID>,
    pub relationship: Option<Relationship>,
}

pub struct GraphProxy<'a> {
    vertices: Vec<InnerVertexData<ProxyRelationshipId>>,
    edges: Vec<InnerEdgeData<ProxyNodeId, ProxyRelationshipId>>,
    repository: &'a mut GraphRepository,
    retrieved_nodes_ids: Vec<ProxyNodeId>,
    map_vertices: HashMap<u64, (ProxyNodeId, DbVertexData)>,
    map_edges: HashMap<u64, (ProxyRelationshipId, DbEdgeData)>,
}


impl <'a> GrowableGraphContainerTrait<ProxyNodeId, ProxyRelationshipId, Node, Relationship> for GraphProxy<'a> {

    fn get_node_ref(&mut self, pid: &ProxyNodeId) -> Option<&Node> {
        let db_id = pid.get_store_id();
        if let Entry::Vacant(e) = self.map_vertices.entry(db_id) {
            let index = self.vertices.len();
            let node = self.repository.retrieve_node_by_id(db_id)?;
            let inbound = node.1.first_inbound_edge.map(ProxyRelationshipId::new_db);
            let outbound = node.1.first_outbound_edge.map(ProxyRelationshipId::new_db);
            let vdata = InnerVertexData{first_outbound_edge: outbound, first_inbound_edge: inbound, node: Some(node.0.clone())};
            self.vertices.push(vdata);
            e.insert((ProxyNodeId::new(index, db_id), node.1));
            self.vertices[index].node.as_ref()
        } else {
            let (pid, vdata) = self.map_vertices[&db_id];
            let index = pid.get_index()?;
            if let None = self.vertices[index].node {
                let node = self.repository.retrieve_node_by_id(db_id)?;
                self.vertices[index].node = Some(node.0);
            }
            self.vertices[index].node.as_ref()
        }   
    }

    fn get_relationship_ref(&mut self, id: &ProxyRelationshipId) -> Option<&Relationship> {
        let db_id = id.get_store_id();
        if let Entry::Vacant(e) = self.map_edges.entry(db_id) {
            let (rel, db_edge_data) = self.repository.retrieve_relationship_by_id(db_id)?;
            let sid = self.get_or_retrieve_vertex(db_edge_data.source)?;
            let tid = self.get_or_retrieve_vertex(db_edge_data.target)?;
            let pid = add_edge(self, &db_edge_data, db_id)?;
            self.edges[pid.get_index()?].relationship.as_ref()
        } else {
            let (rid, edata) = self.map_edges[&db_id];
            let index = rid.get_index()?;
            if let None = self.edges[index].relationship {
                let (rel, db_edge_data) = self.repository.retrieve_relationship_by_id(db_id)?;
                self.edges[index].relationship = Some(rel);
            }
            self.edges[index].relationship.as_ref()
        }
    }

}

pub struct InEdges<'a: 'b, 'b> {
    current_edge_index: Option<ProxyRelationshipId>,
    proxy: &'b mut GraphProxy<'a>,
}

impl <'a, 'b> Iterator for InEdges<'a, 'b> {
    type Item = (ProxyRelationshipId, ProxyNodeId, Relationship);

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let ordata = self.proxy.map_edges.get(&edge_index.get_store_id()).copied();
                if let Some(rdata) = ordata {
                    let edges = &mut self.proxy.edges;
                    let curr_edge = edges.get(rdata.0.get_index()?)?;
                    self.current_edge_index = curr_edge.next_inbound_edge;
                    let pid = rdata.0;
                    Some((rdata.0, self.proxy.edges[pid.get_index()?].source, self.proxy.get_relationship_ref(&pid)?.clone()))
                } else {
                    let edge_data = self.proxy.repository.retrieve_edge_data_by_id(edge_index.get_store_id())?;
                    let pid = add_edge(self.proxy, &edge_data, edge_index.get_store_id())?;
                    self.proxy.map_edges.insert(edge_index.get_store_id(), (pid, edge_data));
                    let edges = &mut self.proxy.edges;
                    let curr_edge = edges.get(pid.get_index()?)?;
                    self.current_edge_index = curr_edge.next_inbound_edge;
                    Some((pid, self.proxy.edges[pid.get_index()?].source, self.proxy.get_relationship_ref(&pid)?.clone()))
                }
            }
        }
    }
}

fn add_vertex(vertices: &mut Vec<InnerVertexData<ProxyRelationshipId>>, db_id: u64, vdata: DbVertexData) -> (ProxyNodeId, &InnerVertexData<ProxyRelationshipId>) {
    let index = vertices.len();
    let inbound = vdata.first_inbound_edge.map(ProxyRelationshipId::new_db);
    let outbound = vdata.first_outbound_edge.map(ProxyRelationshipId::new_db);
    let ivdata = InnerVertexData{first_outbound_edge: outbound, first_inbound_edge: inbound, node: None};
    vertices.push(ivdata);
    (ProxyNodeId::new(index, db_id), &vertices[index])
}

fn add_edge(proxy: &mut GraphProxy, db_edge_data: &DbEdgeData, rel_db_id: u64) -> Option<ProxyRelationshipId> {
    let index = proxy.edges.len();
    
    let source_data = proxy.get_or_retrieve_vertex(db_edge_data.source)?;
    let target_data = proxy.get_or_retrieve_vertex(db_edge_data.target)?;
    {
        proxy.edges.push(InnerEdgeData{source: source_data.0, target: target_data.0, relationship: None,
            next_inbound_edge: db_edge_data.next_inbound_edge.map(ProxyRelationshipId::new_db), 
            next_outbound_edge: db_edge_data.next_outbound_edge.map(ProxyRelationshipId::new_db)});
    }
    let pid = ProxyRelationshipId::new(index, rel_db_id);
    {
        let ms = &mut proxy.vertices[source_data.0.get_index()?];
        if ms.first_outbound_edge.is_none() {
            ms.first_outbound_edge = Some(pid);
        }
    }
    {
        let mt = &mut proxy.vertices[target_data.0.get_index()?];
        if mt.first_inbound_edge.is_none() {
            mt.first_inbound_edge = Some(pid);
        }
    }
    Some(pid)
}

pub struct OutEdges<'a: 'b, 'b> {
    current_edge_index: Option<ProxyRelationshipId>,
    proxy: &'b mut GraphProxy<'a>,
}

impl <'a, 'b> Iterator for OutEdges<'a, 'b> {
    type Item = (ProxyRelationshipId, ProxyNodeId, Relationship);

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let ordata = self.proxy.map_edges.get(&edge_index.get_store_id()).copied();
                if let Some(rdata) = ordata {
                    let edges = &mut self.proxy.edges;
                    let curr_edge = edges.get(rdata.0.get_index()?)?;
                    self.current_edge_index = curr_edge.next_outbound_edge;
                    let pid = rdata.0;
                    Some((rdata.0, self.proxy.edges[pid.get_index()?].target, self.proxy.get_relationship_ref(&pid)?.clone()))
                } else {
                    let edge_data = self.proxy.repository.retrieve_edge_data_by_id(edge_index.get_store_id())?;
                    let pid = add_edge(self.proxy, &edge_data, edge_index.get_store_id())?;
                    self.proxy.map_edges.insert(edge_index.get_store_id(), (pid, edge_data));
                    let edges = &mut self.proxy.edges;
                    let curr_edge = edges.get(pid.get_index()?)?;
                    self.current_edge_index = curr_edge.next_outbound_edge;
                    Some((pid, self.proxy.edges[pid.get_index()?].target, self.proxy.get_relationship_ref(&pid)?.clone()))
                }
            }
        }
    }
}

impl <'a> GraphProxy<'a> {
    pub fn out_edges<'b>(&'b mut self, source: &ProxyNodeId) -> Option<OutEdges<'a, 'b>> {
        let pid = self.get_or_retrieve_vertex(source.get_store_id());
        pid.and_then(|(id, v)| {
            let first_outbound_edge = self.vertices[id.get_index()?].first_outbound_edge;
            Some(OutEdges{ proxy: self, current_edge_index: first_outbound_edge })
        })
    }

    pub fn in_edges<'b>(&'b mut self, target: &ProxyNodeId) -> Option<InEdges<'a, 'b>> {
        let pid = self.get_or_retrieve_vertex(target.get_store_id());
        pid.and_then(|(id, v)| {
            let first_inbound_edge = self.vertices[id.get_index()?].first_inbound_edge;
            Some(InEdges{ proxy: self, current_edge_index: first_inbound_edge })
        })
    }
    pub fn in_degree(&'a mut self, node: &ProxyNodeId) -> Option<usize> {
        self.in_edges(node).map(|edges| edges.count())
    }
    pub fn out_degree(&'a mut self, node: &ProxyNodeId) -> Option<usize> {
        self.out_edges(node).map(|edges| edges.count())
    }
}


impl <'a> GrowableGraphTrait<ProxyNodeId, ProxyRelationshipId> for GraphProxy<'a> {
    fn get_source_index(&self, edge_index: &ProxyRelationshipId) -> Option<ProxyNodeId> {
        let pid = self.map_edges[&edge_index.get_store_id()];
        Some(self.edges[pid.0.get_index()?].source)
    }
    fn get_target_index(&self, edge_index: &ProxyRelationshipId) -> Option<ProxyNodeId> {
        let pid = self.map_edges[&edge_index.get_store_id()];
        Some(self.edges[pid.0.get_index()?].target)
    }
    fn nodes_len(&self) -> usize {
        self.retrieved_nodes_ids.len()
    }
    fn edges_len(&self) -> usize {
        self.edges.len()
    }
    
    fn get_nodes_ids(&self) -> Vec<ProxyNodeId> {
        self.retrieved_nodes_ids.clone()
    }


}

fn extract_nodes_labels(pattern: &PropertyGraph) -> Vec<String> {
    let mut res = Vec::new();
    for node in pattern.get_nodes() {
        node.get_labels_ref().iter().for_each(|l| res.push(l.to_owned()));
    }
    res
}

fn retrieve_db_nodes_ids(repository: &mut GraphRepository, labels: &Vec<String>) -> Vec<ProxyNodeId> {
    let db_node_ids = repository.fetch_nodes_ids_with_labels(labels);
    let mut res = Vec::new();
    for id in db_node_ids {
        res.push(ProxyNodeId::new_db(id))
    }
    res
}

impl <'a> GraphProxy<'a> {
    pub fn new(repo: &'a mut GraphRepository, pattern: &PropertyGraph) -> Option<Self> {
        let labels = extract_nodes_labels(pattern);
        let mut ids = retrieve_db_nodes_ids(repo, &labels);
        let labels_set = labels.iter().collect::<HashSet<&String>>();
        for n_index in pattern.get_nodes_ids() {
            let pattern_node = pattern.get_node_ref(&n_index);
            if let Some(nid) = pattern_node.get_id() {
                let node_labels = pattern_node.get_labels_ref().iter().collect::<HashSet<&String>>();
                if labels_set.is_disjoint(&node_labels) {
                    ids.push(ProxyNodeId::new_db(nid));
                }
            }
        }
        for v in pattern.get_nodes() {
            if v.get_labels_ref().is_empty() && v.get_id().is_none() {
                    ids = repo.get_node_ids().as_ref().map(|v| v.iter().map(|nid|ProxyNodeId::new_db(*nid)).collect())?;
                    break;
            }
        }
        Some(GraphProxy{repository: repo,
            retrieved_nodes_ids: ids, vertices: Vec::new(),
            edges: Vec::new(),
            map_vertices: HashMap::new(),
            map_edges: HashMap::new(),
        })
    }

    pub fn new_full(repo: &'a mut GraphRepository) -> Option<Self> {
        let ids = repo.get_node_ids().as_ref().map(|v| v.iter().map(|nid|ProxyNodeId::new_db(*nid)).collect())?;

        Some(GraphProxy{repository: repo, 
            retrieved_nodes_ids: ids, vertices: Vec::new(),
            edges: Vec::new(),
            map_vertices: HashMap::new(),
            map_edges: HashMap::new(),
        })
    }

    fn add_edge(&mut self, rel_db_id: u64) -> Option<ProxyRelationshipId> {
        let db_edge_data = self.repository.retrieve_edge_data_by_id(rel_db_id)?;
        add_edge(self, &db_edge_data, rel_db_id)
    }

    fn get_or_retrieve_vertex(&mut self, db_id: u64) -> Option<(ProxyNodeId, DbVertexData)> {
        if let Entry::Vacant(e) = self.map_vertices.entry(db_id) {
            let vdata = self.repository.retrieve_vertex_data_by_id(db_id); 
            let index = self.vertices.len();           
            let res = vdata.map(|v| (ProxyNodeId::new(index, db_id), v));
            if let Some(v) = res {
                let inbound = v.1.first_inbound_edge.map(ProxyRelationshipId::new_db);
                let outbound = v.1.first_outbound_edge.map(ProxyRelationshipId::new_db);
                let ivdata = InnerVertexData{first_outbound_edge: outbound, first_inbound_edge: inbound, node: None};
                self.vertices.push(ivdata);
                e.insert(v);
            }
            res
        } else {
            self.map_vertices.get(&db_id).copied()
        }
    }

    pub fn get_relationships_ref(&self) -> Vec<Option<&Relationship>> {
        self.edges.iter().map(|e| e.relationship.as_ref()).collect::<Vec<Option<&Relationship>>>()
    }

    pub fn get_edges_with_relationships(&self) -> &Vec<InnerEdgeData<ProxyNodeId, ProxyRelationshipId>> {
        &self.edges
    }

}




#[cfg(test)]
mod test_cache_model {
    use super::*;
    use crate::{model::init::InitContext, test_utils::build_dir_path_and_rm_old};

    fn create_stored_graph(gr: &mut GraphRepository) {
        for _ in 0..10 {
            let node = create_node();
            gr.create_node(&node);
        }
        let ids = gr.get_node_ids().clone().expect("node ids");
        let mut source = ids[0];
        for id in &ids[1..] {
            let rel = create_relationship();
            gr.create_relationship(&rel, source, *id);
            source = *id;
        }
    }

    fn create_node() -> Node {
        let mut node = Node::new();
        node.set_labels(vec!["Person".to_string()]);
        node
    }

    fn create_relationship() -> Relationship {
        let mut rel = Relationship::new();
        rel.set_labels(vec!["IsFriendOf".to_string()]);
        rel
    }

    fn create_pattern() -> PropertyGraph {
        let mut pattern = PropertyGraph::new();
        let n0 = pattern.add_node(create_node());
        let n1 = pattern.add_node(create_node());
        let n2 = pattern.add_node(create_node());
        pattern.add_relationship(create_relationship(), n0, n1);
        pattern.add_relationship(create_relationship(), n1, n2);
        pattern
    }
    #[test]
    fn test_add_prop_graphs() {
        let db_dir = build_dir_path_and_rm_old("graph_proxy_test").expect("error");
        let ctx = InitContext::new(&db_dir).expect("error");
        let mut gr = GraphRepository::new(ctx);
        create_stored_graph(&mut gr);
        let pattern = create_pattern();
        let ids = gr.get_node_ids().clone().expect("ids");
        let mut gp = GraphProxy::new(&mut gr, &pattern).expect("proxy");
        for id in ids {
            let pid = ProxyNodeId::new_db(id);
            for (rel_id, target_id, rel) in gp.out_edges(&pid).expect("out edges") {
                println!("{rel_id:?} {target_id:?}")
            }
        }

    }

}