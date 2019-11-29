use super::lexer::{Token, TokenType};
use std::collections::VecDeque;

pub struct AstNode {
    token: Token,
    childs: Vec<AstNode>,
}

pub struct Parser  {
    root: Option<AstNode>,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {root: Option::None, tokens: Vec::new()}
    }
    pub fn consume(&mut self, token: Token) {
        self.tokens.push(token)
    }
    fn consume_create(&mut self, token: & Token) {

    }
}

