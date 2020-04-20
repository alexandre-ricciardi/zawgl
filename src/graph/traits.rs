pub trait MemGraphId {
    fn get_index(&self) -> usize;
}

pub trait GraphTrait<'g, NodeId: MemGraphId, EdgeId: MemGraphId> {
    type OutIt: Iterator<Item=EdgeId>;
    type InIt: Iterator<Item=EdgeId>;
    fn out_edges(&'g self, source: &NodeId) -> Self::OutIt;
    fn in_edges(&'g self, target: &NodeId) -> Self::InIt;
    fn get_source_index(&self, edge_index: &EdgeId) -> &NodeId;
    fn get_target_index(&self, edge_index: &EdgeId) -> &NodeId;
    fn nodes_len(&self) -> usize;
    fn get_nodes_ids(&self) -> Vec<NodeId>;
}


pub trait GraphContainerTrait<'g, NID: MemGraphId, EID: MemGraphId, NODE, RELATIONSHIP>: GraphTrait<'g, NID, EID> {
    fn get_node_mut(&mut self, id: &NID) -> &mut NODE;
    fn get_relationship_mut(&mut self, id: &EID) -> &mut RELATIONSHIP;
    fn get_node_ref(&self, id: &NID) -> &NODE;
    fn get_relationship_ref(&self, id: &EID) -> &RELATIONSHIP;
}
