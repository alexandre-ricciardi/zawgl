use super::*;

pub struct GraphContainer<NODE, RELATIONSHIP> {
    pub nodes: Vec<NODE>,
    pub relationships: Vec<RELATIONSHIP>,
    pub graph: Graph,
}

impl <NODE, RELATIONSHIP> GraphContainer<NODE, RELATIONSHIP> {
    pub fn new() -> Self {
        GraphContainer {nodes: Vec::new(), relationships: Vec::new(), graph: Graph::new()}
    }

    pub fn add_node(&mut self, node: NODE) -> usize {
        self.nodes.push(node);
        self.graph.add_node()
    }

    pub fn add_relationship(&mut self, rel: RELATIONSHIP, source: usize, target: usize) -> usize {
        self.relationships.push(rel);
        self.graph.add_edge(source, target)
    }

    pub fn get_node_mut(&mut self, id: usize) -> &mut NODE {
        &mut self.nodes[id]
    }

    pub fn get_relationship_mut(&mut self, id: usize) -> &mut RELATIONSHIP {
        &mut self.relationships[id]
    }

    pub fn get_node_ref(&self, id: usize) -> &NODE {
        &self.nodes[id]
    }

    pub fn get_relationship_ref(&self, id: usize) -> &RELATIONSHIP {
        &self.relationships[id]
    }

    pub fn successors(&self, source: usize) -> Successors {
        self.graph.successors(source)
    }
    
    pub fn ancestors(&self, target: usize) -> Ancestors {
        self.graph.ancestors(target)
    }

    pub fn get_nodes(&self) -> &Vec<NODE> {
        &self.nodes
    }
    
    pub fn get_relationships_and_edges(&self) -> Vec<(&RELATIONSHIP, &EdgeData)> {
        self.relationships.iter().zip(self.graph.get_edges()).collect::<Vec<(&RELATIONSHIP, &EdgeData)>>()
    }

    pub fn get_inner_graph(&self) -> &Graph {
        &self.graph
    }
}
