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

use zawgl_cypher_query_model::{ast::{AstVisitor, AstVisitorResult, AstTagNode, Ast}, model::BoolCondition};

struct BoolExprVisitor {

}

impl AstVisitor for BoolExprVisitor {
    fn enter_create(&mut self) -> AstVisitorResult {
        Ok(())
    }
    fn enter_match(&mut self) -> AstVisitorResult {
        Ok(())
    }
    fn enter_path(&mut self) -> AstVisitorResult {
        Ok(())
    }
    fn enter_node(&mut self) -> AstVisitorResult {
        Ok(())
    }
    fn enter_relationship(&mut self, _node: &AstTagNode) -> AstVisitorResult {
        Ok(())
    }
    fn enter_property(&mut self) -> AstVisitorResult {
        Ok(())
    }
    fn enter_integer_value(&mut self, _value: Option<i64>) -> AstVisitorResult {
        Ok(())
    }
    fn enter_float_value(&mut self, _value: Option<f64>) -> AstVisitorResult {
        Ok(())
    }
    fn enter_string_value(&mut self, _value: Option<&str>) -> AstVisitorResult {
        Ok(())
    }
    fn enter_bool_value(&mut self, _value: Option<bool>) -> AstVisitorResult {
        Ok(())
    }
    fn enter_identifier(&mut self, _key: &str) -> AstVisitorResult {
        Ok(())
    }
    fn enter_variable(&mut self) -> AstVisitorResult {
        Ok(())
    }
    fn enter_label(&mut self) -> AstVisitorResult {
        Ok(())
    }
    fn enter_query(&mut self) -> AstVisitorResult {
        Ok(())
    }
    fn enter_return(&mut self) -> AstVisitorResult {
        Ok(())
    }
    fn enter_function(&mut self) -> AstVisitorResult {
        Ok(())
    }
    fn enter_function_arg(&mut self) -> AstVisitorResult {
        Ok(())
    }
    fn enter_item(&mut self) -> AstVisitorResult {
        Ok(())
    }
    fn enter_where(&mut self, _node: &AstTagNode) -> AstVisitorResult {
        Ok(())
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
    fn enter_parameter(&mut self, _name: &str) -> AstVisitorResult {
        todo!()
    }
    fn exit_parameter(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn enter_equality_operator(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_equality_operator(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn enter_and_operator(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_and_operator(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn enter_or_operator(&mut self) -> AstVisitorResult {
        todo!()
    }

    fn exit_or_operator(&mut self) -> AstVisitorResult {
        todo!()
    }
}

pub fn extract_mandatory_conditions_from_bool_expr(_ast: &Box<dyn Ast>) -> Vec<BoolCondition> {
    let res = Vec::new();

    res
}