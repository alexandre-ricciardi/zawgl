use super::cypher::*;
use super::model::*;
use super::cypher::parser::*;

pub fn process_query(query: &str) -> Option<Request> {
    let mut lexer = lexer::Lexer::new(query);
    match lexer.get_tokens() {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(tokens);
            parser::cypher_parser::parse(&mut parser).ok().and_then(|root| build_request(&root))
        },
        Err(value) => None
    }
}

impl AstVisitable for AstTagNode {
    fn accept(&self, visitor: &dyn AstVisitor) -> Request {
        visitor.visit(&self);
        for child in &self.childs {
            visitor.visit(&child.as_ref());
        }        
    }
}

struct CypherAstVisitor {

}

impl CypherAstVisitor {
    fn new() -> Self {
        CypherAstVisitor {}
    }
}

impl AstVisitor for CypherAstVisitor {
    fn visit(&self, node: &AstNode) {
        
    }
}

fn build_pattern(nodes: &Vec<Box<parser::AstNode>>) -> Pattern {
    let mut pattern = Pattern::new();
    for node in nodes {
        match &node.ast_tag {
            Some(ast_tag) => {
                match &ast_tag {
                    parser::AstTag::Node => {
                        let mut node = Node::new();
    
                        pattern.add_node(node);
                    },
                    _ => {}
                }
                
            },
            None => {}
        }
    }
    pattern
}

pub fn build_request(root: &Box<parser::AstNode>) -> Option<Request> {
    
    root.token_type.and_then(|tok_type| {
        match tok_type {
            lexer::TokenType::Create => {
                let mut rq = Request::new(Directive::CREATE);
                rq.pattern = build_pattern(&root.childs);
                Some(rq)
            },
            _ => None
        }
    })
}