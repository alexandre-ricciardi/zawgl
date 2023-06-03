use std::fmt;

// MIT License
//
// Copyright (c) 2022 Alexandre RICCIARDI
//
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
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Integer,
    Float,
    Plus,
    Minus,
    Divide,
    Mult,
    True,
    False,
    And,
    Or,
    Match,
    Create,
    Delete,
    Where,
    Return,
    OpenParenthesis,
    CloseParenthesis,
    OpenBracket,
    CloseBracket,
    Identifier,
    Colon,
    Comma,
    OpenBrace,
    CloseBrace,
    LeftSourceRel,
    RightTargetRel,
    LeftTargetRel,
    RightSourceRel,
    UndirectedRel,
    Pipe,
    StringType,
    Equals,
    Dot,
    Parameter,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}



#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub begin: usize,
    pub end: usize,
    pub content: &'a str
}

impl <'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{}", self.content))
    }
}


impl <'a> Token<'a> {
    pub fn new(ttype: TokenType, beg: usize, end: usize, token_expr: &str) -> Token {
        Token {token_type: ttype, begin: beg, end: end, content: token_expr}
    }
    pub fn size(&self) -> usize {
        self.end - self.begin
    }
}
