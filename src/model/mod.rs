use super::graph::*;

pub enum Property {
    PString(String),
    PInteger(i64),
    PFloat(f64),
}

pub struct Node {
    propeties: Vec<Property>,

}

pub struct Relationship {

}

pub struct Pattern {
    nodes: Vec<Node>,
    relationships: Vec<Relationship>,
    graph: Graph,
}

impl Pattern {
    pub fn add_node(&mut self, node: Node) -> usize {
        self.nodes.push(node);
        self.graph.add_node()
    }

    pub fn add_relationship(&mut self, rel: Relationship, source: usize, target: usize) {
        self.relationships.push(rel);
        self.graph.add_edge(source, target);
    }
}

pub struct Request {
    pattern: Pattern,

}

impl Request {

}