use super::parser::*;
use super::error::*;
use super::super::lexer::{TokenType};

fn enter_identifier(parser: &mut Parser, mut parent_node: Box<AstNode>) -> ParserResult<Box<AstNode>> {
    if parser.current_token_type(TokenType::Identifier) {
        let id_node = Box::new(AstNode::new(parser.index - 1));
        parent_node.childs.push(id_node);
        Ok(parent_node)
    } else {
        Err(ParserError::SyntaxError)
    }
    
}

fn enter_labels(parser: &mut Parser, parent_node: Box<AstNode>) -> ParserResult<Box<AstNode>> {
    if parser.check(TokenType::Identifier) {
        let id_res = enter_identifier(parser, parent_node)?;
        if parser.current_token_type(TokenType::Comma) {
            return enter_labels(parser, id_res);
        } else {
            return Ok(id_res);
        }
    }
    Ok(parent_node)
}

fn enter_node_def(parser: &mut Parser, parent_node: Box<AstNode>) -> ParserResult<Box<AstNode>> {
    let id_res = enter_identifier(parser, parent_node)?;
    let req_sc = parser.require(id_res, TokenType::Colon)?;
    let labels = enter_labels(parser, req_sc)?;
    Ok(labels)
}

pub fn parse_graph(parser: &mut Parser) -> ParserResult<Box<AstNode>> {
    let create_node = Box::new(AstNode::new(parser.index));
    parser.advance();
    let res = parser.require(create_node, TokenType::OpenParenthesis)?;
    let next = enter_node_def(parser, res)?;
    let req_close_par = parser.require(next, TokenType::CloseParenthesis)?;
    Ok(req_close_par)
}
