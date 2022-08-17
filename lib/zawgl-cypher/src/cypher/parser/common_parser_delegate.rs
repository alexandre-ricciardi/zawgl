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
