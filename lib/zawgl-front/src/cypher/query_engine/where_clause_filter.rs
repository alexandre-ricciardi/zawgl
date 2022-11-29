use zawgl_core::model::PropertyGraph;
use zawgl_cypher_query_model::{ast::AstVisitor, parameters::Parameters};
use zawgl_cypher_query_model::ast::{AstTagNode, AstVisitorResult};

struct WhereClauseAstVisitor<'a> {
    graph: &'a PropertyGraph,
    params: Option<Parameters>,
}

impl <'a> WhereClauseAstVisitor<'a> {
    pub fn new(graph: &'a PropertyGraph, params: Option<Parameters>) -> Self {
        WhereClauseAstVisitor{graph, params}
    }
}

impl <'a> AstVisitor for WhereClauseAstVisitor<'a> {
    fn enter_create(&mut self, node: &AstTagNode) -> AstVisitorResult {
        todo!()
    }

    fn enter_match(&mut self, node: &AstTagNode) -> AstVisitorResult {
        todo!()
    }

    fn enter_path(&mut self, node: &AstTagNode) -> AstVisitorResult {
        todo!()
    }

    fn enter_node(&mut self, node: &AstTagNode) -> AstVisitorResult {
        todo!()
    }

    fn enter_relationship(&mut self, node: &AstTagNode) -> AstVisitorResult {
        todo!()
    }

    fn enter_property(&mut self, node: &AstTagNode) -> AstVisitorResult {
        todo!()
    }

    fn enter_integer_value(&mut self, value: Option<i64>) -> AstVisitorResult {
        todo!()
    }

    fn enter_float_value(&mut self, value: Option<f64>) -> AstVisitorResult {
        todo!()
    }

    fn enter_string_value(&mut self, value: Option<&str>) -> AstVisitorResult {
        todo!()
    }

    fn enter_bool_value(&mut self, value: Option<bool>) -> AstVisitorResult {
        todo!()
    }

    fn enter_identifier(&mut self, key: &str) -> AstVisitorResult {
        todo!()
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
        todo!()
    }

    fn enter_function_arg(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn enter_item(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn enter_where(&mut self, node: &AstTagNode) -> AstVisitorResult {
        todo!()
    }

    fn enter_parameter(&mut self, name: &str) -> AstVisitorResult {
        todo!()
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

    fn exit_identifier(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_variable(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_label(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_query(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_return(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_function(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_function_arg(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_item(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_where(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_parameter(&mut self) -> AstVisitorResult {
        todo!()
    }
}

#[cfg(test)]
mod test_where_clause {
    use zawgl_core::model::{PropertyGraph, Node};
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
            }
            Err(_value) => {}
        }

    }
}