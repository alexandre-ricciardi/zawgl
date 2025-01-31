// MIT License
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use self::with_clause_parser_delegate::parse_with_clause;

use super::error::*;
use zawgl_cypher_query_model::ast::{AstTagNode, AstTag, Ast};
use zawgl_cypher_query_model::token::TokenType;
use super::*;
use super::pattern_parser_delegate::*;
use super::return_clause_parser_delegate::*;
use super::where_clause_parser_delegate::parse_where_clause;

fn parse_match(parser: &mut Parser, parent_node: &mut Box<AstTagNode>, optional: bool) -> ParserResult<()> {
    let mut match_node = if optional {
        make_ast_tag(AstTag::OptionalMatch)
    } else {
        make_ast_tag(AstTag::Match)
    };
    parse_path(parser, &mut match_node)?;
    parent_node.append(match_node);
    if parser.current_token_type_advance(TokenType::Match) {
        parse_match(parser, parent_node, false)?;
    } else if parser.current_token_type_advance(TokenType::Optional) {
        if parser.current_token_type_advance(TokenType::Match) {
            parse_match(parser, parent_node, true)?;
        } else {
            return Err(ParserError::SyntaxError(parser.index, parser.get_current_token_value()))
        }
    }
    Ok(())
}

fn parse_create(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    let mut create_node = make_ast_tag(AstTag::Create);
    parse_path(parser, &mut create_node)?;
    parent_node.append(create_node);
    if parser.current_token_type_advance(TokenType::Create) {
        parse_create(parser, parent_node)?;
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
                parse_create(parser, &mut query_node)?;
                parse_return(parser, &mut query_node)?;
                Ok(query_node)
                
            },
            TokenType::Match => {
                while parser.check(TokenType::Match) {
                    parser.advance();
                    parse_match(parser, &mut query_node, false)?;
                    parse_where_clause(parser, &mut query_node)?;
                }
                if parser.current_token_type_advance(TokenType::Create) {
                    parse_create(parser, &mut query_node)?;
                }
                if parser.current_token_type_advance(TokenType::With) {
                    parse_with_clause(parser, &mut query_node)?;
                }
                parse_return(parser, &mut query_node)?;
                
                Ok(query_node)
            },
            _ => Err(ParserError::SyntaxError(parser.index, parser.get_current_token_value()))
        }
    } else {
        Err(ParserError::SyntaxError(parser.index, parser.get_current_token_value()))
    }
}
