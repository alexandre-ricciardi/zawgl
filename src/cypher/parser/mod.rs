pub mod error;
pub mod parser_utils;
pub mod pattern_parser_delegate;
pub mod properties_parser_delegate;
pub mod cypher_parser;

use super::lexer::*;
use self::error::*;

#[derive(Debug)]
pub enum AstTag  {
    Node,
    Property,
    RelDirectedLR,
    RelDirectedRL,
    RelUndirected,
    Variable,
    Label,
}

pub trait AstVisitor {
    fn visit(&self, node: &AstNode);
}

pub struct AstNode {
    pub token_id: usize,
    pub childs: Vec<Box<AstNode>>,
    pub ast_tag: Option<AstTag>,
    pub token_type: Option<TokenType>,
}

impl AstNode {
    pub fn new_empty() -> Self {
        AstNode::new(0)
    }
    pub fn new(token_id: usize) -> Self {
        AstNode {token_id: token_id, childs: Vec::new(), ast_tag: None, token_type: None}
    }
    pub fn new_tag(ast_tag: AstTag) -> Self {
        AstNode {token_id: 0, childs: Vec::new(), ast_tag: Some(ast_tag), token_type: None}
    }
    pub fn new_token_type(tok_type: TokenType) -> Self {
        AstNode {token_id: 0, childs: Vec::new(), ast_tag: None, token_type: Some(tok_type)}
    }
    pub fn accept_visitor(&self, visitor: &dyn AstVisitor) {
        visitor.visit(&self);
        for child in &self.childs {
            visitor.visit(&child.as_ref());
        }        
    }
}

pub struct Parser  {
    tokens: Vec<Token>,
    pub index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {tokens : tokens, index: 0}
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    } 

    pub fn require(&mut self, token_type: TokenType) -> ParserResult<usize> {
        if !self.check(token_type) {
            return Err(ParserError::SyntaxError);
        }
        self.advance();
        Ok(self.index)
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn has_next(&self) -> bool {
        self.index + 1 < self.tokens.len()
    }

    pub fn current_token_type_advance(&mut self, token_type: TokenType) -> bool {
        if self.tokens[self.index].token_type == token_type {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn get_current_token_type(&self) -> TokenType {
        self.tokens[self.index].token_type
    }

    pub fn check(&self, token_type: TokenType) -> bool {
        self.tokens[self.index].token_type == token_type
    }

    pub fn next_token_type(& self, token_type: TokenType) -> bool {
        self.tokens.len() > self.index && self.tokens[self.index + 1].token_type == token_type
    }

}


#[cfg(test)]
mod test_parser {
    use super::*;

    fn run(qry: &str) {
        let mut lexer = Lexer::new(qry);
        match lexer.get_tokens() {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                let root = cypher_parser::parse(&mut parser);
                parser_utils::print_node(&root.unwrap(), parser.get_tokens(), 0);
            },
            Err(value) => assert!(false)
        }
    }

    #[test]
    fn test_create() {
        run("CREATE (n:Person)");
    }
    #[test]
    fn test_create_labels() {
        run("CREATE (n:Person:Friend:Etc)");
    }

    #[test]
    fn test_create_graph() {
        run("CREATE (n:Person)-[r:FRIEND_OF]->(m:Person)");
    }

    #[test]
    fn test_properties_node() {
        run("CREATE (n:Person { name: 'hello', value: 'world' })");
    }

}

