use zawgl_cypher_query_model::{ast::AstVisitor, parameters::Parameters};
use zawgl_cypher_query_model::ast::{AstTagNode, AstVisitorResult};

struct WhereClauseAstVisistor {
    params: Option<Parameters>,
}

impl AstVisitor for WhereClauseAstVisistor {
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
        todo!()
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