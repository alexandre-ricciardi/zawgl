use super::graph::*;

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

pub struct Node {
    pub properties: Vec<Property>,
    pub labels: Vec<String>
}

impl Node {
    pub fn new() -> Self {
        Node {properties: Vec::new(), labels: Vec::new()}
    }
}

pub struct Relationship {
    propeties: Vec<Property>,
    labels: Vec<String>,
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

    pub fn add_node(&mut self, node: Node) -> usize {
        self.nodes.push(node);
        self.graph.add_node()
    }

    pub fn add_relationship(&mut self, rel: Relationship, source: usize, target: usize) -> usize {
        self.relationships.push(rel);
        self.graph.add_edge(source, target)
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