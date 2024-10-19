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

use super::*;
use super::lexer::LexerError;
use super::parser::error::ParserError;
use serde_json::Value;
use zawgl_core::model::*;
use crate::tx_handler::DatabaseError;

mod path_builder;
mod states;
mod pattern_builder;
pub mod where_clause_filter;

use zawgl_cypher_query_model::{QueryStep, StepType};
use zawgl_cypher_query_model::ast::{AstTagNode, Ast, AstVisitorResult, AstVisitor};
use zawgl_cypher_query_model::model::{Request, EvalScopeClause, WhereClause, EvalScopeExpression, FunctionCall, EvalItem, ValueItem, ItemPropertyName};

use states::*;
use path_builder::*;
use pattern_builder::*;
use std::fmt;

#[derive(Debug, Clone)]
pub enum CypherError {
    ParserError(ParserError),
    LexerError(LexerError),
    SyntaxError,
    RequestError,
    ResponseError,
    TxError(DatabaseError),
    EvalError,
}


impl fmt::Display for CypherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CypherError::ParserError(pe) => f.write_str(&format!("{}", pe)),
            CypherError::LexerError(le) => f.write_str(&format!("{}", le)),
            CypherError::SyntaxError => f.write_str("syntax error"),
            CypherError::RequestError => f.write_str("request error"),
            CypherError::ResponseError => f.write_str("response error"),
            CypherError::TxError(te) => f.write_str(&format!("tx error {}", te)),
            CypherError::EvalError => f.write_str("eval error"),
        }
    }
}


pub fn process_cypher_query(query: &str, params: Option<Value>) -> Result<Request, CypherError> {
    let mut lexer = lexer::Lexer::new(query);
    match lexer.get_tokens() {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(tokens);
            let rast = parser::cypher_parser::parse(&mut parser);
            match rast {
                Ok(ast) => {
                    let mut visitor = CypherAstVisitor::new(params);
                    parser::walk_ast(&mut visitor, &ast).ok().ok_or(CypherError::SyntaxError)?;
                    visitor.request.ok_or(CypherError::SyntaxError)
                },
                Err(parser_err) => {
                    Err(CypherError::ParserError(parser_err))
                },
            }
            
        }
        Err(lerr) => Err(CypherError::LexerError(lerr))
    }
}


struct CypherAstVisitor {
    request: Option<Request>,
    state: Vec<VisitorState>,
    id_type: Option<IdentifierType>,    
    path_builders: Vec<PathBuilder>,
    params: Option<Value>,
    current_identifier: Option<String>,
    item_prop_path: Vec<String>,
    var_scope_filter: Vec<EvalScopeExpression>,
}

impl CypherAstVisitor {
    fn new(params: Option<Value>) -> Self {
        CypherAstVisitor { request: None, state: vec![VisitorState::Init],
            id_type: None, path_builders: Vec::new(), params, current_identifier: None,
            item_prop_path: Vec::new(), var_scope_filter: Vec::new() }
    }
}

impl CypherAstVisitor {
    
    fn current_path_builder(&mut self) -> Option<&mut PathBuilder> {
        self.path_builders.last_mut()
    }

    fn append_path(&mut self) {
        self.path_builders.push(PathBuilder::new(self.params.clone()));
    }
    fn set_visitor_state(&mut self, state: VisitorState) {
        self.state.clear();
        self.state.push(state);
    }
    fn get_visitor_state(&self) -> VisitorState {
        let Some(state) = self.state.last() else {
            panic!("empty visitor state")
        };
        *state
    }
    fn get_first_visitor_state(&self) -> VisitorState {
        let Some(state) = self.state.first() else {
            panic!("empty visitor state")
        };
        *state
    }
    fn push_visitor_state(&mut self, state: VisitorState) {
        self.state.push(state);
    }
    fn pop_visitor_state(&mut self) -> VisitorState {
        self.state.pop();
        *self.state.last().expect("visitor state")
    }
    fn check_var_scope(&mut self, curr_var: &str) -> bool {
        !self.var_scope_filter.is_empty() && self.var_scope_filter.iter().map(|re| {
            match re {
                EvalScopeExpression::FunctionCall(fc) => {
                    fc.alias.clone()
                }
                EvalScopeExpression::Item(i) => {
                    i.alias.clone()
                }
            }
        }).collect::<Vec<Option<String>>>().contains(&Some(curr_var.to_string()))
    }
}

impl AstVisitor for CypherAstVisitor {


    fn enter_query(&mut self) -> AstVisitorResult {
        self.request = Some(Request::new());
        Ok(())
    }
    fn enter_path(&mut self) -> AstVisitorResult {
        self.push_visitor_state(VisitorState::Path);
        self.append_path();
        Ok(())
    }
    fn enter_return(&mut self) -> AstVisitorResult {
        self.set_visitor_state(VisitorState::ReturnClause);
        Ok(())
    }
    fn enter_where(&mut self, node: &AstTagNode) -> AstVisitorResult {
        if let Some(request) = &mut self.request {
            request.steps.push(QueryStep::new_where_clause(WhereClause::new(node.clone_ast(), self.params.clone())));
        }
        Ok(())
    }
    fn enter_function(&mut self) -> AstVisitorResult {
        if let Some(request) = &mut self.request {
            self.push_visitor_state(VisitorState::FunctionCall);
        }
        Ok(())
    }
    fn enter_function_arg(&mut self) -> AstVisitorResult {
        if self.get_visitor_state() == VisitorState::FunctionCall {
            self.push_visitor_state(VisitorState::FunctionArg);
        }
        Ok(())
    }
    fn enter_item(&mut self) -> AstVisitorResult {
        self.push_visitor_state(VisitorState::ReturnItem);
        Ok(())
    }
    fn enter_create(&mut self) -> AstVisitorResult {
        if let Some(rq) = &mut self.request {
            rq.steps.push(QueryStep::new(StepType::CREATE));
        }
        self.set_visitor_state(VisitorState::DirectiveCreate);
        Ok(())
    }
    fn enter_match(&mut self) -> AstVisitorResult {
        if let Some(rq) = &mut self.request {
            rq.steps.push(QueryStep::new(StepType::MATCH));
        }
        self.set_visitor_state(VisitorState::DirectiveMatch);
        Ok(())
    }
    fn enter_node(&mut self) -> AstVisitorResult {
        let state = self.get_first_visitor_state();
        if let Some(pb) = self.current_path_builder() {
            pb.enter_node(state);
        }
        Ok(())
    }
    fn enter_relationship(&mut self, node: &AstTagNode) -> AstVisitorResult {
        let state = self.get_first_visitor_state();
        if let (Some(pb), Some(ast_tag)) = (self.current_path_builder(), node.ast_tag){
            pb.enter_relationship(ast_tag, state)
        }
        Ok(())
    }
    fn enter_property(&mut self) -> AstVisitorResult {
        if let Some(pb) = self.current_path_builder() {
            pb.enter_property();
        }
        Ok(())
    }

    fn enter_integer_value(&mut self, value: Option<i64>) -> AstVisitorResult {
        if let Some(pb) = self.current_path_builder() {
            pb.enter_integer_value(value);
        }
        Ok(())
    }
    fn enter_float_value(&mut self, value: Option<f64>) -> AstVisitorResult {
        if let Some(pb) = self.current_path_builder() {
            pb.enter_float_value(value);
        }
        Ok(())
    }
    fn enter_string_value(&mut self, value: Option<&str>) -> AstVisitorResult {
        if let Some(pb) = self.current_path_builder() {
            pb.enter_string_value(value);
        }
        Ok(())
    }
    fn enter_bool_value(&mut self, value: Option<bool>) -> AstVisitorResult {
        if let Some(pb) = self.current_path_builder() {
            pb.enter_bool_value(value);
        }
        Ok(())
    }

    fn enter_label(&mut self) -> AstVisitorResult {
        if let Some(pb) = self.current_path_builder() {
            pb.enter_label();
        }
        self.id_type = Some(IdentifierType::Label);
        Ok(())
    }

    fn enter_variable(&mut self) -> AstVisitorResult {
        match self.get_visitor_state() {
            VisitorState::DirectiveMatch | VisitorState::DirectiveCreate => {
            },
            _ => {
                if let Some(pb) = self.current_path_builder() {
                    pb.enter_variable();
                }
                self.id_type = Some(IdentifierType::Variable);
            }
        }
        Ok(())
    }

    fn enter_parameter(&mut self, name: &str) -> AstVisitorResult { 
        if let Some(pb) = self.current_path_builder() {
            pb.enter_parameter(name);
        }
        Ok(())
    }

    fn enter_identifier(&mut self, key: &str) -> AstVisitorResult {
        let state = self.get_first_visitor_state();
        match state {
            VisitorState::DirectiveCreate |
            VisitorState::DirectiveMatch => {
                if let Some(pb) = self.current_path_builder() {
                    pb.enter_identifier(state, key);
                }
            }
            VisitorState::ReturnClause | VisitorState::WithClause => {
                let curr_state = self.get_visitor_state();
                match curr_state {
                    VisitorState::FunctionCall => {
                        self.var_scope_filter.push(EvalScopeExpression::FunctionCall(FunctionCall::new(key)));
                    },
                    VisitorState::FunctionArg => {
                        if let Some(expr) = self.var_scope_filter.last_mut() {
                            if let EvalScopeExpression::FunctionCall(func_call) = expr {
                                func_call.args.push(ValueItem::NamedItem(key.to_string()));
                            }
                        }
                    },
                    VisitorState::ReturnItem => {
                        self.var_scope_filter.push(EvalScopeExpression::Item(EvalItem::new_named_item(key)));
                    }
                    VisitorState::ItemPropertyIdentifier => {
                        self.item_prop_path.push(key.to_string());
                    }
                    _ => {}
                }
            }
            
            _ => {}
        }
        Ok(())
    }
    fn exit_create(&mut self) -> AstVisitorResult { 
        if let Some(rq) = &mut self.request {
            let current_step = rq.steps.last_mut();
            if let Some(step) = current_step {
                let paths: Vec<PropertyGraph> = self.path_builders.iter().map(|pb| pb.get_path_graph().clone()).collect();
                step.patterns = merge_paths(&paths);
                self.path_builders.clear();
            }
        }
        Ok(())
    }
    fn exit_match(&mut self) -> AstVisitorResult { 
        if let Some(rq) = &mut self.request {
            let current_step = rq.steps.last_mut();
            if let Some(step) = current_step {
                let paths: &Vec<PropertyGraph> = &self.path_builders.iter().map(|pb| pb.get_path_graph().clone()).collect();
                step.patterns = merge_paths(paths);
                self.path_builders.clear();
            }
        }
        Ok(())
    }   
    fn exit_path(&mut self) -> AstVisitorResult {
        Ok(())
    }
    fn exit_node(&mut self) -> AstVisitorResult { Ok(())}
    fn exit_relationship(&mut self) -> AstVisitorResult { Ok(())}
    fn exit_property(&mut self) -> AstVisitorResult { Ok(())}
    fn exit_integer_value(&mut self) -> AstVisitorResult { Ok(())}
    fn exit_float_value(&mut self) -> AstVisitorResult { Ok(())}
    fn exit_string_value(&mut self) -> AstVisitorResult { Ok(())}
    fn exit_bool_value(&mut self) -> AstVisitorResult { Ok(())}
    fn exit_identifier(&mut self, key: &str) -> AstVisitorResult {
        self.current_identifier = Some(key.to_string());
        Ok(())
    }
    fn exit_variable(&mut self) -> AstVisitorResult { Ok(())}
    fn exit_label(&mut self) -> AstVisitorResult { Ok(())}
    fn exit_query(&mut self) -> AstVisitorResult { Ok(())}
    fn exit_return(&mut self) -> AstVisitorResult { 
        if let Some(req) = &mut self.request {
            req.steps.push(QueryStep::new(StepType::RETURN(EvalScopeClause::new_expression(self.var_scope_filter.clone()))))
        }
        Ok(())
    }
    fn exit_function(&mut self) -> AstVisitorResult { 
        self.set_visitor_state(VisitorState::Empty);    
        Ok(())
    }
    fn exit_function_arg(&mut self) -> AstVisitorResult { Ok(())}
    fn exit_item(&mut self) -> AstVisitorResult { Ok(())}
    fn exit_where(&mut self) -> AstVisitorResult { Ok(())}
    fn exit_parameter(&mut self) -> AstVisitorResult { Ok(())}

    fn enter_equality_operator(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn exit_equality_operator(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn enter_and_operator(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn exit_and_operator(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn enter_or_operator(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn exit_or_operator(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn enter_item_property_identifier(&mut self) -> AstVisitorResult {
        self.push_visitor_state(VisitorState::ItemPropertyIdentifier);
        self.item_prop_path.clear();
        Ok(())
    }

    fn exit_item_property_identifier(&mut self) -> AstVisitorResult {
        let state = self.pop_visitor_state();
        match state {
            VisitorState::FunctionArg => {
                if let Some(expr) = self.var_scope_filter.last_mut() {
                    if let EvalScopeExpression::FunctionCall(func_call) = expr {
                        func_call.args.push(ValueItem::ItemPropertyName(ItemPropertyName::new(&self.item_prop_path[0], &self.item_prop_path[1])));
                    }
                }
            }
            _ => {}
        }
        self.set_visitor_state(VisitorState::Empty);
        Ok(())
    }

    fn enter_gt_operator(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn enter_gte_operator(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn enter_lt_operator(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn enter_lte_operator(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn exit_gt_operator(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn exit_gte_operator(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn exit_lt_operator(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_lte_operator(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn enter_as_operator(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn exit_as_operator(&mut self) -> AstVisitorResult {
        if let Some(alias) = &self.current_identifier {
            if let Some(expr) = self.var_scope_filter.last_mut() {
                match expr {
                    EvalScopeExpression::FunctionCall(fun) => fun.alias = Some(alias.to_string()),
                    EvalScopeExpression::Item(item) => item.alias = Some(alias.to_string()),
                }
            }
        }
        Ok(())
    }
    
    fn enter_with_operator(&mut self) -> AstVisitorResult {
        self.var_scope_filter.clear();
        self.set_visitor_state(VisitorState::WithClause);
        Ok(())
    }
    
    fn exit_with_operator(&mut self) -> AstVisitorResult {
        if let Some(request) = &mut self.request {
            request.steps.push(QueryStep::new(StepType::WITH(EvalScopeClause::new_expression(self.var_scope_filter.clone()))))
        }
        Ok(())
    }
}

#[cfg(test)]
mod test_query_engine {

    use super::*;
    use serde_json::json;
    use zawgl_core::graph::*;
    #[test]
    fn test_create_0() {
        let request = process_cypher_query("CREATE (n:Person)", None);
        if let  Ok(req) = request {
            let node = req.steps[0].patterns[0].get_node_ref(&NodeIndex::new(0));
            assert_eq!(node.get_var(), &Some(String::from("n")));
            assert_eq!(node.get_labels_ref()[0], String::from("Person"));
            assert_eq!(node.get_status(), &Status::Create);
        } else {
            assert!(false, "no request found");
        }
        
    }

    #[test]
    fn test_create_1() {
        let request = process_cypher_query("CREATE (n:Person:Parent {test: 'Hello', case: 4.99})", None);
        if let  Ok(req) = request {
            let node = req.steps[0].patterns[0].get_node_ref(&NodeIndex::new(0));
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
        let request = process_cypher_query("CREATE (n:Person:Parent)-[r:FRIEND_OF]->(p:Person)", None);
        if let  Ok(req) = request {
            let node = req.steps[0].patterns[0].get_node_ref(&NodeIndex::new(0));
            assert_eq!(node.get_var(), &Some(String::from("n")));
            assert_eq!(node.get_labels_ref()[0], String::from("Person"));
            let rel = req.steps[0].patterns[0].get_relationship_ref(&EdgeIndex::new(0));
            assert_eq!(rel.get_var(), &Some(String::from("r")));
            assert_eq!(rel.get_labels_ref()[0], String::from("FRIEND_OF"));
            
            
        } else {
            assert!(false, "no request found");
        }
    }

    #[test]
    fn test_match_and_create() {
        let request = process_cypher_query("MATCH (m:Movie), (a:Actor) CREATE (a)-[r:PLAYED_IN]->(m) RETURN m, a, r", None);
        if let  Ok(req) = request {
            let movie = req.steps[0].patterns[0].get_node_ref(&NodeIndex::new(0));
            assert_eq!(movie.get_var(), &Some(String::from("a")));
            assert_eq!(movie.get_labels_ref()[0], String::from("Actor"));
            assert_eq!(movie.get_status(), &Status::Match);
            let actor = req.steps[0].patterns[1].get_node_ref(&NodeIndex::new(0));
            assert_eq!(actor.get_var(), &Some(String::from("m")));
            assert_eq!(actor.get_status(), &Status::Match);
            assert_eq!(actor.get_labels_ref()[0], String::from("Movie"));
            let rel = req.steps[1].patterns[0].get_relationship_ref(&EdgeIndex::new(0));
            assert_eq!(rel.get_var(), &Some(String::from("r")));
            assert_eq!(rel.get_labels_ref()[0], String::from("PLAYED_IN"));
            assert_eq!(rel.get_status(), &Status::Create);
        } else {
            assert!(false, "no request found");
        }
    }

    
    #[test]
    fn test_match_match() {
        let request = process_cypher_query("MATCH (m:Movie), (a:Actor) MATCH (a)-[r:PLAYED_IN]->(m) RETURN m, a, r", None);
        if let  Ok(req) = request {
            let movie = req.steps[0].patterns[0].get_node_ref(&NodeIndex::new(0));
            assert_eq!(movie.get_var(), &Some(String::from("a")));
            assert_eq!(movie.get_labels_ref()[0], String::from("Actor"));
            assert_eq!(movie.get_status(), &Status::Match);
            let actor = req.steps[0].patterns[1].get_node_ref(&NodeIndex::new(0));
            assert_eq!(actor.get_var(), &Some(String::from("m")));
            assert_eq!(actor.get_status(), &Status::Match);
            assert_eq!(actor.get_labels_ref()[0], String::from("Movie"));
            let rel = req.steps[1].patterns[0].get_relationship_ref(&EdgeIndex::new(0));
            assert_eq!(rel.get_var(), &Some(String::from("r")));
            assert_eq!(rel.get_labels_ref()[0], String::from("PLAYED_IN"));
            assert_eq!(rel.get_status(), &Status::Match);
        } else {
            assert!(false, "no request found");
        }
    }


    #[test]
    fn test_node_id_parameter() {
        let params = json!({"age": 12});
        let request = process_cypher_query("MATCH (m:Movie) WHERE id(m) = $mid RETURN m, a, r", Some(params));
        if let  Ok(req) = request {
            let movie = req.steps[0].patterns[0].get_node_ref(&NodeIndex::new(0));
            assert_eq!(movie.get_var(), &Some(String::from("m")));
            assert_eq!(movie.get_labels_ref()[0], String::from("Movie"));
            assert_eq!(movie.get_status(), &Status::Match);
        } else {
            assert!(false, "no request found");
        }
    }

    
    #[test]
    fn test_gt_parameter() {
        let params = json!({"age": 12});
        let request = process_cypher_query("match (n:Person) where n.age > 40 return n", Some(params));
        if let  Ok(req) = request {
            let movie = req.steps[0].patterns[0].get_node_ref(&NodeIndex::new(0));
            assert_eq!(movie.get_var(), &Some(String::from("n")));
            assert_eq!(movie.get_labels_ref()[0], String::from("Person"));
            assert_eq!(movie.get_status(), &Status::Match);
        } else {
            assert!(false, "no request found");
        }
    }

    #[test]
    fn test_return_sum() {
        let request = process_cypher_query("MATCH (m:Movie), (a:Actor) MATCH (a)-[r:PLAYED_IN]->(m) RETURN sum(a.age) as total", None);
        if let  Ok(req) = request {
            let movie = req.steps[0].patterns[0].get_node_ref(&NodeIndex::new(0));
            assert_eq!(movie.get_var(), &Some(String::from("a")));
            assert_eq!(movie.get_labels_ref()[0], String::from("Actor"));
            assert_eq!(movie.get_status(), &Status::Match);
            let actor = req.steps[0].patterns[1].get_node_ref(&NodeIndex::new(0));
            assert_eq!(actor.get_var(), &Some(String::from("m")));
            assert_eq!(actor.get_status(), &Status::Match);
            assert_eq!(actor.get_labels_ref()[0], String::from("Movie"));
            let rel = req.steps[1].patterns[0].get_relationship_ref(&EdgeIndex::new(0));
            assert_eq!(rel.get_var(), &Some(String::from("r")));
            assert_eq!(rel.get_labels_ref()[0], String::from("PLAYED_IN"));
            assert_eq!(rel.get_status(), &Status::Match);
            if let StepType::RETURN(eval) = &req.steps.last().expect("return clause").step_type {
                if let EvalScopeExpression::FunctionCall(func) = eval.expressions.first().expect("a return function with alias") {
                    assert_eq!(func.alias, Some("total".to_string()));
                    assert_eq!(func.name, "sum".to_string());
                    assert_eq!(func.args.len(), 1);
                }
            }
        } else {
            assert!(false, "no request found");
        }
    }

}