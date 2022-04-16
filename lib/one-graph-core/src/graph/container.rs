use super::*;
use super::traits::*;

#[derive(Clone)]
pub struct GraphContainer<NODE: Clone, RELATIONSHIP: Clone> {
    graph: Graph<NODE, RELATIONSHIP>,
}


impl <NODE: Clone, RELATIONSHIP: Clone> GraphContainerTrait<NodeIndex, EdgeIndex, NODE, RELATIONSHIP> for GraphContainer<NODE, RELATIONSHIP> {

    fn get_node_mut(&mut self, id: &NodeIndex) -> &mut NODE {
        &mut self.graph.vertices[id.get_index()].node
    }

    fn get_relationship_mut(&mut self, id: &EdgeIndex) -> &mut RELATIONSHIP {
        &mut self.graph.edges[id.get_index()].relationship
    }

    fn get_node_ref(&self, id: &NodeIndex) -> &NODE {
        &self.graph.vertices[id.get_index()].node
    }

    fn get_relationship_ref(&self, id: &EdgeIndex) -> &RELATIONSHIP {
        &self.graph.edges[id.get_index()].relationship
    }

}

impl <NODE: Clone, RELATIONSHIP: Clone> GraphContainer<NODE, RELATIONSHIP> {
    
    fn out_edges(&self, source: &NodeIndex) -> OutEdges<'_, RELATIONSHIP> {
        self.get_inner_graph().out_edges(source)
    }

    fn in_edges(&self, target: &NodeIndex) -> InEdges<'_, RELATIONSHIP> {
        self.get_inner_graph().in_edges(target)
    }
    fn in_degree(&self, node: &NodeIndex) -> usize {
        self.in_edges(node).count()
    }
    fn out_degree(&self, node: &NodeIndex) -> usize {
        self.out_edges(node).count()
    }
}

impl <NODE: Clone, RELATIONSHIP: Clone> GraphTrait<NodeIndex, EdgeIndex> for GraphContainer<NODE, RELATIONSHIP> {

    fn get_source_index(&self, edge_index: &EdgeIndex) -> NodeIndex {
        self.get_inner_graph().get_source_index(edge_index)
    }
    fn get_target_index(&self, edge_index: &EdgeIndex) -> NodeIndex {
        self.get_inner_graph().get_target_index(edge_index)
    }
    fn nodes_len(&self) -> usize {
        self.graph.vertices.len()
    }
    fn edges_len(&self) -> usize {
        self.graph.edges_len()
    }
    fn get_nodes_ids(&self) -> Vec<NodeIndex> {
        (0..self.nodes_len()).map(NodeIndex::new).collect()
    }
}

impl <NODE: Clone, RELATIONSHIP: Clone> GraphContainer<NODE, RELATIONSHIP> {
    pub fn new() -> Self {
        GraphContainer {graph: Graph::new()}
    }
    pub fn add_node(&mut self, node: NODE) -> NodeIndex {
        self.graph.add_vertex(node)
    }

    pub fn add_relationship(&mut self, rel: RELATIONSHIP, source: NodeIndex, target: NodeIndex) -> EdgeIndex {
        self.graph.add_edge(rel, source, target)
    }
    
    pub fn get_inner_graph(&self) -> &Graph<NODE, RELATIONSHIP> {
        &self.graph
    }

    pub fn get_relationships_and_edges(&self) -> &Vec<EdgeData<NodeIndex, EdgeIndex, RELATIONSHIP>> {
        &self.get_edges()
    }

    pub fn get_nodes_with_ids(&self) -> Vec<(&NODE, NodeIndex)> {
        self.graph.vertices.iter().map(|v| &v.node).zip(self.graph.get_nodes_ids()).collect()
    }

    pub fn get_relationships(&self) -> Vec<&RELATIONSHIP> {
        self.graph.edges.iter().map(|e| &e.relationship).collect()
    }

    pub fn get_relationships_mut(&mut self) -> &mut Vec<RELATIONSHIP> {
        &mut self.graph.edges.iter().map(|e| e.relationship).collect()
    }

    pub fn get_edges(&self) -> &Vec<EdgeData<NodeIndex, EdgeIndex, RELATIONSHIP>> {
        &self.graph.edges
    }

    pub fn get_nodes(&self) -> Vec<&NODE> {
        self.graph.vertices.iter().map(|v| &v.node).collect()
    }

    pub fn get_nodes_mut(&mut self) -> Vec<&mut NODE> {
        self.graph.vertices.iter().map(|v| &mut v.node).collect()
    }
}
