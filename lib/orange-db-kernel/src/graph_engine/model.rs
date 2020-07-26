use super::super::model::*;
use super::super::graph::traits::*;
use super::super::repository::graph_repository::GraphRepository;

use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct ProxyNodeId {
    mem_id: usize,
    store_id: u64,
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
        ProxyNodeId{mem_id: mem_id, store_id: db_id}
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
    fn new(mem_id: usize, db_id: u64) -> Self {
        ProxyRelationshipId{mem_id: mem_id, store_id: db_id}
    }
    fn get_store_id(&self) -> u64 {
        self.store_id
    }
}

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

pub struct GraphProxy<'r> {
    nodes: Vec<Node>,
    relationships: Vec<Relationship>,
    vertices: Vec<InnerVertexData<ProxyRelationshipId>>,
    edges: Rc<RefCell<Vec<InnerEdgeData<ProxyNodeId, ProxyRelationshipId>>>>,
    repository: &'r mut GraphRepository,
    retrieved_nodes_ids: Vec<ProxyNodeId>,
    map_nodes: HashMap<u64, ProxyNodeId>,
    map_relationships: HashMap<u64, ProxyRelationshipId>,
}


impl <'r> GraphContainerTrait<ProxyNodeId, ProxyRelationshipId, Node, Relationship> for GraphProxy<'r> {

    fn get_node_mut(&mut self, id: &ProxyNodeId) -> &mut Node {
        &mut self.nodes[id.get_index()]
    }

    fn get_relationship_mut(&mut self, id: &ProxyRelationshipId) -> &mut Relationship {
        &mut self.relationships[id.get_index()]
    }

    fn get_node_ref(&self, id: &ProxyNodeId) -> &Node {
        if let Some(pid) = self.map_nodes.get(&id.get_store_id()) {
            &self.nodes[pid.get_index()]
        } else {
            let ornode = self.repository.retrieve_node_by_id(id.get_store_id());
            if let Some(rnode) = ornode {
                let opid = self.add_node(&rnode.0);
                if let Some(pid) = opid {
                    self.map_nodes.insert(pid.get_store_id(), pid);
                    &self.nodes[pid.get_index()]
                }
            }
            
        }
    }

    fn get_relationship_ref(&self, id: &ProxyRelationshipId) -> &Relationship {
        &self.relationships[id.get_index()]
    }

}

pub struct InEdges {
    edges: Rc<RefCell<Vec<InnerEdgeData<ProxyNodeId, ProxyRelationshipId>>>>,
    current_edge_index: Option<ProxyRelationshipId>,
}

impl Iterator for InEdges {
    type Item = ProxyRelationshipId;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.edges.borrow()[edge_index.get_index()];
                let curr_edge_index = self.current_edge_index;
                self.current_edge_index = edge.next_inbound_edge;
                curr_edge_index
            }
        }
    }
}

pub struct OutEdges {
    edges: Rc<RefCell<Vec<InnerEdgeData<ProxyNodeId, ProxyRelationshipId>>>>,
    current_edge_index: Option<ProxyRelationshipId>,
}

impl Iterator for OutEdges {
    type Item = ProxyRelationshipId;

    fn next(&mut self) -> Option<ProxyRelationshipId> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.edges.borrow()[edge_index.get_index()];
                let curr_edge_index = self.current_edge_index;
                self.current_edge_index = edge.next_outbound_edge;
                curr_edge_index
            }
        }
    }
}

impl <'g, 'r> GraphIteratorTrait<ProxyNodeId, ProxyRelationshipId> for &'g mut GraphProxy<'r> {
    type OutIt = OutEdges;
    type InIt = InEdges;
    fn out_edges(&self, source: &ProxyNodeId) -> OutEdges {
        let first_outbound_edge = self.vertices[source.get_index()].first_outbound_edge;
        OutEdges{ edges: self.edges.clone(), current_edge_index: first_outbound_edge }
    }

    fn in_edges(&self, target: &ProxyNodeId) -> Self::InIt {
        let first_inbound_edge = self.vertices[target.get_index()].first_inbound_edge;
        InEdges{ edges: self.edges.clone(), current_edge_index: first_inbound_edge }
    }
}

impl <'r> GraphIteratorTrait<ProxyNodeId, ProxyRelationshipId> for GraphProxy<'r> {
    type OutIt = OutEdges;
    type InIt = InEdges;
    fn out_edges(&self, source: &ProxyNodeId) -> OutEdges {
        let pid = self.map_nodes[&source.get_store_id()];
        let first_outbound_edge = self.vertices[pid.get_index()].first_outbound_edge;
        OutEdges{ edges: self.edges.clone(), current_edge_index: first_outbound_edge }
    }

    fn in_edges(&self, target: &ProxyNodeId) -> Self::InIt {
        let pid = self.map_nodes[&target.get_store_id()];
        let first_inbound_edge = self.vertices[pid.get_index()].first_inbound_edge;
        InEdges{ edges: self.edges.clone(), current_edge_index: first_inbound_edge }
    }
}


impl <'r> GraphTrait<ProxyNodeId, ProxyRelationshipId> for GraphProxy<'r> {
    fn get_source_index(&self, edge_index: &ProxyRelationshipId) -> ProxyNodeId {
        self.edges.borrow()[edge_index.get_index()].source
    }
    fn get_target_index(&self, edge_index: &ProxyRelationshipId) -> ProxyNodeId {
        self.edges.borrow()[edge_index.get_index()].target
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

    fn in_degree(&self, node: &ProxyNodeId) -> usize {
        self.in_edges(node).count()
    }
    fn out_degree(&self, node: &ProxyNodeId) -> usize {
        self.out_edges(node).count()
    }

}

impl <'g> GrowableGraph<ProxyNodeId, ProxyRelationshipId> for GraphProxy<'g> {
    
    fn retrieve_sub_graph_around(&mut self, node_id: &ProxyNodeId) -> Option<()> {
        let pg = self.repository.retrieve_sub_graph_around(node_id.get_store_id())?;
        let mut map_nodes = HashMap::new();
        for node in pg.get_nodes() {
            let id = node.get_id()?;
            if self.map_nodes.contains_key(&id) {
                map_nodes.insert(id, self.map_nodes[&id]);
            } else {
                let pid = self.add_node(node)?;
                self.map_nodes.insert(id, pid);
                map_nodes.insert(id, pid);
            }
            
        }
        for edge in pg.get_edges() {
            let s = pg.get_node_ref(&edge.get_source());
            let t = pg.get_node_ref(&edge.get_target());
            let rel = pg.get_relationship_ref(&edge.id);
            let id_rel = rel.get_id()?;
            if !self.map_relationships.contains_key(&id_rel) {
                let pid = self.add_relationship(map_nodes[&s.get_id()?], map_nodes[&t.get_id()?], rel)?;
                self.map_relationships.insert(id_rel, pid);
            }
        }
        Some(())
    }
}


fn retrieve_db_nodes_ids(repository: &mut GraphRepository, labels: &Vec<String>) -> Vec<ProxyNodeId> {
    let db_node_ids = repository.fetch_nodes_ids_with_labels(labels);
    let mut res = Vec::new();
    for id in db_node_ids {
        res.push(ProxyNodeId::new_db(id))
    }
    res
}

impl <'r> GraphProxy<'r> {
    pub fn new(repo: &'r mut GraphRepository, labels: Vec<String>) -> Self {
        let ids = retrieve_db_nodes_ids(repo, &labels);
        GraphProxy{repository: repo, nodes: Vec::new(),
            relationships: Vec::new(),
            retrieved_nodes_ids: ids, vertices: Vec::new(),
            edges: Rc::new(RefCell::new(Vec::new())),
            map_nodes: HashMap::new(),
            map_relationships: HashMap::new()}
    }

    fn add_edge(&mut self, source: ProxyNodeId, target: ProxyNodeId, rel_db_id: u64) -> ProxyRelationshipId {
        let index = self.edges.borrow().len();
        {
            let source_data = &self.vertices[source.get_index()];
            let target_data = &self.vertices[target.get_index()];
            self.edges.borrow_mut().push(InnerEdgeData{source: source, target: target,
                 next_inbound_edge: target_data.first_inbound_edge, 
                 next_outbound_edge: source_data.first_outbound_edge});
        }
        
        let ms = &mut self.vertices[source.get_index()];
        ms.first_outbound_edge = Some(ProxyRelationshipId::new(index, rel_db_id));
        let mt = &mut self.vertices[target.get_index()];
        mt.first_inbound_edge = Some(ProxyRelationshipId::new(index, rel_db_id));
        ProxyRelationshipId::new(index, rel_db_id)
    }

    fn add_vertex(&mut self, db_id: u64) -> ProxyNodeId {
        let index = self.vertices.len();
        self.vertices.push(InnerVertexData{first_outbound_edge: None, first_inbound_edge: None});
        ProxyNodeId::new(index, db_id)
    }

    fn add_node(&mut self, node: &Node) -> Option<ProxyNodeId> {
        let id = node.get_id()?;
        self.nodes.push(node.clone());
        Some(self.add_vertex(id))
    }

    fn add_relationship(&mut self, source: ProxyNodeId, target: ProxyNodeId, rel: &Relationship) -> Option<ProxyRelationshipId> {
        let id = rel.get_id()?;
        self.relationships.push(rel.clone());
        Some(self.add_edge(source, target, id))
    }
}




#[cfg(test)]
mod test_cache_model {
    use super::*;
    fn test_add_prop_graphs() {
    }

}