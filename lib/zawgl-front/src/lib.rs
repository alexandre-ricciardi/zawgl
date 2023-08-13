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

pub mod planner;
pub mod tx_handler;

use std::{collections::{HashMap, hash_map::Entry}, slice::Iter};

use bson::{Bson, Document, doc};
use cypher::query_engine::{process_cypher_query, CypherError};
use zawgl_core::{model::{Property, PropertyValue, PropertyGraph, Relationship, Node}, graph::{EdgeData, NodeIndex, EdgeIndex}};
use zawgl_cypher_query_model::{parameters::build_parameters, model::{Request, ReturnExpression, ValueItem}};
use tx_handler::{handle_graph_request, request_handler::RequestHandler, handler::TxHandler, DatabaseError};

extern crate zawgl_core;

extern crate bson;

pub mod cypher;

pub fn handle_open_cypher_request(tx_handler: TxHandler, graph_request_handler: RequestHandler, cypher_request: &Document) -> Result<Document, CypherError> {
    let query = cypher_request.get_str("query").map_err(|_err| CypherError::RequestError)?;
    let request_id = cypher_request.get_str("request_id").map_err(|_err| CypherError::RequestError)?;
    let parameters = cypher_request.get_document("parameters");
    let params = parameters.ok().map(build_parameters);
    let request = process_cypher_query(query, params);
    match request {
        Ok(r) => {
            let matched_graphs = handle_graph_request(tx_handler, graph_request_handler.clone(), r.steps.to_vec(), None);
            match matched_graphs {
                Ok(mg) => {
                    build_response(request_id, mg, &r)
                },
                Err(e) => {
                    graph_request_handler.lock().unwrap().cancel();
                    build_error(request_id, e)
                },
            }
        },
        Err(ce) => build_cypher_error(request_id, ce),
    }
}

fn build_error(request_id: &str, err: DatabaseError) -> Result<Document, CypherError> {
    let mut response_doc = Document::new();
    response_doc.insert("request_id", request_id);
    response_doc.insert("error", format!("database error {}", err));
    Ok(response_doc)
}

fn build_cypher_error(request_id: &str, err: CypherError) -> Result<Document, CypherError> {
    let mut response_doc = Document::new();
    response_doc.insert("request_id", request_id);
    response_doc.insert("error", format!("database error {}", err));
    Ok(response_doc)
}
fn build_response(request_id: &str, matched_graphs: Vec<PropertyGraph>, request: &Request) -> Result<Document, CypherError> {
    let mut result_doc = Document::new();
    let mut graph_list = Vec::new();
    
    if let Some(ret) = &request.return_clause {
        let wildcard = ret.has_wildcard();
        for pattern in &matched_graphs {
            let mut graph_doc = Document::new();  
            let mut nodes_doc = Vec::new();
            let mut rels_doc = Vec::new();
            
            for ret_exp in &ret.expressions {
                match ret_exp {
                    ReturnExpression::Item(item) => {
                        match &item.item {
                            ValueItem::NamedItem(named_item) => {
                                nodes_doc.extend(get_nodes_named(wildcard, item.alias.as_ref(), named_item, pattern)?);
                                rels_doc.extend(get_relationships_named(wildcard, item.alias.as_ref(), named_item, pattern)?);
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
            
            if !nodes_doc.is_empty() {
                graph_doc.insert("nodes", nodes_doc);
            }
            if !rels_doc.is_empty() {
                graph_doc.insert("relationships", rels_doc);
            }
            if !graph_doc.is_empty() {
                graph_list.push(graph_doc);
            }
        }

        let mut grouping = Vec::new();
        for ret_exp in &ret.expressions {
            match ret_exp {
                ReturnExpression::Item(item) => {
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

        let mut values_doc = vec![];
        
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
                for ret_exp in &ret.expressions {
                    match ret_exp {
                        ReturnExpression::Item(ret_item) => {
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
                                                        row.push(make_node_doc(ret_item.alias.as_ref(), &named_item, n)?);
                                                    }
                                                }
                                            },
                                            Item::Relationship(rel) => {
                                                if let Some(var) = rel.relationship.get_var() {
                                                    if var == named_item {
                                                        row.push(make_relationship_doc(ret_item.alias.as_ref(), &named_item, rel, combination.graph)?);
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

            let graphs = combinations.iter().map(|c| c.graph).collect::<Vec<&PropertyGraph>>();
            for ret_exp in &ret.expressions {
                match ret_exp {
                    ReturnExpression::FunctionCall(fun) => {
                        let ret_name = if let Some(a) = &fun.alias {
                            a.to_string()
                        } else {
                            "sum".to_string()
                        };
                        match fun.name.as_str() {
                            "sum" => {
                                let sum = compute_sum(&fun.args, &graphs);
                                row.push(doc! {ret_name: sum});
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
            values_doc.push(row);
        }

        if !values_doc.is_empty() {
            result_doc.insert("values", values_doc);
        }
    }
    if !graph_list.is_empty() {
        result_doc.insert("graphs", graph_list);
    }

    let mut response_doc = Document::new();
    response_doc.insert("request_id", request_id);
    response_doc.insert("result", result_doc);
    Ok(response_doc)
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

fn compute_sum(args: &Vec<ValueItem>, graphs: &Vec<&PropertyGraph>) -> Bson {
    let mut group = Vec::<&PropertyValue>::new();
    for graph in graphs {
        get_properties(graph, &mut group, args);
    }

    let mut sum_value = 0.;
    for prop in group {
        sum_value += get_property_sum_value(prop);
    }
    Bson::from(sum_value)
}

fn get_property_sum_value(prop: &PropertyValue) -> f64 {
    match prop {
        PropertyValue::PFloat(f) => *f,
        PropertyValue::PInteger(i) => *i as f64,
        PropertyValue::PUInteger(u) => *u as f64,
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
    graph: &'a PropertyGraph,
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

fn get_property_in_items(alias: Option<&String>, item_name: &str, prop_name: &str, items: &Vec<Item>) -> Result<Document, CypherError> {
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
                                return Ok(build_property_value(&ret_name, prop.get_value()));
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
                                return Ok(build_property_value(&ret_name, prop.get_value()));
                            }
                        }
                    }
                }
            }
        }
    }
    Err(CypherError::ResponseError)
}

fn build_items_combinations<'a: 'b, 'b>(mut grouping: Iter<&String>, graph: &'a PropertyGraph, combinations: &mut Vec::<Combination<'b>>, curr_items: &mut Vec<Item<'a>>) -> Result<(), CypherError> {
    if let Some(next) = grouping.next() {
        let items = get_named_items(next, graph)?;
        for item in items {
            curr_items.push(item);
            build_items_combinations(grouping.clone(), graph, combinations, curr_items)?;
        }
    } else {
        combinations.push(Combination { graph: graph, items: curr_items.to_vec() });
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

fn make_node_doc(alias: Option<&String>, name: &str, node: &Node) -> Result<Document, CypherError> {
    let ret_name = if let Some(a) = alias {
        a.to_string()
    } else {
        name.to_string()
    };
    let node_doc = doc!{
        "name": ret_name,
        "id": node.get_id().ok_or(CypherError::ResponseError)? as i64,
        "properties": build_properties(node.get_properties_ref()),
        "labels": Bson::from(node.get_labels_ref()),
    };
    Ok(node_doc)
}

fn make_relationship_doc(alias: Option<&String>, name: &str, rel: &EdgeData<NodeIndex, EdgeIndex, Relationship>, graph: &PropertyGraph) -> Result<Document, CypherError> {
    let ret_name = if let Some(a) = alias {
        a.to_string()
    } else {
        name.to_string()
    };
    let rel_doc = doc!{
        "name": ret_name,
        "id": rel.relationship.get_id().ok_or(CypherError::ResponseError)? as i64,
        "source_id": graph.get_node_ref(&rel.get_source()).get_id().ok_or(CypherError::ResponseError)? as i64,
        "target_id": graph.get_node_ref(&rel.get_target()).get_id().ok_or(CypherError::ResponseError)? as i64,
        "properties": build_properties(rel.relationship.get_properties_ref()),
        "labels": Bson::from(rel.relationship.get_labels_ref()),
    };
    Ok(rel_doc)
}

fn get_nodes_named(ret_all: bool, alias: Option<&String>, name: &str, graph: &PropertyGraph) -> Result<Vec<Document>, CypherError> {
    let mut nodes_doc = vec![];
    for node in graph.get_nodes() {
        if let Some(var) = node.get_var() {
            if var == name || ret_all {
                nodes_doc.push(make_node_doc(alias, name, node)?);
            }
        }
    }
    Ok(nodes_doc)
}
fn get_relationships_named(ret_all: bool, alias: Option<&String>, name: &str, graph: &PropertyGraph) -> Result<Vec<Document>, CypherError> {
    let mut rels_doc = vec![];
    for rel in graph.get_relationships_and_edges() {
        if let Some(var) = rel.relationship.get_var() {
            if var == name || ret_all {
                rels_doc.push(make_relationship_doc(alias, name, rel, graph)?);
            }
        }
    }
    Ok(rels_doc)
}

fn build_property_value(name: &str, value: &PropertyValue) -> Document {
    match value {
        PropertyValue::PBool(v) => doc! {
            name: v
        },
        PropertyValue::PFloat(f) => doc! {
            name: f
        },
        PropertyValue::PInteger(i) => doc! {
            name: i
        },
        PropertyValue::PUInteger(u) => doc! {
            name: *u as i64
        },
        PropertyValue::PString(s) => doc! {
            name: s
        }
    }
}

fn build_properties(item_properties: &Vec<Property>) -> Vec<Document> {
    let mut props = Vec::new();
    for p in item_properties {
        let name = p.get_name();
        let value = p.get_value();
        props.push(build_property_value(name, value));
    }
    props
}