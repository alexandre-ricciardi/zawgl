// MIT License
//
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

use zawgl_core::graph::*;
use zawgl_core::model::*;
use zawgl_cypher_query_model::ast::AstTag;
use zawgl_cypher_query_model::parameters::Parameters;

use super::states::*;

pub struct PathBuilder {
    curr_node: Option<NodeIndex>,
    curr_directed_relationship: Option<EdgeIndex>,
    curr_both_ways_relationship: Option<(EdgeIndex, EdgeIndex)>,
    pattern_state: VisitorPatternState,
    id_type: Option<IdentifierType>,
    curr_property_name: Option<String>,
    current_path: PropertyGraph,
    params: Option<Parameters>,
}

fn make_relationship(visitor_state: &VisitorState) -> Relationship {
    let mut r = Relationship::new();
    match visitor_state {
        VisitorState::CreatePattern => {
            r.set_status(Status::Create);
        }
        VisitorState::MatchPattern => {
            r.set_status(Status::Match);
        },
        _ => {}
    }
    r
}

fn make_node(visitor_state: &VisitorState) -> Node {
    let mut n = Node::new();
    match visitor_state {
        VisitorState::CreatePattern => {
            n.set_status(Status::Create);
        }
        VisitorState::MatchPattern => {
            n.set_status(Status::Match);
        },
        _ => {}
    }
    n
}

impl PathBuilder {
    pub fn new(params: Option<Parameters>) -> Self {
        PathBuilder {curr_node: None, curr_directed_relationship: None, curr_both_ways_relationship: None,
            pattern_state: VisitorPatternState::Init,
            id_type: None, curr_property_name: None, current_path: PropertyGraph::new(), params: params }
    }

    pub fn get_path_graph(&self) -> &PropertyGraph {
        &self.current_path
    }

    pub fn set_property_value(&mut self, property_value: Option<PropertyValue>) {
        match self.pattern_state {
            VisitorPatternState::DirectedRelationshipProperty |
            VisitorPatternState::UndirectedRelationshipProperty => {
                if let Some(rel_id) = self.curr_directed_relationship {
                    let rel = self.current_path.get_relationship_mut(&rel_id);
                    if let (Some(name), Some(value)) = (&self.curr_property_name, property_value) {
                        rel.get_properties_mut().push(Property::new(name.clone(), value))
                    }
                }
            },
            VisitorPatternState::NodeProperty => {
                if let Some(node_id) = self.curr_node {
                    let node = self.current_path.get_node_mut(&node_id);
                    if let (Some(name), Some(value)) = (&self.curr_property_name, property_value) {
                        node.get_properties_mut().push(Property::new(name.clone(), value.clone()))
                    }
                }
            }
            _ => {}
        }
    }

    pub fn enter_node(&mut self, visitor_state: VisitorState) {
        match self.pattern_state {
            VisitorPatternState::Init => {
                let n = make_node(&visitor_state);
                self.curr_node = Some(self.current_path.add_node(n));
                self.pattern_state = VisitorPatternState::Node;
            },
            VisitorPatternState::RelationshipLR |
            VisitorPatternState::RelationshipRL |
            VisitorPatternState::UndirectedRelationship => {
                self.pattern_state = VisitorPatternState::Node;
            },
            _ => {}
        }
    }

    
    
    pub fn enter_relationship(&mut self, ast_tag: AstTag, visitor_state: VisitorState) {
        
        let prev_node = self.curr_node;
        let pnode = make_node(&visitor_state);
        self.curr_node = Some(self.current_path.add_node(pnode));
        let source_target = prev_node.and_then(|p| self.curr_node.map(|c| (p, c)));

        match ast_tag {
            AstTag::RelDirectedLR => {
                self.pattern_state = VisitorPatternState::RelationshipLR;
                self.curr_directed_relationship = source_target.map(|st| self.current_path.add_relationship(make_relationship(&visitor_state), st.0, st.1))
            }
            AstTag::RelDirectedRL => {
                self.pattern_state = VisitorPatternState::RelationshipRL;
                self.curr_directed_relationship = source_target.map(|st| self.current_path.add_relationship(make_relationship(&visitor_state), st.1, st.0))
            }
            AstTag::RelUndirected => {
                self.pattern_state = VisitorPatternState::UndirectedRelationship;
                self.curr_both_ways_relationship = source_target.map(|st| (self.current_path.add_relationship(make_relationship(&visitor_state), st.0, st.1), self.current_path.add_relationship(Relationship::new(), st.1, st.0)));
            }
            _ => {}
        }
    }

    pub fn enter_property(&mut self) {
        match self.pattern_state {
            VisitorPatternState::Node => self.pattern_state = VisitorPatternState::NodeProperty,
            VisitorPatternState::RelationshipRL |
            VisitorPatternState::RelationshipLR => self.pattern_state = VisitorPatternState::DirectedRelationshipProperty,
            VisitorPatternState::UndirectedRelationship => self.pattern_state = VisitorPatternState::UndirectedRelationshipProperty,
            _ => {}
        }
    }

    pub fn enter_integer_value(&mut self, value: Option<i64>) {
        let pv = value.map(|v| PropertyValue::PInteger(v));
        self.set_property_value(pv);
    }
    pub fn enter_float_value(&mut self, value: Option<f64>) {
        let pv = value.map(|v| PropertyValue::PFloat(v));
        self.set_property_value(pv);
    }
    pub fn enter_string_value(&mut self, value: Option<&str>) {
        let pv = value.map(|sv|PropertyValue::PString(String::from(sv)));
        self.set_property_value(pv);
    }
    pub fn enter_bool_value(&mut self, value: Option<bool>) {
        let pv = value.map(|v| PropertyValue::PBool(v));
        self.set_property_value(pv);
    }
    
    pub fn enter_label(&mut self) {
        self.id_type = Some(IdentifierType::Label);
    }

    pub fn enter_variable(&mut self) {
        self.id_type = Some(IdentifierType::Variable);
    }
    pub fn enter_identifier(&mut self, visitor_state: VisitorState, key: &str) {
        match visitor_state {
            VisitorState::MatchPattern |
            VisitorState::CreatePattern => {
                match self.pattern_state {
                    VisitorPatternState::Node => {
                        if let Some(node_id) = self.curr_node {
                            let node = self.current_path.get_node_mut(&node_id);
                            match self.id_type {
                                Some(IdentifierType::Variable) => {
                                    node.set_var(key);
                                },
                                Some(IdentifierType::Label) => {
                                    node.get_labels_mut().push(String::from(key));
                                },
                                _ => {}
                            } 
                        }
                    },
                    VisitorPatternState::RelationshipRL |
                    VisitorPatternState::RelationshipLR => {
                        if let Some(rel_id) = self.curr_directed_relationship {
                            let rel = self.current_path.get_relationship_mut(&rel_id);
                            
                            match self.id_type {
                                Some(IdentifierType::Variable) => {
                                    rel.set_var(key);
                                },
                                Some(IdentifierType::Label) => {
                                    rel.get_labels_mut().push(String::from(key));
                                },
                                _ => {}
                            } 
                        }
                    },
                    VisitorPatternState::UndirectedRelationship => {
                        if let Some(rel_ids) = self.curr_both_ways_relationship {
                            {
                                let rel = self.current_path.get_relationship_mut(&rel_ids.0);
                                match self.id_type {
                                    Some(IdentifierType::Variable) => {
                                        rel.set_var(key);
                                    },
                                    Some(IdentifierType::Label) => {
                                        rel.get_labels_mut().push(String::from(key));
                                    },
                                    _ => {}
                                } 
                            }
                            let rel = self.current_path.get_relationship_mut(&rel_ids.1);
                            match self.id_type {
                                Some(IdentifierType::Variable) => {
                                    rel.set_var(key);
                                },
                                Some(IdentifierType::Label) => {
                                    rel.get_labels_mut().push(String::from(key));
                                },
                                _ => {}
                            } 
                            
                        }
                    },
                    VisitorPatternState::DirectedRelationshipProperty => {
                        self.curr_property_name = Some(String::from(key));
                    },
                    VisitorPatternState::NodeProperty => {
                        self.curr_property_name = Some(String::from(key));
                    },
                    VisitorPatternState::UndirectedRelationshipProperty => {
                        self.curr_property_name = Some(String::from(key));
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
    
    pub fn enter_parameter(&mut self, name: &str) {
        
    }
}
