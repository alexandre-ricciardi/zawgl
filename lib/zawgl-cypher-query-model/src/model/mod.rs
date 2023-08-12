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

use crate::{ast::{Ast}, QueryStep, parameters::Parameters};

pub enum Directive {
    CREATE,
    MATCH,
    DELETE
}

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

pub struct ItemPropertyName {
    pub item_name: String,
    pub property_name: String,
}

impl ItemPropertyName {
    pub fn new(item_name: &str, prop_name: &str) -> Self {
        ItemPropertyName { item_name: item_name.to_string(), property_name: prop_name.to_string() }
    }
}

pub enum ValueItem {
    ItemPropertyName(ItemPropertyName),
    NamedItem(String)
}

pub struct ReturnItem {
    pub item: ValueItem,
    pub alias: Option<String>,
}

impl ReturnItem {
    pub fn new_property(item_name: &str, prop_name: &str) -> Self {
        ReturnItem{item: ValueItem::ItemPropertyName(ItemPropertyName::new(item_name, prop_name)), alias: None}
    }
    pub fn new_named_item(name: &str) -> Self {
        ReturnItem{item: ValueItem::NamedItem(name.to_string()), alias: None}
    }
}
pub enum ReturnExpression {
    FunctionCall(FunctionCall),
    Item(ReturnItem),
}

pub struct ReturnClause {
    pub expressions: Vec<ReturnExpression>,
}

impl Default for ReturnClause {
    fn default() -> Self {
        Self::new()
    }
}

impl ReturnClause {
    pub fn new() -> Self {
        ReturnClause{expressions: Vec::new()}
    }

    pub fn has_wildcard(&self) -> bool {
        for exp in &self.expressions {
            if let ReturnExpression::Item(item) = exp {
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
            if let ReturnExpression::FunctionCall(func) = exp {
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
    pub params: Option<Parameters>,
}

impl Clone for WhereClause {
    fn clone(&self) -> Self {
        Self { expressions: self.expressions.clone_ast(), params: self.params.clone() }
    }
}

impl WhereClause {
    pub fn new(ast: Box<dyn Ast>, params: Option<Parameters>) -> Self {
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
    pub return_clause: Option<ReturnClause>,
}

impl Request {
    pub fn new() -> Self {
        Request {steps: Vec::new(), return_clause: None}
    }
}

impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}