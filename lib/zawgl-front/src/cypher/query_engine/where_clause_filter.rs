use serde_json::Value;
use zawgl_core::model::{PropertyGraph, PropertyValue};
use zawgl_cypher_query_model::ast::AstVisitor;
use zawgl_cypher_query_model::ast::{AstTagNode, AstVisitorResult, AstVisitorError};
use zawgl_cypher_query_model::properties::convert_json_value;

use super::states::VisitorState;

pub struct WhereClauseAstVisitor<'a> {
    graph: &'a PropertyGraph,
    params: Option<Value>,
    state: VisitorState,
    function_name: Option<String>,
    function_args: Vec<String>,
    item_prop_path: Vec<String>,
    pub eval_stack: Vec<PropertyValue>,
}

impl <'a> WhereClauseAstVisitor<'a> {
    pub fn new(graph: &'a PropertyGraph, params: Option<Value>) -> Self {
        WhereClauseAstVisitor{graph, params, state: VisitorState::Init, function_name: None,
            function_args: vec![], eval_stack: vec![], item_prop_path: vec![]}
    }
}

impl <'a> AstVisitor for WhereClauseAstVisitor<'a> {
    fn enter_create(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn enter_match(&mut self) -> AstVisitorResult {
        todo!()
    }


    fn enter_optional_match(&mut self) -> AstVisitorResult {
        todo!()
    }
    fn enter_path(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn enter_node(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn enter_relationship(&mut self, _node: &AstTagNode) -> AstVisitorResult {
        todo!()
    }

    fn enter_property(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn enter_integer_value(&mut self, value: Option<i64>) -> AstVisitorResult {
        if let Some(v) = value {
            self.eval_stack.push(PropertyValue::PInteger(v));
        }
        Ok(())
    }

    fn enter_float_value(&mut self, value: Option<f64>) -> AstVisitorResult {
        if let Some(v) = value {
            self.eval_stack.push(PropertyValue::PFloat(v));
        }
        Ok(())
    }

    fn enter_string_value(&mut self, value: Option<&str>) -> AstVisitorResult {
        if let Some(v) = value {
            self.eval_stack.push(PropertyValue::PString(v.to_string()));
        }
        Ok(())
    }

    fn enter_bool_value(&mut self, value: Option<bool>) -> AstVisitorResult {
        if let Some(v) = value {
            self.eval_stack.push(PropertyValue::PBool(v));
        }
        Ok(())
    }

    fn enter_identifier(&mut self, key: &str) -> AstVisitorResult {
        match self.state {
            VisitorState::FunctionCall => self.function_name = Some(key.to_string()),
            VisitorState::FunctionArg => self.function_args.push(key.to_string()),
            VisitorState::ItemPropertyIdentifier => self.item_prop_path.push(key.to_string()),
            _ => {}
        }
        Ok(())
    }

    fn enter_variable(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn enter_label(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn enter_query(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn enter_return(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn enter_function(&mut self) -> AstVisitorResult {
        self.state = VisitorState::FunctionCall;
        self.function_args = Vec::new();
        Ok(())
    }

    fn enter_function_arg(&mut self) -> AstVisitorResult {
        self.state = VisitorState::FunctionArg;
        Ok(())
    }

    fn enter_item(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn enter_where(&mut self, _node: &AstTagNode) -> AstVisitorResult {
        Ok(())
    }

    fn enter_parameter(&mut self, name: &str) -> AstVisitorResult {
        let pname = &name[1..];
        if let Some(value) = self.params.as_ref()
        .and_then(|p|
            p.get(pname)) {
                self.eval_stack.push(convert_json_value(value.clone())?);
            Ok(())
        } else {
            Err(AstVisitorError::SyntaxError)
        }
    }

    fn exit_create(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_match(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_path(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_node(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_relationship(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_property(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_integer_value(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_float_value(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_string_value(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_bool_value(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_identifier(&mut self, _key: &str) -> AstVisitorResult {
        Ok(())
    }

    fn exit_variable(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_label(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_query(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn exit_return(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_function(&mut self) -> AstVisitorResult {
        if let Some(fname) = &self.function_name {
            match fname.as_str() {
                "id" => {
                    let Some(id_val) = self.function_args.first().and_then(|item_name| {
                        for n in self.graph.get_nodes() {
                            if n.get_var().as_deref() == Some(item_name) {
                                return n.get_id();
                            }
                        }
                        None
                    }) else {
                        return Err(AstVisitorError::SyntaxError);
                    };
                    self.eval_stack.push(PropertyValue::PUInteger(id_val));
                },
                _ => {}
            };
        }
        Ok(())
    }

    fn exit_function_arg(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn exit_item(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_where(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn exit_parameter(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn enter_equality_operator(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn exit_equality_operator(&mut self) -> AstVisitorResult {
        let ov0 = self.eval_stack.pop();
        let ov1 = self.eval_stack.pop();
        if let (Some(v0), Some(v1)) = &(ov0, ov1) {
            match (v0, v1) {
                (PropertyValue::PInteger(i0), PropertyValue::PUInteger(u1)) => {
                    if *i0 >= 0 {
                        self.eval_stack.push(PropertyValue::PBool(*i0 as u64 == *u1));
                    } else {
                        self.eval_stack.push(PropertyValue::PBool(false));
                    }
                },
                (PropertyValue::PUInteger(u0), PropertyValue::PInteger(i1)) => {
                    if *i1 >= 0 {
                        self.eval_stack.push(PropertyValue::PBool(*i1 as u64 == *u0));
                    } else {
                        self.eval_stack.push(PropertyValue::PBool(false));
                    }
                },
                _ => {self.eval_stack.push(PropertyValue::PBool(v0 == v1));}
            }
            
        }
        Ok(())
    }

    fn enter_and_operator(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn exit_and_operator(&mut self) -> AstVisitorResult {
        let ov0 = self.eval_stack.pop();
        let ov1 = self.eval_stack.pop();
        if let (Some(v0), Some(v1)) = &(ov0, ov1) {
            match (v0, v1) {
                (PropertyValue::PBool(b0), PropertyValue::PBool(b1)) => {
                    self.eval_stack.push(PropertyValue::PBool(*b0 && *b1));
                }
                _ => return Err(AstVisitorError::SyntaxError),
            }
        } else {
            return Err(AstVisitorError::SyntaxError);
        }
        Ok(())
    }

    fn enter_or_operator(&mut self) -> AstVisitorResult {
        Ok(())
    }

    fn exit_or_operator(&mut self) -> AstVisitorResult {
        let ov0 = self.eval_stack.pop();
        let ov1 = self.eval_stack.pop();
        if let (Some(v0), Some(v1)) = &(ov0, ov1) {
            match (v0, v1) {
                (PropertyValue::PBool(b0), PropertyValue::PBool(b1)) => {
                    self.eval_stack.push(PropertyValue::PBool(*b0 || *b1));
                }
                _ => return Err(AstVisitorError::SyntaxError),
            }
        } else {
            return Err(AstVisitorError::SyntaxError);
        }
        Ok(())
    }

    fn enter_item_property_identifier(&mut self) -> AstVisitorResult {
        self.state = VisitorState::ItemPropertyIdentifier;
        Ok(())
    }

    fn exit_item_property_identifier(&mut self) -> AstVisitorResult {
        let item_name = &self.item_prop_path[0];
        let prop_name = &self.item_prop_path[1];
        for n in self.graph.get_nodes() {
            if n.get_var().as_deref() == Some(item_name) {
                for prop in n.get_properties_ref() {
                    if prop.get_name() == prop_name {
                        self.eval_stack.push(prop.get_value().clone());
                    }
                }
            }
        }
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
        let ov1 = self.eval_stack.pop();
        let ov0 = self.eval_stack.pop();
        if let (Some(v0), Some(v1)) = &(ov0, ov1) {
            match (v0, v1) {
                (PropertyValue::PInteger(i0), PropertyValue::PUInteger(u1)) => {
                    if *i0 >= 0 {
                        self.eval_stack.push(PropertyValue::PBool(*i0 as u64 > *u1));
                    } else {
                        self.eval_stack.push(PropertyValue::PBool(false));
                    }
                },
                (PropertyValue::PUInteger(u0), PropertyValue::PInteger(i1)) => {
                    if *i1 >= 0 {
                        self.eval_stack.push(PropertyValue::PBool((*i1 as u64) < *u0));
                    } else {
                        self.eval_stack.push(PropertyValue::PBool(false));
                    }
                },
                _ => {self.eval_stack.push(PropertyValue::PBool(v0 > v1));}
            }
            
        }
        Ok(())
    }

    fn exit_gte_operator(&mut self) -> AstVisitorResult {
        let ov1 = self.eval_stack.pop();
        let ov0 = self.eval_stack.pop();
        if let (Some(v0), Some(v1)) = &(ov0, ov1) {
            match (v0, v1) {
                (PropertyValue::PInteger(i0), PropertyValue::PUInteger(u1)) => {
                    if *i0 >= 0 {
                        self.eval_stack.push(PropertyValue::PBool(*i0 as u64 >= *u1));
                    } else {
                        self.eval_stack.push(PropertyValue::PBool(false));
                    }
                },
                (PropertyValue::PUInteger(u0), PropertyValue::PInteger(i1)) => {
                    if *i1 >= 0 {
                        self.eval_stack.push(PropertyValue::PBool(*i1 as u64 >= *u0));
                    } else {
                        self.eval_stack.push(PropertyValue::PBool(false));
                    }
                },
                _ => {self.eval_stack.push(PropertyValue::PBool(v0 >= v1));}
            }
            
        }
        Ok(())
    }

    fn exit_lt_operator(&mut self) -> AstVisitorResult {
        let ov1 = self.eval_stack.pop();
        let ov0 = self.eval_stack.pop();
        if let (Some(v0), Some(v1)) = &(ov0, ov1) {
            match (v0, v1) {
                (PropertyValue::PInteger(i0), PropertyValue::PUInteger(u1)) => {
                    if *i0 >= 0 {
                        self.eval_stack.push(PropertyValue::PBool((*i0 as u64 ) < *u1));
                    } else {
                        self.eval_stack.push(PropertyValue::PBool(false));
                    }
                },
                (PropertyValue::PUInteger(u0), PropertyValue::PInteger(i1)) => {
                    if *i1 >= 0 {
                        self.eval_stack.push(PropertyValue::PBool(*i1 as u64 > *u0));
                    } else {
                        self.eval_stack.push(PropertyValue::PBool(false));
                    }
                },
                _ => {self.eval_stack.push(PropertyValue::PBool(v0 < v1));}
            }
            
        }
        Ok(())
    }

    fn exit_lte_operator(&mut self) -> AstVisitorResult {
        let ov1 = self.eval_stack.pop();
        let ov0 = self.eval_stack.pop();
        if let (Some(v0), Some(v1)) = &(ov0, ov1) {
            match (v0, v1) {
                (PropertyValue::PInteger(i0), PropertyValue::PUInteger(u1)) => {
                    if *i0 >= 0 {
                        self.eval_stack.push(PropertyValue::PBool(*i0 as u64 <= *u1));
                    } else {
                        self.eval_stack.push(PropertyValue::PBool(false));
                    }
                },
                (PropertyValue::PUInteger(u0), PropertyValue::PInteger(i1)) => {
                    if *i1 >= 0 {
                        self.eval_stack.push(PropertyValue::PBool(*i1 as u64 <= *u0));
                    } else {
                        self.eval_stack.push(PropertyValue::PBool(false));
                    }
                },
                _ => {self.eval_stack.push(PropertyValue::PBool(v0 <= v1));}
            }
            
        }
        Ok(())
    }

    fn enter_as_operator(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_as_operator(&mut self) -> AstVisitorResult {
        todo!()
    }
    
    fn enter_with_operator(&mut self) -> AstVisitorResult {
        todo!()
    }
    
    fn exit_with_operator(&mut self) -> AstVisitorResult {
        todo!()
    }
    
    fn exit_optional_match(&mut self) -> AstVisitorResult {
        todo!()
    }
}

#[cfg(test)]
mod test_where_clause {
    use serde_json::json;
    use zawgl_core::model::{PropertyGraph, Node, Property};
    use zawgl_cypher_query_model::ast::{AstTag, Ast};

    use crate::cypher::{lexer, parser, parser::where_clause_parser_delegate::parse_where_clause};

    use super::*;
    #[test]
    fn simple_test() {
        let mut g = PropertyGraph::new();
        let mut n0 = Node::new();
        n0.set_id(Some(12));
        n0.set_var("a");
        let mut n1 = Node::new();
        n1.set_var("b");
        g.add_node(n0);
        g.add_node(n1);

        let where_clause = "where id(a) = 12";

        let mut lexer = lexer::Lexer::new(where_clause);
        match lexer.get_tokens() {
            Ok(tokens) => {
                let mut parser = parser::Parser::new(tokens);
                let mut ast = Box::new(AstTagNode::new_tag(AstTag::Query));
                parse_where_clause(&mut parser, &mut ast).expect("where clause ast");
                let mut visitor = WhereClauseAstVisitor::new(&g, None);
                parser::walk_ast(&mut visitor, &(ast as Box<dyn Ast>)).expect("walk");
                assert_eq!(visitor.eval_stack.pop(), Some(PropertyValue::PBool(true)));
            }
            Err(_value) => {}
        }

    }

    #[test]
    fn parameters_test() {
        let mut g = PropertyGraph::new();
        let mut n0 = Node::new();
        n0.set_id(Some(12));
        n0.set_var("a");
        let mut n1 = Node::new();
        n1.set_var("b");
        n1.set_id(Some(15));
        g.add_node(n0);
        g.add_node(n1);

        let where_clause = "where id(a) = $aid and id(b) = $bid";
        let params = json!({
            "aid": 12,
            "bid": 15
        });
        let mut lexer = lexer::Lexer::new(where_clause);
        match lexer.get_tokens() {
            Ok(tokens) => {
                let mut parser = parser::Parser::new(tokens);
                let mut ast = Box::new(AstTagNode::new_tag(AstTag::Query));
                parse_where_clause(&mut parser, &mut ast).expect("where clause ast");
                let mut visitor = WhereClauseAstVisitor::new(&g, Some(params));
                parser::walk_ast(&mut visitor, &(ast as Box<dyn Ast>)).expect("walk");
                assert_eq!(visitor.eval_stack.pop(), Some(PropertyValue::PBool(true)));
            }
            Err(_value) => {}
        }

    }


    
    #[test]
    fn parameters_or_op_test() {
        let mut g = PropertyGraph::new();
        let mut n0 = Node::new();
        n0.set_id(Some(12));
        n0.set_var("a");
        let mut n1 = Node::new();
        n1.set_var("b");
        n1.set_id(Some(15));
        g.add_node(n0);
        g.add_node(n1);

        let where_clause = "where id(a) = $aid or id(b) = $bid";
        let params = json!({
            "aid": 12,
            "bid": 16
        });
        let mut lexer = lexer::Lexer::new(where_clause);
        match lexer.get_tokens() {
            Ok(tokens) => {
                let mut parser = parser::Parser::new(tokens);
                let mut ast = Box::new(AstTagNode::new_tag(AstTag::Query));
                parse_where_clause(&mut parser, &mut ast).expect("where clause ast");
                let mut visitor = WhereClauseAstVisitor::new(&g, Some(params));
                parser::walk_ast(&mut visitor, &(ast as Box<dyn Ast>)).expect("walk");
                assert_eq!(visitor.eval_stack.pop(), Some(PropertyValue::PBool(true)));
            }
            Err(_value) => {}
        }

    }

    #[test]
    fn parameters_or_op_fail_test() {
        let mut g = PropertyGraph::new();
        let mut n0 = Node::new();
        n0.set_id(Some(12));
        n0.set_var("a");
        let mut n1 = Node::new();
        n1.set_var("b");
        n1.set_id(Some(15));
        g.add_node(n0);
        g.add_node(n1);

        let where_clause = "where 1 = 2 and id(a) = $aid or id(b) = $bid";
        let params = json!({
            "aid": 12,
            "bid": 16
        });
        let mut lexer = lexer::Lexer::new(where_clause);
        match lexer.get_tokens() {
            Ok(tokens) => {
                let mut parser = parser::Parser::new(tokens);
                let mut ast = Box::new(AstTagNode::new_tag(AstTag::Query));
                parse_where_clause(&mut parser, &mut ast).expect("where clause ast");
                let mut visitor = WhereClauseAstVisitor::new(&g, Some(params));
                parser::walk_ast(&mut visitor, &(ast as Box<dyn Ast>)).expect("walk");
                assert_eq!(visitor.eval_stack.pop(), Some(PropertyValue::PBool(false)));
            }
            Err(_value) => {}
        }

    }

    #[test]
    fn parameters_or_op_param_test() {
        let mut g = PropertyGraph::new();
        let mut n0 = Node::new();
        n0.set_id(Some(12));
        n0.set_var("a");
        let mut n1 = Node::new();
        n1.set_var("b");
        n1.set_id(Some(15));
        g.add_node(n0);
        g.add_node(n1);

        let where_clause = "where $val = 2 and id(a) = $aid or id(b) = $bid";
        let params = json!({
            "aid": 12,
            "bid": 16,
            "val": 2
        });
        let mut lexer = lexer::Lexer::new(where_clause);
        match lexer.get_tokens() {
            Ok(tokens) => {
                let mut parser = parser::Parser::new(tokens);
                let mut ast = Box::new(AstTagNode::new_tag(AstTag::Query));
                parse_where_clause(&mut parser, &mut ast).expect("where clause ast");
                let mut visitor = WhereClauseAstVisitor::new(&g, Some(params));
                parser::walk_ast(&mut visitor, &(ast as Box<dyn Ast>)).expect("walk");
                assert_eq!(visitor.eval_stack.pop(), Some(PropertyValue::PBool(true)));
            }
            Err(_value) => {}
        }

    }
    #[test]
    fn parameters_gt_param_test() {
        let mut g = PropertyGraph::new();
        let mut n0 = Node::new();
        n0.set_var("a");
        n0.set_properties(vec![Property::new("age".to_string(), PropertyValue::PInteger(40))]);
        g.add_node(n0);

        let where_clause = "where a.age > $age";
        let params = json!({
            "age": 39
        });
        let mut lexer = lexer::Lexer::new(where_clause);
        match lexer.get_tokens() {
            Ok(tokens) => {
                let mut parser = parser::Parser::new(tokens);
                let mut ast = Box::new(AstTagNode::new_tag(AstTag::Query));
                parse_where_clause(&mut parser, &mut ast).expect("where clause ast");
                let mut visitor = WhereClauseAstVisitor::new(&g, Some(params));
                parser::walk_ast(&mut visitor, &(ast as Box<dyn Ast>)).expect("walk");
                assert_eq!(visitor.eval_stack.pop(), Some(PropertyValue::PBool(true)));
            }
            Err(_value) => {}
        }

    }
}