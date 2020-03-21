use super::graph::*;

#[derive(Debug, PartialEq, Clone)]
pub enum PropertyValue {
    PString(String),
    PInteger(i64),
    PFloat(f64),
    PBool(bool),
}

pub enum Directive {
    CREATE,
    MATCH,
    DELETE
}

pub struct Property {
    pub name: Option<String>,
    pub value: Option<PropertyValue>,
}

impl Property {
    pub fn new() -> Self {
        Property { name: None, value: None }
    } 
}


pub struct Node {
    pub var: Option<String>,
    pub properties: Vec<Property>,
    pub labels: Vec<String>
}


impl Node {
    pub fn new() -> Self {
        Node {var: None, properties: Vec::new(), labels: Vec::new()}
    }
}

pub struct Relationship {
    pub var: Option<String>,
    pub properties: Vec<Property>,
    pub labels: Vec<String>,
}

impl Relationship {
    pub fn new() -> Self {
        Relationship {var: None, properties: Vec::new(), labels: Vec::new()}
    }
}

pub struct Pattern {
    nodes: Vec<Node>,
    relationships: Vec<Relationship>,
    graph: Graph,
}

impl Pattern {
    pub fn new() -> Self {
        Pattern {nodes: Vec::new(), relationships: Vec::new(), graph: Graph::new()}
    }

    pub fn add_node(&mut self) -> usize {
        let node = Node::new();
        self.nodes.push(node);
        self.graph.add_node()
    }

    pub fn add_relationship(&mut self, source: usize, target: usize) -> usize {
        let rel = Relationship::new();
        self.relationships.push(rel);
        self.graph.add_edge(source, target)
    }

    pub fn get_node_mut(&mut self, id: usize) -> &mut Node {
        &mut self.nodes[id]
    }

    pub fn get_relationship_mut(&mut self, id: usize) -> &mut Relationship {
        &mut self.relationships[id]
    }

    pub fn get_node_ref(&self, id: usize) -> &Node {
        &self.nodes[id]
    }

    pub fn get_relationship_ref(&self, id: usize) -> &Relationship {
        &self.relationships[id]
    }

    pub fn successors(&self, source: usize) -> Successors {
        self.graph.successors(source)
    }
    
    pub fn ancestors(&self, target: usize) -> Ancestors {
        self.graph.ancestors(target)
    }
}

pub struct Request {
    pub pattern: Pattern,
    pub directive: Directive,
}

impl Request {
    pub fn new(directive: Directive) -> Self {
        Request {pattern: Pattern::new(), directive: directive}
    }
}