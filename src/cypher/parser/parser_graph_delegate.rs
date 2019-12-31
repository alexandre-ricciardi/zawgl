use super::parser::*;
use super::error::*;

pub struct GraphParserDelegate<'a> {
    parser: &'a Parser,
}

impl <'a> GraphParserDelegate<'a> {
    pub fn parse_graph(&mut self) -> ParserResult<Box<AstNode>> {

    }

    fn enter_identifier(&mut self, mut parent_node: Box<AstNode>) -> error::ParserResult<Box<AstNode>> {
        if self.parser.current_token_type(TokenType::Identifier) {
            let id_node = Box::new(AstNode::new(self.index - 1));
            parent_node.childs.push(id_node);
            Ok(parent_node)
        } else {
            Err(error::ParserError::SyntaxError)
        }
        
    }

    fn enter_labels(&mut self, parent_node: Box<AstNode>) -> error::ParserResult<Box<AstNode>> {
        if self.parser.check(TokenType::Identifier) {
            let id_res = self.enter_identifier(parent_node)?;
            if self.parser.current_token_type(TokenType::Comma) {
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
        self.parser.advance();
        let res = self.parser.require(create_node, TokenType::OpenParenthesis)?;
        let next = self.enter_node_def(res)?;
        let req_close_par = self.parser.require(next, TokenType::CloseParenthesis)?;
        Ok(req_close_par)
    }
}