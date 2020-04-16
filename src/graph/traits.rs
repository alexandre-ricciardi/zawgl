pub trait GraphTrait<NID, EID, OutIt: Iterator<Item=EID>, InIt: Iterator<Item=EID>> {
    fn out_edges(&self, source: &NID) -> OutIt;
    fn in_edges(&self, target: &NID) -> InIt;
    fn get_source_index(&self, edge_index: &EID) -> NID;
    fn get_target_index(&self, edge_index: &EID) -> NID;

}


pub trait GraphContainerTrait<NID, EID, OutIt: Iterator<Item=EID>, InIt: Iterator<Item=EID>, NODE, RELATIONSHIP>: GraphTrait<NID, EID, OutIt, InIt> {
    fn get_node_mut(&mut self, id: &NID) -> &mut NODE;
    fn get_relationship_mut(&mut self, id: &EID) -> &mut RELATIONSHIP;
    fn get_node_ref(&self, id: &NID) -> &NODE;
    fn get_relationship_ref(&self, id: &EID) -> &RELATIONSHIP;
}
