use super::super::model::*;
use std::collections::HashMap;
use super::super::graph::traits::{GraphTrait, GraphContainerTrait, MemGraphId};

pub struct ProxyNodeId {
    mem_id: usize,
    store_id: u64,
}

impl MemGraphId for ProxyNodeId {
    fn get_index(&self) -> usize {
        self.mem_id
    }
}

impl ProxyNodeId {
    fn get_store_id(&self) -> u64 {
        self.store_id
    }
}

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
    fn get_store_id(&self) -> u64 {
        self.store_id
    }
}

pub struct InnerNodeData<EID: MemGraphId> {
    first_outbound_edge: Option<EID>,
    first_inbound_edge: Option<EID>,
}

pub struct InnerEdgeData<NID: MemGraphId, EID: MemGraphId> {
    pub source: NID,
    pub target: NID,
    pub next_outbound_edge: Option<EID>,
    pub next_inbound_edge: Option<EID>,
}


pub struct InnerGraph {
    nodes: Vec<InnerNodeData<ProxyRelationshipId>>,
    edges: Vec<InnerEdgeData<ProxyNodeId, ProxyRelationshipId>>,
}

pub struct GraphProxy<NODE, RELATIONSHIP> {
    nodes: Vec<NODE>,
    relationships: Vec<RELATIONSHIP>,
    graph: InnerGraph,
}


impl <'g, NODE, RELATIONSHIP> GraphContainerTrait<'g, ProxyNodeId, ProxyRelationshipId, NODE, RELATIONSHIP> for GraphProxy<NODE, RELATIONSHIP> {

    fn get_node_mut(&mut self, id: &ProxyNodeId) -> &mut NODE {
        &mut self.nodes[id.get_index()]
    }

    fn get_relationship_mut(&mut self, id: &ProxyRelationshipId) -> &mut RELATIONSHIP {
        &mut self.relationships[id.get_index()]
    }

    fn get_node_ref(&self, id: &ProxyNodeId) -> &NODE {
        &self.nodes[id.get_index()]
    }

    fn get_relationship_ref(&self, id: &ProxyRelationshipId) -> &RELATIONSHIP {
        &self.relationships[id.get_index()]
    }

}

pub struct InEdges<'g> {
    graph: &'g InnerGraph,
    current_edge_index: Option<ProxyRelationshipId>,
}

impl <'graph> Iterator for InEdges<'graph> {
    type Item = ProxyRelationshipId;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index.get_index()];
                let curr_edge_index = self.current_edge_index;
                self.current_edge_index = edge.next_inbound_edge;
                curr_edge_index
            }
        }
    }
}

pub struct OutEdges<'g> {
    graph: &'g InnerGraph,
    current_edge_index: Option<ProxyRelationshipId>,
}

impl <'g> Iterator for OutEdges<'g> {
    type Item = ProxyRelationshipId;

    fn next(&mut self) -> Option<ProxyRelationshipId> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index.get_index()];
                let curr_edge_index = self.current_edge_index;
                self.current_edge_index = edge.next_outbound_edge;
                curr_edge_index
            }
        }
    }
}


impl <'g> GraphTrait<'g, ProxyNodeId, ProxyRelationshipId> for InnerGraph {
    type OutIt = OutEdges<'g>;
    type InIt = InEdges<'g>;
    fn out_edges(&self, source: &ProxyNodeId) -> OutEdges {
        let first_outbound_edge = self.nodes[source.get_index()].first_outbound_edge;
        OutEdges{ graph: self, current_edge_index: first_outbound_edge }
    }

    fn in_edges(&self, target: &ProxyNodeId) -> InEdges {
        let first_inbound_edge = self.nodes[target.get_index()].first_inbound_edge;
        InEdges{ graph: self, current_edge_index: first_inbound_edge }
    }

    fn get_source_index(&self, edge_index: &ProxyRelationshipId) -> &ProxyNodeId {
        &self.edges[edge_index.get_index()].source
    }
    fn get_target_index(&self, edge_index: &ProxyRelationshipId) -> &ProxyNodeId {
        &self.edges[edge_index.get_index()].target
    }

    fn nodes_len(&self) -> usize {
        self.nodes.len()
    }

    fn edges_len(&self) -> usize {
        self.edges.len()
    }
 
    fn get_nodes_ids(&self) -> Vec<ProxyNodeId> {
        Vec::new()//(0..self.nodes_len()).map(ProxyNodeId::new).collect()
    }
    
    fn in_degree(&self, node: &ProxyNodeId) -> usize {
        self.in_edges(node).count()
    }
    fn out_degree(&self, node: &ProxyNodeId) -> usize {
        self.out_edges(node).count()
    }
}

impl <'g, NODE, RELATIONSHIP> GraphTrait<'g, ProxyNodeId, ProxyRelationshipId> for GraphProxy<NODE, RELATIONSHIP> {
    type OutIt = OutEdges<'g>;
    type InIt = InEdges<'g>;
    fn out_edges(&'g self, source: &ProxyNodeId) -> OutEdges {
        self.graph.out_edges(source)
    }

    fn in_edges(&'g self, target: &ProxyNodeId) -> Self::InIt {
        self.graph.in_edges(target)
    }
    fn get_source_index(&self, edge_index: &ProxyRelationshipId) -> &ProxyNodeId {
        &self.graph.edges[edge_index.get_index()].source
    }
    fn get_target_index(&self, edge_index: &ProxyRelationshipId) -> &ProxyNodeId {
        &self.graph.edges[edge_index.get_index()].target
    }
    fn nodes_len(&self) -> usize {
        self.nodes.len()
    }
    fn edges_len(&self) -> usize {
        self.relationships.len()
    }
    fn get_nodes_ids(&self) -> Vec<ProxyNodeId> {
        self.graph.get_nodes_ids()
    }
    fn in_degree(&self, node: &ProxyNodeId) -> usize {
        self.in_edges(node).count()
    }
    fn out_degree(&self, node: &ProxyNodeId) -> usize {
        self.out_edges(node).count()
    }
}




#[cfg(test)]
mod test_cache_model {
    use super::*;
    fn test_add_prop_graphs() {
    }

}