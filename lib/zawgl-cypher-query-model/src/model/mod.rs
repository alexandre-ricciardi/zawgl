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

use serde_json::Value;
use zawgl_core::{graph::{EdgeData, EdgeIndex, NodeIndex}, model::{Node, Relationship}};
use crate::{ast::Ast, QueryStep};

pub enum Directive {
    CREATE,
    MATCH,
    DELETE
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<ValueItem>,
    pub alias: Option<String>,
}

impl FunctionCall {
    pub fn new(name: &str) -> Self {
        FunctionCall{name: String::from(name), args: Vec::new(), alias: None}
    }
}
#[derive(Debug, Clone)]
pub struct ItemPropertyName {
    pub item_name: String,
    pub property_name: String,
}

impl ItemPropertyName {
    pub fn new(item_name: &str, prop_name: &str) -> Self {
        ItemPropertyName { item_name: item_name.to_string(), property_name: prop_name.to_string() }
    }
}
#[derive(Debug, Clone)]
pub enum ValueItem {
    ItemPropertyName(ItemPropertyName),
    NamedItem(String)
}

#[derive(Debug, Clone)]
pub struct ScalarResult {
    pub name: String,
    pub value: f64,
}

impl ScalarResult {
    pub fn new(name: String, value: f64) -> Self {
        ScalarResult{name, value}
    }
}

#[derive(Debug, Clone)]
pub struct NodeResult {
    pub name: String,
    pub value: Node,
}

impl NodeResult {
    pub fn new(name: String, value: Node) -> Self {
        NodeResult{name, value}
    }
}

#[derive(Debug, Clone)]
pub struct StringResult {
    pub name: String,
    pub value: String,
}
impl StringResult {
    pub fn new(name: String, value: String) -> Self {
        StringResult{name, value}
    }
}
#[derive(Debug, Clone)]
pub struct BoolResult {
    pub name: String,
    pub value: bool,
}

impl BoolResult {
    pub fn new(name: String, value: bool) -> Self {
        BoolResult{name, value}
    }
}
#[derive(Debug, Clone)]
pub struct ListResult {
    pub name: String,
    pub values: Vec<EvalResultItem>,
}

impl ListResult {
    pub fn new(name: String, values: Vec<EvalResultItem>) -> Self {
        ListResult{name, values}
    }
}
#[derive(Debug, Clone)]
pub struct RelationshipResult {
    pub name: String,
    pub value: EdgeData<NodeIndex, EdgeIndex, Relationship>,
    pub source_nid: i64,
    pub target_nid: i64,
}

impl RelationshipResult {
    pub fn new(name: String, value: EdgeData<NodeIndex, EdgeIndex, Relationship>, source_nid: i64, target_nid: i64) -> Self {
        RelationshipResult{name, value, source_nid, target_nid}
    }
}

#[derive(Debug, Clone)]
pub enum EvalResultItem {
    Node(NodeResult),
    Relationship(RelationshipResult),
    Scalar(ScalarResult),
    Bool(BoolResult),
    String(StringResult),
    List(ListResult),
}

impl EvalResultItem {
    pub fn get_name(&self) -> &str {
        match self {
            EvalResultItem::Node(i) => &i.name,
            EvalResultItem::Relationship(i) => &i.name,
            EvalResultItem::Scalar(i) => &i.name,
            EvalResultItem::Bool(i) => &i.name,
            EvalResultItem::String(i) => &i.name,
            EvalResultItem::List(i) => &i.name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvalItem {
    pub item: ValueItem,
    pub alias: Option<String>,
}

impl EvalItem {
    pub fn new_property(item_name: &str, prop_name: &str) -> Self {
        EvalItem{item: ValueItem::ItemPropertyName(ItemPropertyName::new(item_name, prop_name)), alias: None}
    }
    pub fn new_named_item(name: &str) -> Self {
        EvalItem{item: ValueItem::NamedItem(name.to_string()), alias: None}
    }
}

#[derive(Debug, Clone)]
pub enum EvalScopeExpression {
    FunctionCall(FunctionCall),
    Item(EvalItem),
}

#[derive(Debug, Clone)]
pub struct EvalScopeClause {
    pub expressions: Vec<EvalScopeExpression>,
}

impl Default for EvalScopeClause {
    fn default() -> Self {
        Self::new()
    }
}

impl EvalScopeClause {
    pub fn new() -> Self {
        EvalScopeClause{expressions: Vec::new()}
    }
    pub fn new_expression(expressions: Vec<EvalScopeExpression>) -> Self {
        EvalScopeClause{expressions}
    }

    pub fn has_wildcard(&self) -> bool {
        for exp in &self.expressions {
            if let EvalScopeExpression::Item(item) = exp {
                if let ValueItem::NamedItem(ni) = &item.item {
                    if ni == "*" {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn contains_aggregation_function(&self) -> bool {
        for exp in &self.expressions {
            if let EvalScopeExpression::FunctionCall(func) = exp {
                if matches!(func.name.as_str(), "sum" | "collect" | "count") {
                    return true
                }
            }
        }
        false
    }
}

pub struct WhereClause {
    pub expressions: Box<dyn Ast>,
    pub params: Option<Value>,
}

impl Clone for WhereClause {
    fn clone(&self) -> Self {
        Self { expressions: self.expressions.clone_ast(), params: self.params.clone() }
    }
}

impl WhereClause {
    pub fn new(ast: Box<dyn Ast>, params: Option<Value>) -> Self {
        WhereClause{expressions: ast, params}
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
}

impl Request {
    pub fn new() -> Self {
        Request {steps: Vec::new()}
    }
}

impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}