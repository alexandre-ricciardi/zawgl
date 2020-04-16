use super::*;

pub struct GraphContainer<NODE, RELATIONSHIP> {
    nodes: Vec<NODE>,
    relationships: Vec<RELATIONSHIP>,
    graph: Graph,
}

pub trait GraphContainerTrait<NODE, RELATIONSHIP> {
    fn get_node_mut(&mut self, id: NodeIndex) -> &mut NODE;
    fn get_relationship_mut(&mut self, id: EdgeIndex) -> &mut RELATIONSHIP;
    fn get_node_ref(&self, id: NodeIndex) -> &NODE;
    fn get_relationship_ref(&self, id: EdgeIndex) -> &RELATIONSHIP;
}

pub trait GraphTrait {
    fn out_edges(&self, source: &NodeIndex) -> OutEdges;
    fn in_edges(&self, target: &NodeIndex) -> InEdges;
    fn get_source_index(&self, edge_index: &EdgeIndex) -> NodeIndex;
    fn get_target_index(&self, edge_index: &EdgeIndex) -> NodeIndex;

}

impl <NODE, RELATIONSHIP> GraphContainerTrait<NODE, RELATIONSHIP> for GraphContainer<NODE, RELATIONSHIP> {

    fn get_node_mut(&mut self, id: NodeIndex) -> &mut NODE {
        &mut self.nodes[id.0]
    }

    fn get_relationship_mut(&mut self, id: EdgeIndex) -> &mut RELATIONSHIP {
        &mut self.relationships[id.0]
    }

    fn get_node_ref(&self, id: NodeIndex) -> &NODE {
        &self.nodes[id.0]
    }

    fn get_relationship_ref(&self, id: EdgeIndex) -> &RELATIONSHIP {
        &self.relationships[id.0]
    }

}

impl <NODE, RELATIONSHIP> GraphTrait for GraphContainer<NODE, RELATIONSHIP> {
    fn out_edges(&self, source: &NodeIndex) -> OutEdges {
        self.get_inner_graph().out_edges(*source)
    }

    fn in_edges(&self, target: &NodeIndex) -> InEdges {
        self.get_inner_graph().in_edges(*target)
    }
    fn get_source_index(&self, edge_index: &EdgeIndex) -> NodeIndex {
        self.get_inner_graph().get_edges()[edge_index.0].source
    }
    fn get_target_index(&self, edge_index: &EdgeIndex) -> NodeIndex {
        self.get_inner_graph().get_edges()[edge_index.0].target

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

    pub fn get_relationships_and_edges(&self) -> Vec<(&RELATIONSHIP, &EdgeData)> {
        self.relationships.iter().zip(self.graph.get_edges()).collect::<Vec<(&RELATIONSHIP, &EdgeData)>>()
    }
    pub fn get_nodes(&self) -> &Vec<NODE> {
        &self.nodes
    }

    
    pub fn successors(&self, source: &NodeIndex) -> Successors {
        self.get_inner_graph().successors(*source)
    }
    
    pub fn ancestors(&self, target: &NodeIndex) -> Ancestors {
        self.get_inner_graph().ancestors(*target)
    }

}
