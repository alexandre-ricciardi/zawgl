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
use model::{EvalResultItem, EvalScopeClause, WhereClause};
use zawgl_core::model::PropertyGraph;

pub mod model;
pub mod ast;
pub mod token;
pub mod properties;

#[derive(Clone)]
pub enum StepType {
    Match, OptionalMatch, Create, Delete, Where, With(EvalScopeClause), Return(EvalScopeClause)
}

#[derive(Clone)]
pub struct QueryStep {
    pub patterns: Vec<PropertyGraph>,
    pub step_type: StepType,
    pub where_clause: Option<WhereClause>,
}

impl QueryStep {
    pub fn new(step_type: StepType) -> Self {
        QueryStep {step_type, patterns: Vec::new(), where_clause: None }
    }

    pub fn new_where_clause(where_clause: WhereClause) -> Self {
        QueryStep {step_type: StepType::Where, patterns: Vec::new(), where_clause: Some(where_clause) }
    }
}

pub struct QueryResult {
    pub matched_graphs: Vec<PropertyGraph>,
    pub merged_graphs: PropertyGraph,
    pub return_eval: Vec::<Vec<EvalResultItem>>,
}

impl QueryResult {
    pub fn new(matched_graphs: Vec<PropertyGraph>, merged_graphs: PropertyGraph, return_eval: Vec::<Vec<EvalResultItem>>) -> Self {
        QueryResult{matched_graphs, merged_graphs, return_eval}
    }
    
}
