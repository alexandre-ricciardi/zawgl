use super::graph::*;
pub mod init;
use std::hash::Hash;
use std::hash::Hasher;


#[derive(Debug, Clone)]
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
            PropertyValue::PFloat(_) => {
                
            }
        }
    }
}

impl PartialEq for PropertyValue {
    fn eq(&self, other: &Self) -> bool {
        use self::PropertyValue::*;
        match (self, other) {
            (PBool(sval), PBool(oval)) => {
                sval == oval
            },
            (PString(sval), PString(oval))  => {
                sval == oval
            },
            (PInteger(sval), PInteger(oval))  => {
                sval == oval
            },
            (PFloat(_), PFloat(_))  => {
                false
            },
            _ => {false}
        }
    }
}
impl Eq for PropertyValue {}


pub enum Directive {
    CREATE,
    MATCH,
    DELETE
}

#[derive(Hash, Eq, PartialEq, Clone)]
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

#[derive(Hash, Eq, PartialEq, Clone)]
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
#[derive(Hash, Eq, PartialEq, Clone)]
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