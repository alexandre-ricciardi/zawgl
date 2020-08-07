use super::model::init::InitContext;
use super::query_engine::process_cypher_query;
use super::graph_engine::GraphEngine;
use super::model::Directive;

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
                graph_engine.add_graph(&req.pattern);
                graph_engine.sync();
                Some(Document::new())
            },
            Directive::MATCH => {
                let res = graph_engine.match_pattern(&req.pattern)?;
                Some(Document::new())
            },
            Directive::DELETE => {
                Some(Document::new())
            }
        }
    }
}