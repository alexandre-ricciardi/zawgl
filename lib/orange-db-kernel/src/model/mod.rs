use super::graph::*;
pub mod init;
use std::hash::Hash;
use std::hash::Hasher;


#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum PropertyValue {
    PString(String),
    PInteger(i64),
    PFloat(f64),
    PBool(bool),
}

impl Hash for PropertyValue {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        match self {
            PropertyValue::PBool(bval) => {
                bval.hash(state);
            },
            PropertyValue::PString(sval) => {
                sval.hash(state);
            },
            PropertyValue::PInteger(ival) => {
                ival.hash(state);
            },
            PropertyValue::PFloat(fval) => {
                
            }
        }
    }
}


pub enum Directive {
    CREATE,
    MATCH,
    DELETE
}

#[derive(Hash, Eq, PartialEq)]
pub struct Property {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub value: Option<PropertyValue>,
}

impl Property {
    pub fn new() -> Self {
        Property { name: None, value: None, id: None }
    } 
}

#[derive(Hash, Eq, PartialEq)]
pub struct Node {
    pub id: Option<u64>,
    pub var: Option<String>,
    pub properties: Vec<Property>,
    pub labels: Vec<String>
}


impl Node {
    pub fn new() -> Self {
        Node {var: None, properties: Vec::new(), labels: Vec::new(), id:None}
    }
}

pub struct Relationship {
    pub id: Option<u64>,
    pub var: Option<String>,
    pub properties: Vec<Property>,
    pub labels: Vec<String>,
}

impl Relationship {
    pub fn new() -> Self {
        Relationship {var: None, properties: Vec::new(), labels: Vec::new(), id: None}
    }
}

pub type PropertyGraph = container::GraphContainer<Node, Relationship>;

pub struct Request {
    pub pattern: PropertyGraph,
    pub directive: Directive,
}

impl Request {
    pub fn new(directive: Directive) -> Self {
        Request {pattern: PropertyGraph::new(), directive: directive}
    }
}