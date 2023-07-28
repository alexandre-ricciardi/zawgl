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

use std::collections::{HashSet, HashMap};

use bson::{Bson, Document, doc};
use cypher::query_engine::{process_cypher_query, CypherError};
use planner::make_cartesian_product;
use zawgl_core::model::{Property, PropertyValue, PropertyGraph};
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
    for pattern in &matched_graphs {
        let mut graph_doc = Document::new();  
        let mut nodes_doc = Vec::new();
        let mut rels_doc = Vec::new();
        let mut values_doc = Vec::new();
        if let Some(ret) = &request.return_clause {
            let wildcard = ret.has_wildcard();
            let mut grouping = HashSet::new();
            for ret_exp in &ret.expressions {
                match ret_exp {
                    ReturnExpression::Item(item) => {
                        match &item.item {
                            ValueItem::ItemPropertyName(prop_name) => {
                                get_item_property(item.alias.as_ref(), &prop_name.item_name, &prop_name.property_name, pattern, &mut values_doc)?;
                            },
                            ValueItem::NamedItem(named_item) => {
                                grouping.insert(named_item);
                                get_node_named(wildcard, item.alias.as_ref(), named_item, pattern, &mut nodes_doc)?;
                                get_relationship_named(wildcard, item.alias.as_ref(), named_item, pattern, &mut rels_doc)?;
                            }
                        }
                    },
                    _ => {}
                }
            }
            for ret_exp in &ret.expressions {
                match ret_exp {
                    ReturnExpression::FunctionCall(fun) => {
                        let mut values = Vec::new();
                        match fun.name.as_str() {
                            "sum" => {
                                let sums = compute_sum(&fun.args, pattern, &grouping);
                                values.extend(sums);},
                            _ => {}
                        }
                        let ret_name = if let Some(a) = &fun.alias {
                            a.to_string()
                        } else {
                            "sum".to_string()
                        };
                        if !values.is_empty() {
                            values_doc.push(doc! {ret_name: values});
                        }
                    },
                    _ => {}
                }
            }
        }
        if !nodes_doc.is_empty() {
            graph_doc.insert("nodes", nodes_doc);
        }
        if !rels_doc.is_empty() {
            graph_doc.insert("relationships", rels_doc);
        }
        if !values_doc.is_empty() {
            graph_doc.insert("values", values_doc);
        }
        graph_list.push(graph_doc);
    }
    result_doc.insert("graphs", graph_list);

    let mut response_doc = Document::new();
    response_doc.insert("request_id", request_id);
    response_doc.insert("result", result_doc);
    Ok(response_doc)
}

fn compute_sum(args: &Vec<ValueItem>, graph: &PropertyGraph, grouping: &HashSet<&String>) -> Vec<Bson> {
    let mut values = Vec::new();
    let mut groups = Vec::<Vec<&PropertyValue>>::new();
    for name in grouping {
        let mut group = Vec::<&PropertyValue>::new();
        for node in graph.get_nodes() {
            if let Some(var) = node.get_var() {
                if name == &var {
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
        }
        for rel in graph.get_relationships() {
            if let Some(var) = rel.get_var() {
                if name == &var {
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
        groups.push(group);
    }

    let product = make_cartesian_product(&groups);

    for elements in product {
        let mut sum_value = 0.;
        for prop in elements {
            sum_value += get_property_sum_value(prop);
        }
        for node in graph.get_nodes() {
            if let Some(var) = node.get_var() {
                if !grouping.contains(var) {
                    for arg in args {
                        if let ValueItem::ItemPropertyName(prop_arg) = arg {
                            if &prop_arg.item_name == var {
                                for prop in node.get_properties_ref() {
                                    if prop.get_name() == prop_arg.property_name {
                                        sum_value += get_property_sum_value(prop.get_value());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        for rel in graph.get_relationships() {
            if let Some(var) = rel.get_var() {
                if !grouping.contains(var) {
                    for arg in args {
                        if let ValueItem::ItemPropertyName(prop_arg) = arg {
                            if &prop_arg.item_name == var {
                                for prop in rel.get_properties_ref() {
                                    if prop.get_name() == prop_arg.property_name {
                                        sum_value += get_property_sum_value(prop.get_value());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        values.push(Bson::from(sum_value));
    }
    values
}

fn get_property_sum_value(prop: &PropertyValue) -> f64 {
    match prop {
        PropertyValue::PFloat(f) => *f,
        PropertyValue::PInteger(i) => *i as f64,
        PropertyValue::PUInteger(u) => *u as f64,
        _ => 0.
    }
}

fn get_item_property(alias: Option<&String>, item_name: &str, prop_name: &str, graph: &PropertyGraph, values_doc: &mut Vec<Document>) -> Result<(), CypherError> {
    for node in graph.get_nodes() {
        if let Some(var) = node.get_var() {
            if var == item_name {
                let ret_name = if let Some(a) = alias {
                    a.to_string()
                } else {
                    item_name.to_string()
                };
                for prop in node.get_properties_ref() {
                    if prop.get_name() == prop_name {
                        values_doc.push(build_property_value(&ret_name, prop.get_value()));
                    }
                }
            }
        }
    }
    for rel in graph.get_relationships_and_edges() {
        if let Some(var) = rel.relationship.get_var() {
            if var == item_name {
                let ret_name = if let Some(a) = alias {
                    a.to_string()
                } else {
                    item_name.to_string()
                };
                for prop in rel.relationship.get_properties_ref() {
                    if prop.get_name() == prop_name {
                        values_doc.push(build_property_value(&ret_name, prop.get_value()));
                    }
                }
            }
        }
    }
    Ok(())
}

fn get_node_named(ret_all: bool, alias: Option<&String>, name: &str, graph: &PropertyGraph, nodes_doc: &mut Vec<Document>) -> Result<(), CypherError> {
    for node in graph.get_nodes() {
        if let Some(var) = node.get_var() {
            if var == name || ret_all {
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
                nodes_doc.push(node_doc);
            }
        }
    }
    Ok(())
}
fn get_relationship_named(ret_all: bool, alias: Option<&String>, name: &str, graph: &PropertyGraph, rels_doc: &mut Vec<Document>) -> Result<(), CypherError> {
    for rel in graph.get_relationships_and_edges() {
        if let Some(var) = rel.relationship.get_var() {
            if var == name || ret_all {
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
                rels_doc.push(rel_doc);
            }
        }
    }
    Ok(())
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