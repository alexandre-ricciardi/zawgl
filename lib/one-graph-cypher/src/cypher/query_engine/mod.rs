use super::*;
use super::super::model::*;
use super::parser::*;
use one_graph_core::graph::traits::{GraphContainerTrait};
use one_graph_core::graph::*;
use one_graph_core::model::*;

pub fn process_cypher_query(query: &str) -> Option<Request> {
    let mut lexer = lexer::Lexer::new(query);
    match lexer.get_tokens() {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(tokens);
            let ast = parser::cypher_parser::parse(&mut parser).ok()?;
            let mut visitor = CypherAstVisitor::new();
            parser::walk_ast(&mut visitor, &ast).ok()?;
            visitor.request
        }
        Err(value) => None
    }
}

#[derive(PartialEq)]
enum VisitorState {
    Init,
    DirectiveCreate,
    DirectiveMatch,
    Pattern,
    FunctionCall,
    FunctionArg,
    ReturnItem,
}
#[derive(PartialEq)]
enum VisitorPatternState {
    Init,
    Node,
    RelationshipLR,
    RelationshipRL,
    UndirectedRelationship,
    NodeProperty,
    DirectedRelationshipProperty,
    UndirectedRelationshipProperty,
}

enum IdentifierType {
    Variable,
    Label
}

struct CypherAstVisitor {
    request: Option<Request>,
    curr_node: Option<NodeIndex>,
    curr_directed_relationship: Option<EdgeIndex>,
    curr_both_ways_relationship: Option<(EdgeIndex, EdgeIndex)>,
    curr_property_id: Option<usize>,
    curr_both_ways_property_ids: Option<(usize, usize)>,
    curr_identifier: Option<String>,
    state: VisitorState,
    pattern_state: VisitorPatternState,
    id_type: Option<IdentifierType>,
    curr_property_name: Option<String>,
}

impl CypherAstVisitor {
    fn new() -> Self {
        CypherAstVisitor { request: None, curr_node: None, curr_directed_relationship: None, curr_both_ways_relationship: None,
            curr_property_id: None, state: VisitorState::Init, pattern_state: VisitorPatternState::Init, curr_both_ways_property_ids: None,
            curr_identifier: None, id_type: None, curr_property_name: None }
    }
}

impl CypherAstVisitor {
    
    fn set_property_value(&mut self, property_value: Option<PropertyValue>) {
        if let Some(req) = &mut self.request {
            match self.pattern_state {
                VisitorPatternState::DirectedRelationshipProperty |
                VisitorPatternState::UndirectedRelationshipProperty => {
                    if let Some(rel_id) = self.curr_directed_relationship {
                        let rel = req.pattern.get_relationship_mut(&rel_id);
                        if let (Some(name), Some(value)) = (&self.curr_property_name, property_value) {
                            rel.get_properties_mut().push(Property::new(name.clone(), value))
                        }
                    }
                },
                VisitorPatternState::NodeProperty => {
                    if let Some(node_id) = self.curr_node {
                        let node = req.pattern.get_node_mut(&node_id);
                        if let (Some(name), Some(value)) = (&self.curr_property_name, property_value) {
                            node.get_properties_mut().push(Property::new(name.clone(), value.clone()))
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
impl AstVisitor for CypherAstVisitor {
    fn enter_query(&mut self) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_return(&mut self) -> AstVisitorResult<bool> {
        if let Some(request) = &mut self.request {
            request.return_clause = Some(ReturnClause::new());
        }
        Ok(true)
    }
    fn enter_where(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        if let Some(request) = &mut self.request {
            request.where_clause = Some(WhereClause::new(node.clone_ast()));
        }
        Ok(false)
    }
    fn enter_function(&mut self) -> AstVisitorResult<bool> {
        if let Some(request) = &mut self.request {
            if let Some(_) = &mut request.return_clause {
                self.state = VisitorState::FunctionCall;
            }
        }
        Ok(true)
    }
    fn enter_function_arg(&mut self) -> AstVisitorResult<bool> {
        if self.state == VisitorState::FunctionCall {
            self.state = VisitorState::FunctionArg;
        }
        Ok(true)
    }
    fn enter_item(&mut self) -> AstVisitorResult<bool> {
        self.state = VisitorState::ReturnItem;
        Ok(true)
    }
    fn enter_create(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        self.request = Some(Request::new(Directive::CREATE));
        self.state = VisitorState::DirectiveCreate;
        Ok(true)
    }
    fn enter_match(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        self.request = Some(Request::new(Directive::MATCH));
        self.state = VisitorState::DirectiveMatch;
        Ok(true)
    }
    fn enter_node(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        
        let mut n = Node::new();
        match self.state {
            VisitorState::DirectiveCreate => {
                n.set_status(Status::Create);
            }
            VisitorState::DirectiveMatch => {
                n.set_status(Status::Match);
            },
            _ => {}
        }
        self.curr_node = self.request.as_mut().map(|req| req.pattern.add_node(n));
        self.pattern_state = VisitorPatternState::Node;
        self.state = VisitorState::Pattern;
        Ok(true)
    }
    fn enter_relationship(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        
        let prev_node = self.curr_node;
        let pnode = Node::new();
        self.curr_node = self.request.as_mut().map(|req| req.pattern.add_node(pnode));
        let source_target = prev_node.and_then(|p| self.curr_node.map(|c| (p, c)));
        self.curr_directed_relationship = node.ast_tag.and_then(|ast_tag| {
            match ast_tag {
                AstTag::RelDirectedLR => {
                    self.pattern_state = VisitorPatternState::RelationshipLR;
                    source_target.and_then(|st| self.request.as_mut().map(|req| req.pattern.add_relationship(Relationship::new(), st.0, st.1)))
                },
                AstTag::RelDirectedRL => {
                    self.pattern_state = VisitorPatternState::RelationshipRL;
                    source_target.and_then(|st| self.request.as_mut().map(|req| req.pattern.add_relationship(Relationship::new(), st.1, st.0)))
                },
                _ => None
            }
        });
        self.curr_both_ways_relationship = node.ast_tag.and_then(|ast_tag| {
            match ast_tag {
                AstTag::RelUndirected => {
                    self.pattern_state = VisitorPatternState::UndirectedRelationship;
                    source_target.and_then(|st| self.request.as_mut().map(|req| (req.pattern.add_relationship(Relationship::new(), st.0, st.1), req.pattern.add_relationship(Relationship::new(), st.1, st.0))))
                },
                _ => None
            }
        });
        Ok(true)
    }
    fn enter_property(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        match self.pattern_state {
            VisitorPatternState::Node => self.pattern_state = VisitorPatternState::NodeProperty,
            VisitorPatternState::RelationshipRL |
            VisitorPatternState::RelationshipLR => self.pattern_state = VisitorPatternState::DirectedRelationshipProperty,
            VisitorPatternState::UndirectedRelationship => self.pattern_state = VisitorPatternState::UndirectedRelationshipProperty,
            _ => {}
        }
        if self.pattern_state == VisitorPatternState::DirectedRelationshipProperty {
            if let Some(rel_id) = self.curr_directed_relationship {
                self.curr_property_id = self.request.as_mut().map(|req| {
                    let rel = req.pattern.get_relationship_mut(&rel_id);
                    let pvec = &mut rel.get_properties_mut();
                    rel.get_properties_ref().len()
                })
            }
        }
        if self.pattern_state == VisitorPatternState::UndirectedRelationshipProperty {
            if let Some(both_ways) = self.curr_both_ways_relationship {
                self.curr_both_ways_property_ids = self.request.as_mut().map(|req| {
                    let pattern = &mut req.pattern;
                    let mut sizes: (usize, usize) = (0, 0);
                    {
                        let rel0 = pattern.get_relationship_mut(&both_ways.0);
                        sizes.0 = rel0.get_properties_ref().len();
                    }
                    let rel1 = pattern.get_relationship_mut(&both_ways.1);
                    sizes.1 = rel1.get_properties_ref().len();
                    sizes
                })
            }
        }
        if self.pattern_state == VisitorPatternState::NodeProperty {
            if let Some(node_id) = self.curr_node {
                self.curr_property_id = self.request.as_mut().map(|req| {
                    let pnode = req.pattern.get_node_mut(&node_id);
                    let pvec = pnode.get_properties_mut();
                    pvec.len()
                })
            }
        }
        Ok(true)
        
            
    }


    fn enter_integer_value(&mut self, value: Option<i64>) -> AstVisitorResult<bool> {
        let pv = value.map(|v| PropertyValue::PInteger(v));
        self.set_property_value(pv);
        Ok(true)
    }
    fn enter_float_value(&mut self, value: Option<f64>) -> AstVisitorResult<bool> {
        let pv = value.map(|v| PropertyValue::PFloat(v));
        self.set_property_value(pv);
        Ok(true)
    }
    fn enter_string_value(&mut self, value: Option<&str>) -> AstVisitorResult<bool> {
        let pv = value.map(|sv|PropertyValue::PString(String::from(sv)));
        self.set_property_value(pv);
        Ok(true)
    }
    fn enter_bool_value(&mut self, value: Option<bool>) -> AstVisitorResult<bool> {
        let pv = value.map(|v| PropertyValue::PBool(v));
        self.set_property_value(pv);
        Ok(true)
    }

    fn enter_label(&mut self) -> AstVisitorResult<bool> {
        self.id_type = Some(IdentifierType::Label);
        Ok(true)
    }

    fn enter_variable(&mut self) -> AstVisitorResult<bool> {
        self.id_type = Some(IdentifierType::Variable);
        Ok(true)
    }

    fn enter_identifier(&mut self, key: &str) -> AstVisitorResult<bool> {
        self.curr_identifier = Some(String::from(key));
        if let Some(req) = &mut self.request {
            match self.state {
                VisitorState::Pattern => {
                    match self.pattern_state {
                        VisitorPatternState::Node => {
                            if let Some(node_id) = self.curr_node {
                                let node = req.pattern.get_node_mut(&node_id);
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
                                let rel = req.pattern.get_relationship_mut(&rel_id);
                                
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
                                    let rel = req.pattern.get_relationship_mut(&rel_ids.0);
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
                                let rel = req.pattern.get_relationship_mut(&rel_ids.1);
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
                VisitorState::FunctionCall => {
                    if let Some(req) = &mut self.request {
                        if let Some(ret) = &mut req.return_clause {
                            ret.expressions.push(ReturnExpression::FunctionCall(FunctionCall::new(key)));
                        }
                    }
                },
                VisitorState::FunctionArg => {
                    if let Some(req) = &mut self.request {
                        if let Some(ret) = &mut req.return_clause {
                            if let Some(expr) = ret.expressions.last_mut() {
                                if let ReturnExpression::FunctionCall(func_call) = expr {
                                    func_call.args.push(String::from(key));
                                }
                            }
                        }
                    }
                },
                VisitorState::ReturnItem => {
                    if let Some(req) = &mut self.request {
                        if let Some(ret) = &mut req.return_clause {
                            ret.expressions.push(ReturnExpression::Item(String::from(key)));
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod test_query_engine {
    use super::*;

    #[test]
    fn test_create_0() {
        let request = process_cypher_query("CREATE (n:Person)");
        if let  Some(req) = request {
            let node = req.pattern.get_node_ref(&NodeIndex::new(0));
            assert_eq!(node.get_var(), &Some(String::from("n")));
            assert_eq!(node.get_labels_ref()[0], String::from("Person"));
        } else {
            assert!(false, "no request found");
        }
        
    }

    #[test]
    fn test_create_1() {
        let request = process_cypher_query("CREATE (n:Person:Parent {test: 'Hello', case: 4.99})");
        if let  Some(req) = request {
            let node = req.pattern.get_node_ref(&NodeIndex::new(0));
            assert_eq!(node.get_var(), &Some(String::from("n")));
            assert_eq!(node.get_labels_ref()[0], String::from("Person"));
            assert_eq!(node.get_labels_ref()[1], String::from("Parent"));
            assert_eq!(node.get_properties_ref()[0].get_name(), "test");
            assert_eq!(node.get_properties_ref()[0].get_value(), &PropertyValue::PString(String::from("Hello")));
            assert_eq!(node.get_properties_ref()[1].get_name(), "case");
            //assert_eq!(node.properties[1].value, Some(PropertyValue::PFloat(4.99)));
        } else {
            assert!(false, "no request found");
        }
        
    }

    #[test]
    fn test_create_2() {
        let request = process_cypher_query("CREATE (n:Person:Parent)-[r:FRIEND_OF]->(p:Person)");
        if let  Some(req) = request {
            let node = req.pattern.get_node_ref(&NodeIndex::new(0));
            assert_eq!(node.get_var(), &Some(String::from("n")));
            assert_eq!(node.get_labels_ref()[0], String::from("Person"));
            let rel = req.pattern.get_relationship_ref(&EdgeIndex::new(0));
            assert_eq!(rel.get_var(), &Some(String::from("r")));
            assert_eq!(rel.get_labels_ref()[0], String::from("FRIEND_OF"));
            
            
        } else {
            assert!(false, "no request found");
        }
    }

    #[test]
    fn test_match_and_create() {
        let request = process_cypher_query("MATCH (m:Movie), (a:Actor) CREATE (a)-[r:PLAYED_IN]->(m) RETURN m, a, r");
        if let  Some(req) = request {
            let movie = req.pattern.get_node_ref(&NodeIndex::new(0));
            assert_eq!(movie.get_var(), &Some(String::from("a")));
            assert_eq!(movie.get_labels_ref()[0], String::from("Movie"));
            assert_eq!(movie.get_status(), &Status::Match);
            let actor = req.pattern.get_node_ref(&NodeIndex::new(1));
            assert_eq!(actor.get_var(), &Some(String::from("a")));
            assert_eq!(actor.get_status(), &Status::Match);
            assert_eq!(actor.get_labels_ref()[0], String::from("Actor"));
            let rel = req.pattern.get_relationship_ref(&EdgeIndex::new(0));
            assert_eq!(rel.get_var(), &Some(String::from("r")));
            assert_eq!(rel.get_labels_ref()[0], String::from("PLAYED_IN"));
            assert_eq!(rel.get_status(), &Status::Create);
        } else {
            assert!(false, "no request found");
        }
    }
}