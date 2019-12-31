use super::error::*;
use super::super::lexer::{Token, TokenType};

pub struct AstNode {
    pub token_id: usize,
    pub childs: Vec<Box<AstNode>>,
}

impl AstNode {
    pub fn new(token_id: usize) -> Self {
        AstNode {token_id: token_id, childs: Vec::new()}
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

    pub fn require(&mut self, ast_node: Box<AstNode>, token_type: TokenType) -> ParserResult<Box<AstNode>> {
        if self.tokens[self.index].token_type != token_type {
            return Err(ParserError::SyntaxError);
        }
        self.advance();
        Ok(ast_node)
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn current_token_type(&mut self, token_type: TokenType) -> bool {
        if self.tokens[self.index].token_type == token_type {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn check(&self, token_type: TokenType) -> bool {
        self.tokens[self.index].token_type == token_type
    }

    pub fn next_token_type(& self, token_type: TokenType) -> bool {
        self.tokens.len() > self.index && self.tokens[self.index + 1].token_type == token_type
    }

}