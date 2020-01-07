use super::parser::*;
use super::error::*;
use super::super::lexer::{TokenType};


fn enter_prop_name(parser: &mut Parser, mut parent_node: &mut Box<AstNode>) -> ParserResult<usize> {
    let id_node = Box::new(AstNode::new(parser.index - 1));
    parent_node.childs.push(id_node);
    Ok(parser.index)
}


fn enter_property(parser: &mut Parser, mut parent_node: &mut Box<AstNode>) -> ParserResult<usize> {
    if parser.current_token_type_advance(TokenType::Identifier) {
        let mut prop_node = Box::new(AstNode::new_tag(AstTag::Property));
        enter_prop_name(parser, &mut prop_node)?;
        parser.require(TokenType::Colon)?;
        
        parent_node.childs.push(prop_node);
        Ok(parser.index)
    } else {
        Err(ParserError::SyntaxError)
    }
}

fn enter_properties(parser: &mut Parser, mut parent_node: &mut Box<AstNode>) -> ParserResult<usize> {
    if parser.check(TokenType::OpenBrace) {
        enter_property(parser, parent_node)
    } else {
        Ok(parser.index)
    }
}