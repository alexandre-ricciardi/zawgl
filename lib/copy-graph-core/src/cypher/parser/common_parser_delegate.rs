use super::*;
use super::error::*;
use super::super::lexer::TokenType;

pub fn parse_function_definition(parser: &mut Parser) -> ParserResult<Box<AstTagNode>> {
    let mut item_id = make_ast_token(&parser);
    parser.require(TokenType::OpenParenthesis)?;
    let mut func_node = make_ast_tag(AstTag::Function);
    parse_func_args(parser, &mut item_id)?;
    func_node.append(item_id);
    parser.require(TokenType::CloseParenthesis)?;
    Ok(func_node)
}

fn parse_func_args(parser: &mut Parser, parent_node: &mut Box<AstTokenNode>) -> ParserResult<()> {
    
    while parser.check(TokenType::Identifier) {
        parser.advance();
        let mut func_arg = Box::new(AstTagNode::new_tag(AstTag::FunctionArg));
        func_arg.append(make_ast_token(parser));
        parent_node.append(func_arg);
        if !parser.check(TokenType::Comma) {
            break;
        } else {
            parser.advance();
        }
    }
    Ok(())
}
