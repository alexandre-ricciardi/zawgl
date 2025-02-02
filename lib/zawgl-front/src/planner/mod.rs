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

use std::{collections::{hash_map::Entry, HashMap, HashSet}, ops::Deref, slice::Iter};

use zawgl_core::{graph::{EdgeData, EdgeIndex, NodeIndex}, graph_engine::{model::GraphProxy, GraphEngine}, make_cartesian_product, model::*};

mod pattern_builder;

use pattern_builder::{build_pattern, merge_patterns};
use zawgl_cypher_query_model::{ast::AstVisitorError, model::{BoolResult, EvalResultItem, EvalScopeClause, EvalScopeExpression, ListResult, NodeResult, RelationshipResult, ScalarResult, StringResult, ValueItem, WhereClause}, QueryResult, QueryStep, StepType};

use crate::cypher::{parser, query_engine::{where_clause_filter::WhereClauseAstVisitor, CypherError}};

pub fn handle_query_steps(steps: Vec<QueryStep>, graph_engine: &mut GraphEngine) -> Result<QueryResult, CypherError> {
    let mut results = Vec::<Vec<PropertyGraph>>::new();
    let mut eval_results = Vec::<Vec<EvalResultItem>>::new();
    let mut return_eval_results = Vec::<Vec<EvalResultItem>>::new();
    let mut result_graphs = vec![];
    for step in steps {
        match step.step_type {
            StepType::Match => {
                if eval_results.is_empty() {
                    results = handle_match(&results, graph_engine, &step, &vec![], false);
                } else {
                    let mut res = vec![];
                    for eval_row in &eval_results {
                        res.append(&mut handle_match(&results, graph_engine, &step, eval_row, false));
                    }
                    results = res;
                }
            },
            StepType::OptionalMatch => {
                if eval_results.is_empty() {
                    results = handle_match(&results, graph_engine, &step, &vec![], true);
                } else {
                    let mut res = vec![];
                    for eval_row in &eval_results {
                        res.append(&mut handle_match(&results, graph_engine, &step, eval_row, true));
                    }
                    results = res;
                }
            },
            StepType::Create => {
                if eval_results.is_empty() {
                    results = handle_create(&results, graph_engine, &step, &vec![]);
                } else {
                    let mut res = vec![];
                    for eval_row in &eval_results {
                        res.append(&mut handle_create(&results, graph_engine, &step, eval_row));
                    }
                    results = res;
                }
            },
            StepType::Delete => todo!(),
            StepType::Where => {
                if let Some(where_clause) = &step.where_clause {
                    let mut where_clause_results = Vec::new();
                    let products = make_cartesian_product(&results);
                    for product in &products {
                        let merged_product = merge_patterns(product, &vec![]);
                        if where_clause_filter(&merged_product, where_clause).map_err(|err| CypherError::EvalError)? {
                            where_clause_results.push(vec![merged_product]);
                        }
                    }
                    results = where_clause_results;
                }
            },
            StepType::With(eval_scope) => {
                (_, eval_results) = handle_eval(&mut results, eval_scope, &eval_results)?;
            },
            StepType::Return(eval_scope) => {
                (result_graphs, return_eval_results) = handle_eval(&mut results, eval_scope, &eval_results)?;
            },
        }
    }
    let merged_graphs = merge_graphs(&result_graphs);
    Ok(QueryResult::new(result_graphs, merged_graphs, return_eval_results))
}

fn handle_eval(results: &mut Vec::<Vec<PropertyGraph>>, eval_scope: EvalScopeClause, eval_results: &Vec<Vec<EvalResultItem>>) -> Result<(Vec<PropertyGraph>, Vec<Vec<EvalResultItem>>), CypherError> {
    let matched_graphs = flatten_results(results);
    let mut grouping = Vec::new();
    for ret_exp in &eval_scope.expressions {
        match ret_exp {
            EvalScopeExpression::Item(item) => {
                match &item.item {
                    ValueItem::ItemPropertyName(prop_name) => {
                        grouping.push(&prop_name.item_name);
                    },
                    ValueItem::NamedItem(named_item) => {
                        grouping.push(named_item);
                    }
                }
            },
            _ => {}
        }
    }

    let mut combinations = vec![];
    let mut curr_items = vec![];
    for graph in &matched_graphs {
        build_items_combinations(grouping.iter(), &graph, &mut combinations, &mut curr_items)?;
    }
    let mut eval_result_scope = vec![];
    
    let mut aggregations = HashMap::new();

    for combination in &combinations {
        let ids = combination.get_item_ids();
        if let Entry::Vacant(e) = aggregations.entry(ids) {
            e.insert(vec![combination]);
        } else {
            let idsref = combination.get_item_ids();
            aggregations.get_mut(&idsref).unwrap().push(combination);
        }
    }

    for combinations in aggregations.values() {
        let mut row = vec![];
        if let Some(combination) = combinations.first() {
            let items = combination.get_items();
            for ret_exp in &eval_scope.expressions {
                match ret_exp {
                    EvalScopeExpression::Item(ret_item) => {
                        match &ret_item.item {
                            ValueItem::ItemPropertyName(prop_name) => {
                                row.push(get_property_in_items(ret_item.alias.as_ref(), &prop_name.item_name, &prop_name.property_name, items)?);
                            },
                            ValueItem::NamedItem(named_item) => {
                                for item in &combination.items {
                                    match item {
                                        Item::Node(n) => {
                                            if let Some(var) = n.get_var() {
                                                if var == named_item {
                                                    row.push(EvalResultItem::Node(make_node(ret_item.alias.as_ref(), &named_item, n)));
                                                }
                                            }
                                        },
                                        Item::Relationship(rel) => {
                                            if let (Some(var), Some(graph)) = (rel.relationship.get_var(), combination.graph) {
                                                if var == named_item {
                                                    row.push(EvalResultItem::Relationship(make_relationship(ret_item.alias.as_ref(), &named_item, rel, graph)?));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
        }

        let graphs = combinations.iter().map(|c| c.graph).collect::<Vec<Option<&PropertyGraph>>>();
        for ret_exp in &eval_scope.expressions {
            match ret_exp {
                EvalScopeExpression::FunctionCall(fun) => {
                    let ret_name = if let Some(a) = &fun.alias {
                        a.to_string()
                    } else {
                        fun.name.to_string()
                    };
                    match fun.name.as_str() {
                        "sum" => {
                            let sum = compute_sum(&fun.args, &graphs);
                            row.push(EvalResultItem::Scalar(ScalarResult::new(ret_name, sum)));
                        },
                        "collect" => {
                            row.push(EvalResultItem::List(ListResult::new(ret_name, build_item_list(&fun.args, &graphs)?)));
                        }
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        eval_result_scope.push(row);
    }

    let mut eval_result_scope_join = vec![];

    for eval_item_row in eval_results {
        let mut row = vec![];
        if eval_result_scope.is_empty() {
            for eval_item in eval_item_row {
                if grouping.contains(&&eval_item.get_name().to_string()) {
                    row.push(eval_item.clone());
                }
            }
        } else {
            for eval_row in &mut eval_result_scope {
                for eval_item in eval_item_row {
                    if grouping.contains(&&eval_item.get_name().to_string()) {
                        row.push(eval_item.clone());
                        row.append(eval_row);
                    }
                }
            }
        }
        eval_result_scope_join.push(row);
    }

    if eval_results.is_empty() {
        Ok((matched_graphs, eval_result_scope))
    } else {
        Ok((matched_graphs, eval_result_scope_join))
    }
}

fn handle_match(results: &Vec::<Vec<PropertyGraph>>, graph_engine: &mut GraphEngine, step: &QueryStep, eval_row: &Vec<EvalResultItem>, optional: bool) -> Vec::<Vec<PropertyGraph>> {
    let mut new_res = Vec::new();
    if results.is_empty() {
        for pattern in &step.patterns {
            let matched = graph_engine.match_pattern(pattern);
            if let Some(res) = matched {
                new_res.push(res);
            }
        }
    } else {
        for pattern in &step.patterns {
            let products = make_cartesian_product(&results);
            for product in &products {
                let merge_sources = merge_patterns(product, eval_row);
                let merge = build_pattern(&merge_sources, pattern);
                let matched = graph_engine.match_pattern(&merge);
                if let Some(c) = matched {
                    new_res.push(c);
                } else if optional {
                    new_res.push(product.iter().map(|g| g.deref().clone()).collect());
                }
            }
        }
    }
    new_res
}

fn handle_create(results: &Vec::<Vec<PropertyGraph>>, graph_engine: &mut GraphEngine, step: &QueryStep, eval_row: &Vec<EvalResultItem>) -> Vec::<Vec<PropertyGraph>> {
    let mut new_res = Vec::new();
    if results.is_empty() {
        let created = graph_engine.match_patterns_and_create(&step.patterns);
        if let Some(created_graphs) = created {
            new_res = created_graphs;
        }
    } else {
        let mut to_match_and_create = Vec::new();
        for pattern in &step.patterns {
            let products = make_cartesian_product(&results);
            for product in &products {
                let merge_sources = merge_patterns(product, eval_row);
                let merge = build_pattern(&merge_sources, pattern);
                to_match_and_create.push(merge);
            }
        }
        let created = graph_engine.match_patterns_and_create(&to_match_and_create);
        if let Some(created_graphs) = created {
            new_res = created_graphs;
        }
    }
    new_res
}

fn flatten_results(results: &mut Vec::<Vec<PropertyGraph>>) -> Vec<PropertyGraph> {
    let mut result = Vec::new();
    for res in results {
        result.append(res);
    }
    result
}

fn where_clause_filter(graph: &PropertyGraph, where_clause: &WhereClause) -> Result<bool, AstVisitorError> {
    let ast = &where_clause.expressions;
    let mut visitor = WhereClauseAstVisitor::new(graph, where_clause.params.clone());
    parser::walk_ast(&mut visitor, ast)?;
    Ok(visitor.eval_stack.pop() == Some(PropertyValue::PBool(true)))
}

fn get_properties<'a: 'b, 'b>(graph: &'a PropertyGraph, group: &'b mut Vec::<&'a PropertyValue>, args: &Vec<ValueItem>) {
    for node in graph.get_nodes() {
        if let Some(var) = node.get_var() {
            for arg in args {
                if let ValueItem::ItemPropertyName(prop_arg) = arg {
                    if &prop_arg.item_name == var {
                        for prop in node.get_properties_ref() {
                            if prop.get_name() == prop_arg.property_name {
                                group.push(prop.get_value())
                            }
                        }
                    }
                }
            }
        }
    }
    for rel in graph.get_relationships() {
        if let Some(var) = rel.get_var() {
            for arg in args {
                if let ValueItem::ItemPropertyName(prop_arg) = arg {
                    if &prop_arg.item_name == var {
                        for prop in rel.get_properties_ref() {
                            if prop.get_name() == prop_arg.property_name {
                                group.push(prop.get_value())
                            }
                        }
                    }
                }
            }
            
        }
    }
}

fn compute_sum(args: &Vec<ValueItem>, graphs: &Vec<Option<&PropertyGraph>>) -> f64 {
    let mut group = Vec::<&PropertyValue>::new();
    for ograph in graphs {
        if let Some(graph) = ograph {
            get_properties(graph, &mut group, args);
        }
    }

    let mut sum_value = 0.;
    for prop in group {
        sum_value += get_property_sum_value(prop);
    }
    sum_value
}

fn build_item_list(args: &Vec<ValueItem>, graphs: &Vec<Option<&PropertyGraph>>) -> Result<Vec<EvalResultItem>, CypherError> {
    let mut list = Vec::new();
    for ograph in graphs {
        if let Some(graph) = ograph {
            for node in graph.get_nodes() {
                if let Some(var) = node.get_var() {
                    for arg in args {
                        if let ValueItem::NamedItem(name) = arg {
                            if name == var {
                                list.push(EvalResultItem::Node(make_node(None, &name, node)));
                            }
                        }
                    }
                }
            }
            for rel in graph.get_edges() {
                if let Some(var) = rel.relationship.get_var() {
                    for arg in args {
                        if let ValueItem::NamedItem(name) = arg {
                            if name == var {
                                list.push(EvalResultItem::Relationship(make_relationship(None, &name, rel, graph)?));
                            }
                        }
                    }
                    
                }
            }
        }
    }
    Ok(list)
}

fn make_node(alias: Option<&String>, name: &str, node: &Node) -> NodeResult {
    let ret_name = if let Some(a) = alias {
        a.to_string()
    } else {
        name.to_string()
    };
    let mut ret_node = node.clone();
    ret_node.set_var(&ret_name);
    NodeResult::new(ret_name, ret_node)
}

fn make_relationship(alias: Option<&String>, name: &str, rel: &EdgeData<NodeIndex, EdgeIndex, Relationship>, graph: &PropertyGraph) -> Result<RelationshipResult, CypherError> {
    let ret_name = if let Some(a) = alias {
        a.to_string()
    } else {
        name.to_string()
    };
    let mut ret_rel = rel.clone();
    ret_rel.relationship.set_var(&ret_name);
    let sid = graph.get_node_ref(&ret_rel.get_source()).get_id().ok_or(CypherError::ResponseError)? as i64;
    let tid = graph.get_node_ref(&ret_rel.get_target()).get_id().ok_or(CypherError::ResponseError)? as i64;
    Ok(RelationshipResult::new(ret_name, ret_rel, sid, tid))
}

fn get_property_sum_value(prop: &PropertyValue) -> f64 {
    match prop {
        PropertyValue::PFloat(f) => *f,
        PropertyValue::PInteger(i) => *i as f64,
        PropertyValue::PUInteger(u) => f64::try_from(*u as u32).unwrap_or_default(),
        _ => 0.
    }
}

#[derive(Debug, Clone)]
enum Item<'a> {
    Node(&'a Node),
    Relationship(&'a EdgeData<NodeIndex, EdgeIndex, Relationship>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ItemId {
    NodeId(u64),
    RelationshipId(u64),
}

struct Combination<'a> {
    graph: Option<&'a PropertyGraph>,
    items: Vec<Item<'a>>,
}

impl<'a> Combination<'a> {
    fn get_item_ids(&self) -> Vec<ItemId> {
        self.items.iter().map(|item| match item {
            Item::Node(node) => ItemId::NodeId(node.get_id().unwrap()),
            Item::Relationship(rel) => ItemId::RelationshipId(rel.relationship.get_id().unwrap())
        }).collect::<Vec<ItemId>>()
    }
    fn get_items(&self) -> &'a Vec<Item> {
        &self.items
    }
}

fn get_property_in_items(alias: Option<&String>, item_name: &str, prop_name: &str, items: &Vec<Item>) -> Result<EvalResultItem, CypherError> {
    for item in items {
        match item {
            Item::Node(node) => {
                if let Some(var) = node.get_var() {
                    if var == item_name {
                        let ret_name = if let Some(a) = alias {
                            a.to_string()
                        } else {
                            item_name.to_string()
                        };
                        for prop in node.get_properties_ref() {
                            if prop.get_name() == prop_name {
                                return Ok(build_property_value(ret_name, prop.get_value()));
                            }
                        }
                    }
                }
            },
            Item::Relationship(rel) => {
                if let Some(var) = rel.relationship.get_var() {
                    if var == item_name {
                        let ret_name = if let Some(a) = alias {
                            a.to_string()
                        } else {
                            item_name.to_string()
                        };
                        for prop in rel.relationship.get_properties_ref() {
                            if prop.get_name() == prop_name {
                                return Ok(build_property_value(ret_name, prop.get_value()));
                            }
                        }
                    }
                }
            }
        }
    }
    Err(CypherError::EvalError)
}

fn build_property_value(ret_name: String, value: &PropertyValue) -> EvalResultItem {
    match value {
        PropertyValue::PBool(v) => EvalResultItem::Bool(BoolResult::new(ret_name, *v)),
        PropertyValue::PFloat(f) => EvalResultItem::Scalar(ScalarResult::new(ret_name, *f)),
        PropertyValue::PInteger(i) => EvalResultItem::Scalar(ScalarResult::new(ret_name,*i as f64)),
        PropertyValue::PUInteger(u) => EvalResultItem::Scalar(ScalarResult::new(ret_name,*u as f64)),
        PropertyValue::PString(s) => EvalResultItem::String(StringResult::new(ret_name,s.clone())),
    }
}

fn build_items_combinations<'a: 'b, 'b>(mut grouping: Iter<&String>, graph: &'a PropertyGraph, combinations: &mut Vec::<Combination<'b>>, curr_items: &mut Vec<Item<'a>>) -> Result<(), CypherError> {
    if let Some(next) = grouping.next() {
        let items = get_named_items(next, graph)?;
        for item in items {
            curr_items.push(item);
            build_items_combinations(grouping.clone(), graph, combinations, curr_items)?;
        }
    } else {
        combinations.push(Combination { graph: Some(graph), items: curr_items.to_vec() });
        curr_items.clear();
    }
    Ok(())
}


fn get_named_items<'a>(name: &str, graph: &'a PropertyGraph) -> Result<Vec<Item<'a>>, CypherError> {
    let mut res = vec![];
    for node in graph.get_nodes() {
        if let Some(var) = node.get_var() {
            if var == name {
                res.push(Item::Node(&node));
            }
        }
    }
    for rel in graph.get_relationships_and_edges() {
        if let Some(var) = rel.relationship.get_var() {
            if var == name {
                res.push(Item::Relationship(&rel));
            }
        }
    }
    Ok(res)
}

fn merge_graphs(graphs: &Vec<PropertyGraph>) -> PropertyGraph {
    let mut merge = PropertyGraph::new();
    let mut graph_index_to_mid = HashMap::new();
    let mut nid_set = HashSet::new();
    let mut graph_index = 0;
    for g in graphs {
        let mut graph_id_to_mid = HashMap::new();
        for n in g.get_nodes() {
            if !nid_set.contains(&n.get_id()) {
                let mid = merge.add_node(n.clone());
                nid_set.insert(n.get_id());
                graph_id_to_mid.insert(n.get_id(), mid);
            }
        }
        graph_index_to_mid.insert(graph_index, graph_id_to_mid);
        graph_index += 1;
    }
    let mut rid_set = HashSet::new();
    graph_index = 0;
    for g in graphs {
        for edge_data in g.get_relationships_and_edges() {
            if !rid_set.contains(&edge_data.relationship.get_id()) {
                let src_node = g.get_node_ref(&edge_data.source);
                let tgt_node = g.get_node_ref(&edge_data.target);
                let rel = edge_data.relationship.clone();
                let source = graph_index_to_mid[&graph_index].get(&src_node.get_id());
                let target = graph_index_to_mid[&graph_index].get(&tgt_node.get_id());
                if let (Some(sid), Some(tid)) = (source, target) {
                    merge.add_relationship(rel, *sid, *tid);
                    rid_set.insert(edge_data.relationship.get_id());
                }
            }
        }
    }
    merge
}