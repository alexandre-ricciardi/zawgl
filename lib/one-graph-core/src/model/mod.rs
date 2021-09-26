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

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Property {
    id: Option<u64>,
    name: Option<String>,
    value: Option<PropertyValue>,
}

impl Property {
    pub fn new() -> Self {
        Property {id: None, name: None, value: None}
    }

    pub fn get_id(&self) -> Option<u64> {
        self.id
    }

    pub fn set_id(&mut self, id: Option<u64>) {
        self.id = id;
    }
    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(String::from(name));
    }

    pub fn set_option_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    pub fn get_value(&self) -> &Option<PropertyValue> {
        &self.value
    }

    pub fn set_value(&mut self, val: Option<PropertyValue>) {
        self.value = val;
    }
}


#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Status {
    Match,
    Create,
    Update,
    Empty,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Node {
    id: Option<u64>,
    var: Option<String>,
    properties: Vec<Property>,
    labels: Vec<String>,
    status: Status,
}


impl Node {
    pub fn new() -> Self {
        Node {var: None, properties: Vec::new(), labels: Vec::new(), id:None, status: Status::Empty}
    }

    pub fn get_id(&self) -> Option<u64> {
        self.id
    }

    pub fn set_id(&mut self, id: Option<u64>) {
        self.id = id;
    }

    pub fn get_var(&self) -> &Option<String> {
        &self.var
    }

    pub fn set_var(&mut self, var: &str) {
        self.var = Some(String::from(var));
    }
    
    pub fn set_option_var(&mut self, var: &Option<String>) {
        self.var = var.to_owned();
    }

    pub fn get_properties_ref(&self) -> &Vec<Property> {
        &self.properties
    }

    pub fn get_properties_mut(&mut self) -> &mut Vec<Property> {
        &mut self.properties
    }
    
    pub fn set_properties(&mut self, properties: Vec<Property>) {
        self.properties = properties;
    }
    pub fn get_labels_ref(&self) -> &Vec<String> {
        &self.labels
    }

    pub fn get_labels_mut(&mut self) -> &mut Vec<String> {
        &mut self.labels
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Relationship {
    id: Option<u64>,
    var: Option<String>,
    properties: Vec<Property>,
    labels: Vec<String>,
    status: Status,
}

impl Relationship {
    pub fn new() -> Self {
        Relationship {var: None, properties: Vec::new(), labels: Vec::new(), id: None, status: Status::Empty}
    }
    pub fn get_id(&self) -> Option<u64> {
        self.id
    }

    pub fn set_id(&mut self, id: Option<u64>) {
        self.id = id;
    }

    pub fn get_var(&self) -> &Option<String> {
        &self.var
    }

    pub fn set_var(&mut self, var: &str) {
        self.var = Some(String::from(var));
    }

    pub fn set_option_var(&mut self, var: &Option<String>) {
        self.var = var.to_owned();
    }

    pub fn get_properties_ref(&self) -> &Vec<Property> {
        &self.properties
    }

    pub fn get_properties_mut(&mut self) -> &mut Vec<Property> {
        &mut self.properties
    }

    pub fn set_properties(&mut self, properties: Vec<Property>) {
        self.properties = properties;
    }

    pub fn set_labels(&mut self, labels: Vec<String>) {
        self.labels = labels;
    }

    pub fn get_labels_ref(&self) -> &Vec<String> {
        &self.labels
    }

    pub fn get_labels_mut(&mut self) -> &mut Vec<String> {
        &mut self.labels
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}

pub type PropertyGraph = container::GraphContainer<Node, Relationship>;