// MIT License
//
// Copyright (c) 2022 Alexandre RICCIARDI
//
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

use crate::{ast::Ast, QueryStep};

pub enum Directive {
    CREATE,
    MATCH,
    DELETE
}

pub struct FunctionCall {
    pub name: String,
    pub args: Vec<String>,
}

impl FunctionCall {
    pub fn new(name: &str) -> Self {
        FunctionCall{name: String::from(name), args: Vec::new()}
    }
}

pub enum ReturnExpression {
    FunctionCall(FunctionCall),
    Item(String),
}

pub struct ReturnClause {
    pub expressions: Vec<ReturnExpression>,
}

impl ReturnClause {
    pub fn new() -> Self {
        ReturnClause{expressions: Vec::new()}
    }
}

pub struct WhereClause {
    pub expressions: Box<dyn Ast>,
}

impl WhereClause {
    pub fn new(ast: Box<dyn Ast>) -> Self {
        WhereClause{expressions: ast}
    }
}

pub enum Operator {
    Equal,
    Inferior,
    Superior,
    InferiorOrEqual,
    SuperiorOrEqual,
}
pub struct BoolCondition {
    pub first_member: Box<dyn Ast>,
    pub second_member: Box<dyn Ast>,
    pub operator: Operator,
}

pub struct Request {
    pub steps: Vec<QueryStep>,
    pub return_clause: Option<ReturnClause>,
}

impl Request {
    pub fn new() -> Self {
        Request {steps: Vec::new(), return_clause: None}
    }
}