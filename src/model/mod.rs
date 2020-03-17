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

#[derive(PartialEq, Clone)]
pub struct Property {
    name: String,
    value: PropertyValue,
}

impl Property {
    pub fn new_string(name: &str, value: &str) -> Self {
        Property { name: name.to_owned(), value: PropertyValue::PString(value.to_owned()) }
    }
    pub fn new_float(name: &str, value: f64) -> Self {
        Property { name: name.to_owned(), value: PropertyValue::PFloat(value) }
    }
    pub fn new_integer(name: &str, value: i64) -> Self {
        Property { name: name.to_owned(), value: PropertyValue::PInteger(value) }
    }
    pub fn new_bool(name: &str, value: bool) -> Self {
        Property { name: name.to_owned(), value: PropertyValue::PBool(value) }
    }
}

#[derive(PartialEq, Clone)]
pub struct Node {
    pub id: usize,
    pub properties: Vec<Property>,
    pub labels: Vec<String>
}


impl Node {
    pub fn new(id: usize) -> Self {
        Node {id: id, properties: Vec::new(), labels: Vec::new()}
    }
}
#[derive(PartialEq, Clone)]
pub struct Relationship {
    pub id: usize,
    pub properties: Vec<Property>,
    pub labels: Vec<String>,
}

impl Relationship {
    pub fn new(id: usize) -> Self {
        Relationship {id: id, properties: Vec::new(), labels: Vec::new()}
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

    pub fn add_node(&mut self) -> Node {
        let id = self.graph.add_node();
        let mut node = Node::new(id);
        self.nodes.push(node);
        node.clone()
    }

    pub fn add_relationship(&mut self, source: usize, target: usize) -> Relationship {
        let id = self.graph.add_edge(source, target);
        let mut rel = Relationship::new(id);
        self.relationships.push(rel);
        rel.clone()
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