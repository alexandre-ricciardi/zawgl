use super::error::*;
use super::super::lexer::{TokenType};
use super::*;
use super::pattern_parser_delegate::*;
use super::return_clause_parser_delegate::*;
use super::where_clause_parser_delegate::parse_where_clause;

fn parse_match(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    let mut match_node = make_ast_tag(AstTag::Match);
    parse_path(parser, &mut match_node)?;
    parent_node.append(match_node);
    if parser.current_token_type_advance(TokenType::Match) {
        parse_match(parser, parent_node)?;
    }
    Ok(())
}


pub fn parse(parser: &mut Parser) -> ParserResult<Box<dyn Ast>> {
    if parser.get_tokens().len() > 0  {
        let mut query_node = make_ast_tag(AstTag::Query);

        let tok = &parser.get_tokens()[0];
        match tok.token_type {
            TokenType::Create =>  {
                parser.advance();
                let mut create_node = make_ast_tag(AstTag::Create);
                parse_path(parser, &mut create_node)?;
                query_node.append(create_node);
                parse_where_clause(parser, &mut query_node)?;
                parse_return(parser, &mut query_node)?;
                
                Ok(query_node)
                
            },
            TokenType::Match => {
                parser.advance();
                parse_match(parser, &mut query_node)?;
                if parser.current_token_type_advance(TokenType::Create) {
                    let mut create_node = make_ast_tag(AstTag::Create);
                    parse_path(parser, &mut create_node)?;
                    query_node.append(create_node);
                }
                parse_where_clause(parser, &mut query_node)?;
                parse_return(parser, &mut query_node)?;
                
                Ok(query_node)
            },
            _ => Err(ParserError::SyntaxError(parser.index))
        }
    } else {
        Err(ParserError::SyntaxError(parser.index))
    }
}
