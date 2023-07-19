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

use zawgl_core::{model::*, graph_engine::GraphEngine};

mod pattern_builder;

use pattern_builder::{build_pattern, merge_patterns};
use zawgl_cypher_query_model::{QueryStep, StepType, model::WhereClause, ast::AstVisitorError};

use crate::cypher::{query_engine::where_clause_filter::WhereClauseAstVisitor, parser};

pub fn make_cartesian_product<T>(pools: &[Vec<T>]) -> Vec<Vec<&T>> {
    let mut res = vec![];
 
    let mut list_iter = pools.iter();
    if let Some(first_list) = list_iter.next() {
        for i in first_list {
            res.push(vec![i]);
        }
    }
    for l in list_iter {
        let mut tmp = vec![];
        for r in res {
            for el in l {
                let mut tmp_el = r.clone();
                tmp_el.push(el);
                tmp.push(tmp_el);
            }
        }
        res = tmp;
    }
    res
}

pub fn handle_query_steps(steps: Vec<QueryStep>, graph_engine: &mut GraphEngine) -> Option<Vec<PropertyGraph>> {
    let mut results = Vec::<Vec<PropertyGraph>>::new();
    for step in steps {
        match step.step_type {
            StepType::MATCH => {
                if results.is_empty() {
                    for pattern in &step.patterns {
                        let matched = graph_engine.match_pattern(pattern);
                        if let Some(res) = matched {
                            results.push(res);
                        }
                    }
                } else {
                    let mut new_res = Vec::new();
                    for pattern in &step.patterns {
                        let products = make_cartesian_product(&results);
                        for product in &products {
                            let merge_sources = merge_patterns(product);
                            let merge = build_pattern(&merge_sources, pattern);
                            let matched = graph_engine.match_pattern(&merge);
                            if let Some(c) = matched {
                                new_res.push(c);
                            }
                        }
                    }
                    results = new_res;
                }
            },
            StepType::CREATE => {
                if results.is_empty() {
                    let created = graph_engine.match_patterns_and_create(&step.patterns);
                    if let Some(created_graphs) = created {
                        results = created_graphs;
                    }
                } else {
                    let mut to_match_and_create = Vec::new();
                    for pattern in &step.patterns {
                        let products = make_cartesian_product(&results);
                        for product in &products {
                            let merge_sources = merge_patterns(product);
                            let merge = build_pattern(&merge_sources, pattern);
                            to_match_and_create.push(merge);
                        }
                    }
                    let created = graph_engine.match_patterns_and_create(&to_match_and_create);
                    if let Some(created_graphs) = created {
                        results = created_graphs;
                    }
                }
            },
            StepType::DELETE => todo!(),
            StepType::WHERE => {
                if let Some(where_clause) = &step.where_clause {
                    let mut where_clause_results = Vec::new();
                    let products = make_cartesian_product(&results);
                    for product in &products {
                        let merged_product = merge_patterns(product);
                        if where_clause_filter(&merged_product, where_clause).ok()? {
                            where_clause_results.push(vec![merged_product]);
                        }
                    }
                    results = where_clause_results;
                }
            },
        }
    }
    let mut result = Vec::new();
    for res in &mut results {
        result.append(res);
    }
    Some(result)
}

fn where_clause_filter(graph: &PropertyGraph, where_clause: &WhereClause) -> Result<bool, AstVisitorError> {
    let ast = &where_clause.expressions;
    let mut visitor = WhereClauseAstVisitor::new(graph, where_clause.params.clone());
    parser::walk_ast(&mut visitor, ast)?;
    Ok(visitor.eval_stack.pop() == Some(PropertyValue::PBool(true)))
}