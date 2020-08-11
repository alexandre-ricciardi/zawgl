use super::model::init::InitContext;
use super::cypher::query_engine::process_cypher_query;
use super::graph_engine::GraphEngine;
use super::model::*;

use bson::Document;


pub struct DbKernel<'a> {
    ctx: InitContext<'a>,
}

impl <'a> DbKernel<'a> {
    pub fn new(dir: &'a str) -> Self {
        let ctx = InitContext::new(dir);
        DbKernel{ctx: ctx}
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
                        doc.insert(counter.to_string(), process_return_clause(&ret, graph)?)?;
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
    for expr in &return_clause.expressions {
        match expr {
            Expression::Item(item) => {
                return evaluate_item(result, item)
            }
            Expression::FunctionCall(func_call) => {
                return evaluate_function_call(result, func_call)
            }
        }
    }
    None
}

fn evaluate_item(result: &PropertyGraph, item: &str) -> Option<Document> {
    for node in result.get_nodes() {
        if let Some(var) = node.get_var() {
            if var == item {
                return Some(doc!{
                    "id": node.get_id()?,
                    "properties": ""
                })
            }
        }
    }

    None
    
}

fn evaluate_function_call(result: &PropertyGraph, func_call: &FunctionCall) -> Option<Document> {
    None
}