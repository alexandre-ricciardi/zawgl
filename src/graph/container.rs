use super::*;
use super::traits::*;

pub struct GraphContainer<NODE, RELATIONSHIP> {
    nodes: Vec<NODE>,
    relationships: Vec<RELATIONSHIP>,
    graph: Graph,
}


impl <'g, NODE, RELATIONSHIP> GraphContainerTrait<'g, NodeIndex, EdgeIndex, NODE, RELATIONSHIP> for GraphContainer<NODE, RELATIONSHIP> {

    fn get_node_mut(&mut self, id: &NodeIndex) -> &mut NODE {
        &mut self.nodes[id.get_index()]
    }

    fn get_relationship_mut(&mut self, id: &EdgeIndex) -> &mut RELATIONSHIP {
        &mut self.relationships[id.get_index()]
    }

    fn get_node_ref(&self, id: &NodeIndex) -> &NODE {
        &self.nodes[id.get_index()]
    }

    fn get_relationship_ref(&self, id: &EdgeIndex) -> &RELATIONSHIP {
        &self.relationships[id.get_index()]
    }

}

impl <'g, NODE, RELATIONSHIP> GraphTrait<'g, NodeIndex, EdgeIndex> for GraphContainer<NODE, RELATIONSHIP> {
    type OutIt = OutEdges<'g>;
    type InIt = InEdges<'g>;
    fn out_edges(&'g self, source: &NodeIndex) -> OutEdges {
        self.get_inner_graph().out_edges(source)
    }

    fn in_edges(&'g self, target: &NodeIndex) -> Self::InIt {
        self.get_inner_graph().in_edges(target)
    }
    fn get_source_index(&self, edge_index: &EdgeIndex) -> &NodeIndex {
        &self.get_inner_graph().get_edges()[edge_index.get_index()].source
    }
    fn get_target_index(&self, edge_index: &EdgeIndex) -> &NodeIndex {
        &self.get_inner_graph().get_edges()[edge_index.get_index()].target
    }
    fn nodes_len(&self) -> usize {
        self.nodes.len()
    }
    fn get_nodes_ids(&self) -> Vec<NodeIndex> {
        (0..self.nodes_len()).map(NodeIndex::new).collect()
    }
}

impl <NODE, RELATIONSHIP> GraphContainer<NODE, RELATIONSHIP> {
    pub fn new() -> Self {
        GraphContainer {nodes: Vec::new(), relationships: Vec::new(), graph: Graph::new()}
    }
    pub fn add_node(&mut self, node: NODE) -> NodeIndex {
        self.nodes.push(node);
        self.graph.add_node()
    }

    pub fn add_relationship(&mut self, rel: RELATIONSHIP, source: NodeIndex, target: NodeIndex) -> EdgeIndex {
        self.relationships.push(rel);
        self.graph.add_edge(source, target)
    }
    
    pub fn get_inner_graph(&self) -> &Graph {
        &self.graph
    }

    pub fn get_relationships_and_edges(&self) -> Vec<(&RELATIONSHIP, &EdgeData<NodeIndex, EdgeIndex>)> {
        self.relationships.iter().zip(self.graph.get_edges()).collect::<Vec<(&RELATIONSHIP, &EdgeData<NodeIndex, EdgeIndex>)>>()
    }
    pub fn get_nodes(&self) -> &Vec<NODE> {
        &self.nodes
    }

    
    pub fn successors(&self, source: &NodeIndex) -> Successors {
        self.get_inner_graph().successors(source)
    }
    
    pub fn ancestors(&self, target: &NodeIndex) -> Ancestors {
        self.get_inner_graph().ancestors(target)
    }

}
