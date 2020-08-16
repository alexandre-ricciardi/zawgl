use super::*;
use super::error::*;
use super::super::lexer::TokenType;
use super::common_parser_delegate::*;

pub fn parse_return(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    if parser.has_next() && parser.check(TokenType::Return) {
        parser.require(TokenType::Return)?;
        let mut ret_node = Box::new(AstTagNode::new_tag(AstTag::Return));
        parse_return_expression(parser, &mut ret_node)?;
        parent_node.append(ret_node);
    }
    Ok(())
}

fn parse_return_expression(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    if parser.current_token_type_advance(TokenType::Identifier) {
        if parser.check(TokenType::OpenParenthesis) {
            let func = parse_function_definition(parser)?;
            parent_node.append(func);
        } else {
            let item_id = make_ast_token(&parser);
            let mut item_node = make_ast_tag(AstTag::Item);
            item_node.append(item_id);
            parent_node.append(item_node);
        }
        if parser.current_token_type_advance(TokenType::Comma) { 
            parse_return_expression(parser, parent_node)?;
        }
    }
    Ok(())
}