mod error;
use super::lexer::{Token, TokenType};

pub struct AstNode {
    token_id: usize,
    childs: Vec<Box<AstNode>>,
}

impl AstNode {
    fn new(token_id: usize) -> Self {
        AstNode {token_id: token_id, childs: Vec::new()}
    }
}

pub struct Parser  {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {tokens : tokens, index: 0}
    }

    pub fn parse(&mut self) -> error::ParserResult<Box<AstNode>> {
        if self.tokens.len() > 0  {
            let tok = &self.tokens[0];
            match tok.token_type {
                TokenType::Create =>  {
                    self.enter_create(0)
                },
                _ => Err(error::ParserError::SyntaxError)
            }
        } else {
            Err(error::ParserError::SyntaxError)
        }
    }

    fn require(&mut self, ast_node: Box<AstNode>, token_type: TokenType) -> error::ParserResult<Box<AstNode>> {
        self.index += 1;
        if self.tokens[self.index].token_type != token_type {
            return Err(error::ParserError::SyntaxError);
        }
        Ok(ast_node)
    }

    fn enter_identifier(&mut self, mut parent_node: Box<AstNode>) -> error::ParserResult<Box<AstNode>> {
        self.index += 1;
        if self.tokens[self.index].token_type != TokenType::Identifier {
            return Err(error::ParserError::SyntaxError);
        }
        let id_node = Box::new(AstNode::new(self.index));
        parent_node.childs.push(id_node);
        Ok(parent_node)
    }

    fn current_token_type(&mut self, token_type: TokenType) -> bool {
        if self.tokens[self.index].token_type == token_type {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn next_token_type(& self, token_type: TokenType) -> bool {
        self.tokens.len() > self.index && self.tokens[self.index + 1].token_type == token_type
    }

    fn enter_labels(&mut self, parent_node: Box<AstNode>) -> error::ParserResult<Box<AstNode>> {
        if self.current_token_type(TokenType::Identifier) {
            let id_res = self.enter_identifier(parent_node)?;
            if self.current_token_type(TokenType::Comma) {
                return self.enter_labels(id_res);
            } else {
                return Ok(id_res);
            }
        }
        Ok(parent_node)
    }

    fn enter_node_def(&mut self, parent_node: Box<AstNode>) -> error::ParserResult<Box<AstNode>> {
        let id_res = self.enter_identifier(parent_node)?;
        let req_sc = self.require(id_res, TokenType::Colon)?;
        let labels = self.enter_labels(req_sc)?;
        Ok(labels)
    }

    fn enter_create(&mut self, token_id: usize) -> error::ParserResult<Box<AstNode>> {
        let create_node = Box::new(AstNode::new(token_id));
        let res = self.require(create_node, TokenType::OpenParenthesis)?;
        let next = self.enter_identifier(res)?;
        let req_close_par = self.require(next, TokenType::CloseParenthesis)?;
        Ok(req_close_par)
    }

}


#[cfg(test)]
mod test_parser {
    use super::*;
    #[test]
    fn test_bool_expr() {

    }
}

