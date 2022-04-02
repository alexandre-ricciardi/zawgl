use super::*;
use super::super::model::*;
use super::parser::*;
use one_graph_core::graph::traits::{GraphContainerTrait};
use one_graph_core::graph::*;
use one_graph_core::model::*;

mod path_builder;
mod states;

use states::*;
use path_builder::*;

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


struct CypherAstVisitor {
    request: Option<Request>,
    curr_identifier: Option<String>,
    state: VisitorState,
    id_type: Option<IdentifierType>,    
    pattern_state: VisitorPatternState,
    current_path_builder: Option<PathBuilder>, 
}

impl CypherAstVisitor {
    fn new() -> Self {
        CypherAstVisitor { request: None, state: VisitorState::Init, pattern_state: VisitorPatternState::Init,
            curr_identifier: None, id_type: None, current_path_builder: None }
    }


}

impl CypherAstVisitor {
    

}
impl AstVisitor for CypherAstVisitor {
    fn enter_query(&mut self) -> AstVisitorResult<bool> {
        self.request = Some(Request::new());
        Ok(true)
    }
    fn enter_pattern(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        match self.state {
            VisitorState::DirectiveMatch => {
                self.state = VisitorState::MatchPattern;
            }
            VisitorState::DirectiveCreate => {
                self.state = VisitorState::CreatePattern;
            }
            _ => {}
        }
        self.current_path_builder = Some(PathBuilder::new(self.state));
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
        
        self.state = VisitorState::DirectiveCreate;
        Ok(true)
    }
    fn enter_match(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        self.state = VisitorState::DirectiveMatch;
        Ok(true)
    }
    fn enter_node(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        if let Some(pb) = &mut self.current_path_builder {
            pb.enter_node();
        }
        Ok(true)
    }
    fn enter_relationship(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        if let (Some(pb), Some(ast_tag)) = (&mut self.current_path_builder, node.ast_tag){
            pb.enter_relationship(ast_tag)
        }
        Ok(true)
    }
    fn enter_property(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        if let Some(pb) = &mut self.current_path_builder {
            pb.enter_property();
        }
        Ok(true)
    }


    fn enter_integer_value(&mut self, value: Option<i64>) -> AstVisitorResult<bool> {
        if let Some(pb) = &mut self.current_path_builder {
            pb.enter_integer_value(value);
        }
        Ok(true)
    }
    fn enter_float_value(&mut self, value: Option<f64>) -> AstVisitorResult<bool> {
        if let Some(pb) = &mut self.current_path_builder {
            pb.enter_float_value(value);
        }
        Ok(true)
    }
    fn enter_string_value(&mut self, value: Option<&str>) -> AstVisitorResult<bool> {
        if let Some(pb) = &mut self.current_path_builder {
            pb.enter_string_value(value);
        }
        Ok(true)
    }
    fn enter_bool_value(&mut self, value: Option<bool>) -> AstVisitorResult<bool> {
        if let Some(pb) = &mut self.current_path_builder {
            pb.enter_bool_value(value);
        }
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
                VisitorState::MatchPattern |
                VisitorState::CreatePattern => {
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
            assert_eq!(movie.get_labels_ref()[0], String::from("Actor"));
            assert_eq!(movie.get_status(), &Status::Match);
            let actor = req.pattern.get_node_ref(&NodeIndex::new(1));
            assert_eq!(actor.get_var(), &Some(String::from("m")));
            assert_eq!(actor.get_status(), &Status::Match);
            assert_eq!(actor.get_labels_ref()[0], String::from("Movie"));
            let rel = req.pattern.get_relationship_ref(&EdgeIndex::new(0));
            assert_eq!(rel.get_var(), &Some(String::from("r")));
            assert_eq!(rel.get_labels_ref()[0], String::from("PLAYED_IN"));
            assert_eq!(rel.get_status(), &Status::Create);
        } else {
            assert!(false, "no request found");
        }
    }
}