use super::error::*;
use super::super::lexer::{Token, TokenType};

#[derive(Debug)]
pub enum AstTag  {
    Node,
    Property,
    RelDirectedLR,
    RelDirectedRL,
    RelUndirected,
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
