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

pub mod error;
pub mod parser_utils;
mod pattern_parser_delegate;
mod properties_parser_delegate;
mod common_parser_delegate;
mod return_clause_parser_delegate;
mod with_clause_parser_delegate;
pub mod where_clause_parser_delegate;
pub mod cypher_parser;

use zawgl_cypher_query_model::ast::{AstTagNode, AstTag, AstTokenNode, Ast, AstVisitorResult, AstVisitor};
use zawgl_cypher_query_model::token::{TokenType, Token};
use self::error::*;


pub fn walk_ast(visitor: &mut dyn AstVisitor, ast: &Box<dyn Ast>) -> AstVisitorResult  {
    ast.accept(visitor)?;
    for child in ast.get_childs() {
        walk_ast(visitor, child)?;
    }
    ast.accept_exit(visitor)?;
    Ok(())
}

pub struct Parser<'a>  {
    tokens: Vec<Token<'a>>,
    pub index: usize,
}

impl <'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {tokens, index: 0}
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn get_current_token_value(&self) -> String {
        self.tokens[self.index].to_string()
    }

    pub fn require(&mut self, token_type: TokenType) -> ParserResult<usize> {
        if !self.check(token_type) {
            return Err(ParserError::SyntaxError(self.index, self.get_current_token_value()));
        }
        self.advance();
        Ok(self.index)
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn has_next(&self) -> bool {
        self.index + 1 < self.tokens.len()
    }

    pub fn current_token_type_advance(&mut self, token_type: TokenType) -> bool {
        if self.tokens.len() > self.index && self.tokens[self.index].token_type == token_type {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn get_current_token_type(&self) -> TokenType {
        self.tokens[self.index].token_type
    }

    pub fn check(&self, token_type: TokenType) -> bool {
        self.tokens.len() > self.index && self.tokens[self.index].token_type == token_type
    }

    pub fn next_token_type(& self, token_type: TokenType) -> bool {
        self.tokens.len() > self.index + 1 && self.tokens[self.index + 1].token_type == token_type
    }

}


fn make_ast_token(parser: &Parser) -> Box<AstTokenNode> {
    let token_id = parser.index - 1;
    let token = &parser.get_tokens()[token_id];
    Box::new(AstTokenNode::new_token(token_id, token.content.to_owned(), token.token_type ))
}

fn make_ast_tag(tag: AstTag) -> Box<AstTagNode> {
    Box::new(AstTagNode::new_tag(tag))
}

#[cfg(test)]
mod test_parser {
    use crate::cypher::lexer::Lexer;

    use super::*;

    fn run(qry: &str) {
        let mut lexer = Lexer::new(qry);
        match lexer.get_tokens() {
            Ok(tokens) => {
                println!("tokens: {:?}", tokens);
                let mut parser = Parser::new(tokens);
                let root = cypher_parser::parse(&mut parser);
                parser_utils::print_node(&root.unwrap(), parser.get_tokens(), 0);
            },
            Err(value) => assert!(false, "lexer error: {}", value)
        }
    }

    fn run_err(qry: &str) {
        let mut lexer = Lexer::new(qry);
        match lexer.get_tokens() {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                let root = cypher_parser::parse(&mut parser);
                if let Err(pe) = root {
                    match pe {
                        ParserError::SyntaxError(index, token) => println!("error at token {} with value {}", index, token),
                        ParserError::EndOfFile => todo!(),
                    }
                    
                } else {
                    assert!(false, "syntax error around")
                }
            },
            Err(value) => assert!(false, "lexer error: {}", value)
        }
    }
    #[test]
    fn test_create() {
        run("CREATE (n:Person) RETURN id(n, r, z)");
    }
    #[test]
    fn test_create_labels() {
        run("CREATE (n:Person:Friend:Etc)");
    }

    #[test]
    fn test_create_graph() {
        run("CREATE (n:Person)-[r:FRIEND_OF]->(m:Person) RETURN n, r, m");
    }

    #[test]
    fn test_properties_node() {
        run("CREATE (n:Person { name: 'hello', value: 'world' })");
    }
    #[test]
    fn test_properties_node_1() {
        run("CREATE (n:Person:Parent {test: 'Hello', case: 4.99})");
    }
    

    #[test]
    fn test_where_clause_1() {
        run("CREATE (n:Person:Parent {test: 'Hello', case: 4.99}) WHERE id(n) = 112 AND n.test = 'hello' OR n.case = 123.9 RETURN n, id(n)");
    }

    #[test]
    fn test_match_then_create() {
        run("match (p:Person), (m:Movie) create (m)<-[r:Played]-(p) return m, r, p");
    }

    
    #[test]
    fn test_match_match() {
        run("match (p:Person), (m:Movie) match (m)<-[r:Played]-(p) return m, r, p");
    }

    #[test]
    fn test_where_id_parameter() {
        run("MATCH (m:Movie) WHERE id(m) = $mid RETURN m, a, r");
    }
    
    #[test]
    fn test_match_where_create() {
        run("match (n:Movie), (p:Person) where id(n) = $nid and id(p) = $pid create (n:Movie)<-[r:Played]-(p:Person) return n, r, p");
    }

    #[test]
    fn test_syntax_error() {
        run_err("match (n:Movie)), (p:Person) where id(n) = $nid and id(p) = $pid create (n:Movie)<-[r:Played]-(p:Person) return n, r, p");
    }

    #[test]
    fn test_syntax_bool_expr() {
        run("match (n:Movie), (p:Person) where 1 = 2 and id(a) = $aid or id(b) = $bid return n, r, p");
    }
    
    #[test]
    fn test_syntax_param_bool_expr() {
        run("match (n:Movie), (p:Person) where $val = 2 and id(a) = $aid or id(b) = $bid return n, r, p");
    }

    #[test]
    fn test_greater_than_integer() {
        run("match (n:Person) where n.age > 40 return n");
    }
    #[test]
    fn test_return_sum() {
        run("MATCH (m:Movie), (a:Actor) MATCH (a)-[r:PLAYED_IN]->(m) RETURN sum(a.age) as total, collect(a) as list");
    }
    #[test]
    fn test_return_match_match_create() {
        run("match (test:Person) where id(test) = $pid1 match (t:Person) where id(t) = $pid2 create (test:Person)-[:IsFriendOf]->(new:Person {weight: $weight})-[:IsFriendOf]->(t:Person) return new");
    }
    #[test]
    fn test_return_match_create_create() {
        run("match (s:Person) where id(s) = $pid 
        create (s)-[:IsFriendOf]->(new:Person {weight: $weight})
        create (new)-[:IsFriendOf]->(new1:Person {weight: $weight})
        create (new)-[:IsFriendOf]->(new2:Person {weight: $weight})
        return s, test, test");
    }
    #[test]
    fn test_with_collect() {
        run("match (s:Person)-[:IsFriendOf]->(new:Person)-[:IsFriendOf]->(end:Person)
        with collect(end) as ps
        return ps");
    }
    #[test]
    fn test_return_collect() {
        run("match (s:Person)-[:IsFriendOf]->(new:Person)-[:IsFriendOf]->(end:Person)
        return collect(end) as ps");
    }
    #[test]
    fn test_optional_match() {
        run("match (s:Person)-[:IsFriendOf]->(new:Person) optional match (new)-[:IsFriendOf]->(end:Person)
        return collect(end) as ps");
    }
    #[test]
    fn test_recursive_match() {
        run("match (s:Person)-[:IsFriendOf*]->(new:Person) 
        return collect(end) as ps");
    }
    #[test]
    fn test_star() {
        run("match (s:Person)-[:IsFriendOf*]->(new:Person) 
        return *");
    }
}

