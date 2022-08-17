// MIT License
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use self::predicates::NamedPropertyPredicate;

use super::graph::*;
pub mod init;
pub mod predicates;
use std::cmp::Ordering;
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

impl PartialOrd for PropertyValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use self::PropertyValue::*;
        match (self, other) {
            (PBool(sval), PBool(oval)) => {
                Some(sval.cmp(oval))
            },
            (PString(sval), PString(oval))  => {
                Some(sval.cmp(oval))
            },
            (PInteger(sval), PInteger(oval))  => {
                Some(sval.cmp(oval))
            },
            (PFloat(sval), PFloat(oval))  => {
                sval.partial_cmp(oval)
            },
            _ => {None}
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Property {
    id: Option<u64>,
    name: String,
    value: PropertyValue,
}

impl Property {
    pub fn new(name: String, value: PropertyValue) -> Self {
        Property {id: None, name: name, value: value}
    }

    pub fn new_with_id(id: u64, name: String, value: PropertyValue) -> Self {
        Property {id: Some(id), name: name, value: value}
    }

    pub fn get_id(&self) -> Option<u64> {
        self.id
    }

    pub fn set_id(&mut self, id: Option<u64>) {
        self.id = id;
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_value(&self) -> &PropertyValue {
        &self.value
    }
}


#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Status {
    Match,
    Create,
    Update,
    Empty,
}

#[derive(Clone, Debug)]
pub struct Node {
    id: Option<u64>,
    var: Option<String>,
    properties: Vec<Property>,
    labels: Vec<String>,
    status: Status,
    property_predicates: Vec<NamedPropertyPredicate>,
}


impl Node {
    pub fn new() -> Self {
        Node {var: None, properties: Vec::new(), labels: Vec::new(), id:None, status: Status::Empty, property_predicates: Vec::new()}
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

    pub fn set_labels(&mut self, labels: Vec<String>) {
        self.labels = labels;
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

    pub fn add_predicate(&mut self, predicate: NamedPropertyPredicate) {
        self.property_predicates.push(predicate)
    }

    pub fn get_predicates_ref(&self) -> &Vec<NamedPropertyPredicate> {
        &self.property_predicates
    }
}

#[derive(Clone, Debug)]
pub struct Relationship {
    id: Option<u64>,
    var: Option<String>,
    properties: Vec<Property>,
    labels: Vec<String>,
    status: Status,
    property_predicates: Vec<NamedPropertyPredicate>,
}

impl Relationship {
    pub fn new() -> Self {
        Relationship {var: None, properties: Vec::new(), labels: Vec::new(), id: None, status: Status::Empty, property_predicates: Vec::new()}
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
    
    pub fn add_predicate(&mut self, predicate: NamedPropertyPredicate) {
        self.property_predicates.push(predicate)
    }

    pub fn get_predicates_ref(&self) -> &Vec<NamedPropertyPredicate> {
        &self.property_predicates
    }
}

pub type PropertyGraph = container::GraphContainer<Node, Relationship>;