use one_graph_core::model::init::InitContext;
use one_graph_core::model::*;
use super::cypher::query_engine::process_cypher_query;
use one_graph_core::graph_engine::GraphEngine;
use super::model::*;

use bson::{Document, doc};


pub struct GraphStore<'a> {
    ctx: InitContext<'a>,
}

impl <'a> GraphStore<'a> {
    pub fn new(dir: &'a str) -> Self {
        let ctx = InitContext::new(dir);
        GraphStore{ctx: ctx}
    }

    pub fn process_cypher_query(&mut self, query: &str) -> Option<Document> {
        let req = process_cypher_query(query)?;
        let mut graph_engine = GraphEngine::new(&self.ctx);
        match req.directive {
            Directive::CREATE => {
                let res = graph_engine.create_graph(&req.pattern)?;
                graph_engine.sync();
                req.return_clause.and_then(|ret| process_return_clause(&ret, &res))
            },
            Directive::MATCH => {
                let res = graph_engine.match_pattern(&req.pattern)?;
                req.return_clause.and_then(|ret| {
                    let mut doc = Document::new();
                    let mut counter = 0;
                    for graph in &res {
                        doc.insert(counter.to_string(), process_return_clause(&ret, graph)?);
                        counter += 1;
                    }
                    Some(doc)
                })
            },
            Directive::DELETE => {
                Some(Document::new())
            }
        }
    }
}

fn process_return_clause(return_clause: &ReturnClause, result: &PropertyGraph) -> Option<Document> {
    let mut res = Document::new();
    for expr in &return_clause.expressions {
        match expr {
            ReturnExpression::Item(item) => {
                res.insert(item, evaluate_item(result, item)?);
            }
            ReturnExpression::FunctionCall(func_call) => {
                res.insert(&func_call.name, evaluate_function_call(result, func_call)?);
            }
        }
    }
    Some(res)
}

fn evaluate_item(result: &PropertyGraph, item: &str) -> Option<Document> {
    for node in result.get_nodes() {
        if let Some(var) = node.get_var() {
            if var == item {

                let mut props = Vec::new();
                for p in node.get_properties_ref() {
                    
                    let name = p.get_name();
                    let value = p.get_value();
                    let mut bprop = Document::new();
                    match value {
                        PropertyValue::PBool(v) => bprop.insert(name, v),
                        PropertyValue::PFloat(f) => bprop.insert(name, f),
                        PropertyValue::PInteger(i) => bprop.insert(name, i),
                        PropertyValue::PString(s) => bprop.insert(name, s),
                    };
                    props.push(bprop);
                }
                return Some(doc!{
                    "id": node.get_id()?,
                    "properties": props
                });
            }
        }
    }

    for relationship in result.get_relationships() {
        if let Some(var) = relationship.get_var() {
            if var == item {

                let mut props = Vec::new();
                for p in relationship.get_properties_ref() {
                    
                    let name = p.get_name();
                    let value = p.get_value();

                    let mut bprop = Document::new();
                    match value {
                        PropertyValue::PBool(v) => bprop.insert(name, v),
                        PropertyValue::PFloat(f) => bprop.insert(name, f),
                        PropertyValue::PInteger(i) => bprop.insert(name, i),
                        PropertyValue::PString(s) => bprop.insert(name, s),
                    };
                    props.push(bprop);
                      
                return Some(doc!{
                    "id": relationship.get_id()?,
                    "properties": props
                });
            }
        }
    }

    None
}

fn evaluate_function_call(result: &PropertyGraph, func_call: &FunctionCall) -> Option<Document> {
    if func_call.name == "id" {
        for node in result.get_nodes() {
            if let Some(var) = node.get_var() {
                if func_call.args.contains(var) {
                    return Some(doc!{
                        var: node.get_id()?
                    });
                }
            }
        }
    }
    None
}