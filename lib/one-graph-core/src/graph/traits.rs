pub trait MemGraphId {
    fn get_index(&self) -> usize;
}

pub trait GraphTrait<NodeId: MemGraphId, EdgeId: MemGraphId> {
    fn get_source_index(&self, edge_index: &EdgeId) -> NodeId;
    fn get_target_index(&self, edge_index: &EdgeId) -> NodeId;
    fn nodes_len(&self) -> usize;
    fn edges_len(&self) -> usize;
    fn get_nodes_ids(&self) -> Vec<NodeId>;
}


pub trait GraphContainerTrait<NID: MemGraphId, EID: MemGraphId, NODE, RELATIONSHIP>: GraphTrait<NID, EID> {
    fn get_node_mut(&mut self, id: &NID) -> &mut NODE;
    fn get_relationship_mut(&mut self, id: &EID) -> &mut RELATIONSHIP;
    fn get_node_ref(&self, id: &NID) -> &NODE;
    fn get_relationship_ref(&self, id: &EID) -> &RELATIONSHIP;
}

pub trait GrowableGraphTrait<NodeId: MemGraphId, EdgeId: MemGraphId> {
    fn get_source_index(&self, edge_index: &EdgeId) -> NodeId;
    fn get_target_index(&self, edge_index: &EdgeId) -> NodeId;
    fn nodes_len(&self) -> usize;
    fn edges_len(&self) -> usize;
    fn get_nodes_ids(&self) -> Vec<NodeId>;
}

pub trait GrowableGraphIteratorTrait<NodeId: MemGraphId, EdgeId: MemGraphId> {
    type OutIt: Iterator<Item=EdgeId>;
    type InIt: Iterator<Item=EdgeId>;
    fn out_edges(&mut self, source: &NodeId) -> Self::OutIt;
    fn in_edges(&mut self, target: &NodeId) -> Self::InIt;
    fn in_degree(&mut self, node: &NodeId) -> usize;
    fn out_degree(&mut self, node: &NodeId) -> usize;
}


pub trait GrowableGraphContainerTrait<NID: MemGraphId, EID: MemGraphId, NODE, RELATIONSHIP>: GrowableGraphTrait<NID, EID> {
    fn get_node_ref(&mut self, id: &NID) -> Option<&NODE>;
    fn get_relationship_ref(&mut self, id: &EID) -> Option<&RELATIONSHIP>;
}
