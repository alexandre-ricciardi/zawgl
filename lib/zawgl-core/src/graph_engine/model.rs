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

use std::hash::{Hash, Hasher};
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug)]
pub struct ProxyNodeId {
    pub mem_id: usize,
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

impl MemGraphId for ProxyNodeId {
    fn get_index(&self) -> usize {
        self.mem_id
    }
}

impl ProxyNodeId {

    fn new_db(db_id: u64) -> Self {
        ProxyNodeId{mem_id: 0, store_id: db_id}
    }
    fn new(mem_id: usize, db_id: u64) -> Self {
        ProxyNodeId{mem_id, store_id: db_id}
    }
    fn get_store_id(&self) -> u64 {
        self.store_id
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct ProxyRelationshipId {
    mem_id: usize,
    store_id: u64,
}

impl MemGraphId for ProxyRelationshipId {
    fn get_index(&self) -> usize {
        self.mem_id
    }
}

impl ProxyRelationshipId {
    fn new_db(db_id: u64) -> Self {
        ProxyRelationshipId{mem_id: 0, store_id: db_id}
    }
    fn new(mem_id: usize, db_id: u64) -> Self {
        ProxyRelationshipId{mem_id, store_id: db_id}
    }
    fn get_store_id(&self) -> u64 {
        self.store_id
    }
}

#[derive(Copy, Clone)]
pub struct InnerVertexData<EID: MemGraphId> {
    first_outbound_edge: Option<EID>,
    first_inbound_edge: Option<EID>,
}

#[derive(Clone)]
pub struct InnerEdgeData<NID: MemGraphId, EID: MemGraphId> {
    pub source: NID,
    pub target: NID,
    pub next_outbound_edge: Option<EID>,
    pub next_inbound_edge: Option<EID>,
}

pub struct GraphProxy<'a> {
    nodes: Vec<Node>,
    relationships: Vec<Relationship>,
    vertices: Vec<InnerVertexData<ProxyRelationshipId>>,
    edges: Vec<InnerEdgeData<ProxyNodeId, ProxyRelationshipId>>,
    repository: &'a mut GraphRepository,
    retrieved_nodes_ids: Vec<ProxyNodeId>,
    map_vertices: HashMap<u64, (ProxyNodeId, DbVertexData)>,
    map_edges: HashMap<u64, (ProxyRelationshipId, DbEdgeData)>,
}


fn get_relationship_clone(proxy: &mut GraphProxy, id: ProxyRelationshipId) -> Option<Relationship> {
    let ordata = proxy.map_edges.get(&id.get_store_id()).copied();
    let index = {
        let mut retrieve = true;
        let mut edge_exists = false;
        let mut res = 0;
        if let Some(rdata) = ordata {
            edge_exists = true;
            if rdata.0.get_index() < proxy.relationships.len() {
                res = rdata.0.get_index();
                if proxy.relationships[res].get_id().is_some() {
                    retrieve = false;
                }
            }
        }
        if retrieve {
            let rrel = proxy.repository.retrieve_relationship_by_id(id.get_store_id())?;
            get_or_retrieve_vertex_data(proxy, rrel.1.source)?;
            get_or_retrieve_vertex_data(proxy, rrel.1.target)?;
            let pid = proxy.add_relationship(&rrel.0, !edge_exists)?;
            proxy.map_edges.insert(pid.get_store_id(), (pid, rrel.1));
            res = pid.get_index();
        }
        res
    };
    Some(proxy.relationships[index].clone())
}


impl <'a> GrowableGraphContainerTrait<ProxyNodeId, ProxyRelationshipId, Node, Relationship> for GraphProxy<'a> {

    fn get_node_ref(&mut self, id: &ProxyNodeId) -> Option<&Node> {
        let ondata = self.map_vertices.get(&id.get_store_id()).copied();
        let index = {
            let mut retrieve = true;
            let mut vertex_exists = false;
            let mut res = 0;
            if let Some(ndata) = ondata {
                vertex_exists = true;
                if ndata.0.get_index() < self.nodes.len() {
                    res = ndata.0.get_index();
                    if self.nodes[res].get_id().is_some() {
                        retrieve = false;
                    }
                }
            }
            if retrieve {
                let rnode = self.repository.retrieve_node_by_id(id.get_store_id())?;
                let pid = self.add_node(&rnode, !vertex_exists)?;
                self.map_vertices.insert(pid.get_store_id(), (pid, rnode.1));
                res = pid.get_index();
            }
            res
        };
        Some(&self.nodes[index])
    }

    fn get_relationship_ref(&mut self, id: &ProxyRelationshipId) -> Option<&Relationship> {
        let ordata = self.map_edges.get(&id.get_store_id()).copied();
        let index = {
            let mut retrieve = true;
            let mut edge_exists = false;
            let mut res = 0;
            if let Some(rdata) = ordata {
                edge_exists = true;
                if rdata.0.get_index() < self.relationships.len() {
                    res = rdata.0.get_index();
                    if self.relationships[res].get_id().is_some() {
                        retrieve = false;
                    }
                }
            }
            if retrieve {
                let rrel = self.repository.retrieve_relationship_by_id(id.get_store_id())?;
                get_or_retrieve_vertex_data(self, rrel.1.source)?;
                get_or_retrieve_vertex_data(self, rrel.1.target)?;
                let pid = self.add_relationship(&rrel.0, !edge_exists)?;
                self.map_edges.insert(pid.get_store_id(), (pid, rrel.1));
                res = pid.get_index();
            }
            res
        };
        Some(&self.relationships[index])
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
                    let curr_edge = edges.get(rdata.0.get_index())?;
                    self.current_edge_index = curr_edge.next_inbound_edge;
                    let pid = rdata.0;
                    Some((rdata.0, self.proxy.edges[pid.get_index()].source, self.proxy.get_relationship_ref(&pid)?.clone()))
                } else {
                    let edge_data = self.proxy.repository.retrieve_edge_data_by_id(edge_index.get_store_id())?;
                    let pid = add_edge(self.proxy, &edge_data, edge_index.get_store_id())?;
                    self.proxy.map_edges.insert(edge_index.get_store_id(), (pid, edge_data));
                    let edges = &mut self.proxy.edges;
                    let curr_edge = edges.get(pid.get_index())?;
                    self.current_edge_index = curr_edge.next_inbound_edge;
                    Some((pid, self.proxy.edges[pid.get_index()].source, self.proxy.get_relationship_ref(&pid)?.clone()))
                }
            }
        }
    }
}

fn add_vertex(vertices: &mut Vec<InnerVertexData<ProxyRelationshipId>>, db_id: u64, vdata: DbVertexData) -> (ProxyNodeId, InnerVertexData<ProxyRelationshipId>) {
    let index = vertices.len();
    let inbound = vdata.first_inbound_edge.map(ProxyRelationshipId::new_db);
    let outbound = vdata.first_outbound_edge.map(ProxyRelationshipId::new_db);
    let ivdata = InnerVertexData{first_outbound_edge: outbound, first_inbound_edge: inbound};
    vertices.push(ivdata);
    (ProxyNodeId::new(index, db_id), ivdata)
}


fn get_or_retrieve_vertex_data<'a>(proxy: &'a mut GraphProxy, id: u64) -> Option<(ProxyNodeId, InnerVertexData<ProxyRelationshipId>)> {
    let ovdata = proxy.map_vertices.get(&id).copied();
    if let Some(vdata) = ovdata {
        proxy.vertices.get(vdata.0.get_index()).map(|v| (vdata.0, *v))
    } else {
        let vdata = proxy.repository.retrieve_vertex_data_by_id(id)?;
        let pid = add_vertex(&mut proxy.vertices, id, vdata);
        proxy.map_vertices.insert(id, (pid.0, vdata));
        Some(pid)
    }
}

fn add_edge<'a>(proxy: &'a mut GraphProxy, db_edge_data: &DbEdgeData, rel_db_id: u64) -> Option<ProxyRelationshipId> {
    let index = proxy.edges.len();
    
    let source_data = get_or_retrieve_vertex_data(proxy, db_edge_data.source)?;
    let target_data = get_or_retrieve_vertex_data(proxy, db_edge_data.target)?;
    {
        proxy.edges.push(InnerEdgeData{source: source_data.0, target: target_data.0,
            next_inbound_edge: db_edge_data.next_inbound_edge.map(ProxyRelationshipId::new_db), 
            next_outbound_edge: db_edge_data.next_outbound_edge.map(ProxyRelationshipId::new_db)});
    }
    let pid = ProxyRelationshipId::new(index, rel_db_id);
    {
        let ms = &mut proxy.vertices[source_data.0.get_index()];
        if ms.first_outbound_edge.is_none() {
            ms.first_outbound_edge = Some(pid);
        }
    }
    {
        let mt = &mut proxy.vertices[target_data.0.get_index()];
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
                    let curr_edge = edges.get(rdata.0.get_index())?;
                    self.current_edge_index = curr_edge.next_outbound_edge;
                    let pid = rdata.0;
                    Some((rdata.0, self.proxy.edges[pid.get_index()].target, self.proxy.get_relationship_ref(&pid)?.clone()))
                } else {
                    let edge_data = self.proxy.repository.retrieve_edge_data_by_id(edge_index.get_store_id())?;
                    let pid = add_edge(&mut self.proxy, &edge_data, edge_index.get_store_id())?;
                    self.proxy.map_edges.insert(edge_index.get_store_id(), (pid, edge_data));
                    let edges = &mut self.proxy.edges;
                    let curr_edge = edges.get(pid.get_index())?;
                    self.current_edge_index = curr_edge.next_outbound_edge;
                    Some((pid, self.proxy.edges[pid.get_index()].target, self.proxy.get_relationship_ref(&pid)?.clone()))
                }
            }
        }
    }
}

impl <'a> GraphProxy<'a> {
    pub fn out_edges<'b>(&'b mut self, source: &ProxyNodeId) -> OutEdges<'a, 'b> {
        let pid = &self.map_vertices[&source.get_store_id()];
        let first_outbound_edge = self.vertices[pid.0.get_index()].first_outbound_edge;
        OutEdges{ proxy: self, current_edge_index: first_outbound_edge }
    }

    pub fn in_edges<'b>(&'b mut self, target: &ProxyNodeId) -> InEdges<'a, 'b> {
        let pid = &self.map_vertices[&target.get_store_id()];
        let first_inbound_edge = self.vertices[pid.0.get_index()].first_inbound_edge;
        InEdges{ proxy: self, current_edge_index: first_inbound_edge }
    }
    pub fn in_degree(&'a mut self, node: &ProxyNodeId) -> usize {
        self.in_edges(node).count()
    }
    pub fn out_degree(&'a mut self, node: &ProxyNodeId) -> usize {
        self.out_edges(node).count()
    }
}


impl <'a> GrowableGraphTrait<ProxyNodeId, ProxyRelationshipId> for GraphProxy<'a> {
    fn get_source_index(&self, edge_index: &ProxyRelationshipId) -> ProxyNodeId {
        let pid = self.map_edges[&edge_index.get_store_id()];
        self.edges[pid.0.get_index()].source
    }
    fn get_target_index(&self, edge_index: &ProxyRelationshipId) -> ProxyNodeId {
        let pid = self.map_edges[&edge_index.get_store_id()];
        self.edges[pid.0.get_index()].target
    }
    fn nodes_len(&self) -> usize {
        self.retrieved_nodes_ids.len()
    }
    fn edges_len(&self) -> usize {
        self.relationships.len()
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
        Some(GraphProxy{repository: repo, nodes: Vec::new(),
            relationships: Vec::new(),
            retrieved_nodes_ids: ids, vertices: Vec::new(),
            edges: Vec::new(),
            map_vertices: HashMap::new(),
            map_edges: HashMap::new(),
        })
    }

    pub fn new_full(repo: &'a mut GraphRepository) -> Option<Self> {
        let ids = repo.get_node_ids().as_ref().map(|v| v.iter().map(|nid|ProxyNodeId::new_db(*nid)).collect())?;

        Some(GraphProxy{repository: repo, nodes: Vec::new(),
            relationships: Vec::new(),
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

    fn add_vertex(&mut self, db_id: u64, vdata: DbVertexData) -> (ProxyNodeId, InnerVertexData<ProxyRelationshipId>) {
        add_vertex(&mut self.vertices, db_id, vdata)
    }

    fn add_node(&mut self, node: &(Node, DbVertexData), retrieve_vertex: bool) -> Option<ProxyNodeId> {
        let id = node.0.get_id()?;
        let pid = {
            if retrieve_vertex {
                self.add_vertex(id, node.1).0
            } else {
                self.map_vertices[&id].0
            }
        };
        while pid.get_index() > self.nodes.len() {
            self.nodes.push(Node::new());
        }
        self.nodes.insert(pid.get_index(), node.0.clone());
        Some(pid)
    }

    fn add_relationship(&mut self, rel: &Relationship, retrieve_edge: bool) -> Option<ProxyRelationshipId> {
        let id = rel.get_id()?;
        let pid = {
            if retrieve_edge {
                self.add_edge(id)?
            } else {
                self.map_edges[&id].0
            }
        };
        while pid.get_index() > self.relationships.len() {
            self.relationships.push(Relationship::new());
        }
        self.relationships.insert(pid.get_index(), rel.clone());
        Some(pid)
    }

    pub fn get_relationships_ref(&self) -> &Vec<Relationship> {
        &self.relationships
    }

    pub fn get_edges_with_relationships(&self) -> Vec<(InnerEdgeData<ProxyNodeId, ProxyRelationshipId>, Relationship)> {
        self.edges.clone().into_iter().zip(self.relationships.clone()).collect()
    }

}




#[cfg(test)]
mod test_cache_model {
    fn test_add_prop_graphs() {
    }

}