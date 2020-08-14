mod fsm;
use std;
use std::fmt;
use std::error::Error;

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

pub struct Lexer<'a> {
    keywords: Vec<(TokenType, &'static str)>,
    input: &'a str,
    position: usize,
    line: usize,
    column: usize,
    lookahead: usize,
}

fn make_token(ttype: TokenType, beg: usize, end: usize, input: &str) -> Option<Token> {
    input.get(beg..end).map(|tok_expr| Token {token_type: ttype, begin: beg, end: end, content: tok_expr})
}

fn run_keyword_fsm<'a>(tok_type: TokenType, keyword: &'static str, input: &'a str, index: usize) -> Option<Token<'a>> {
    let mut kfsm = fsm::keyword_fsm::make_keyword_ignorecase_fsm(keyword);
    input.get(index..).and_then(|rest| kfsm.run(&rest)).and_then(|size| input.get(index..index + size.0)).map(|tok_expr| Token::new(tok_type, index, index + tok_expr.len(), tok_expr))
}

#[derive(Debug, Clone)]
pub enum LexerError {
    NotFound,
    WrongNumberFormat(usize),
    EndOfFile(usize),
    WrongIdentifierFormat(usize),
}

pub type LexerResult<T> = std::result::Result<T, LexerError>;


impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LexerError::NotFound => f.write_str("NotFound"),
            LexerError::EndOfFile(pos) => f.write_str(&format!("end of file at position : {}", pos)),
            LexerError::WrongNumberFormat(pos) => f.write_str(&format!("wrong format for number at position : {}", pos)),
            LexerError::WrongIdentifierFormat(pos) => f.write_str(&format!("wrong identifier format at position : {}", pos)),
        }
    }
}

impl Error for LexerError {
    fn description(&self) -> &str {
        match *self {
            LexerError::NotFound => "Record not found",
            LexerError::EndOfFile(_pos) => "Internal server error",
            LexerError::WrongNumberFormat(_pos) => "wrong number format",
            LexerError::WrongIdentifierFormat(_pos) => "wrong identifier format",

        }
    }
}

impl <'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            keywords: vec![(TokenType::True, "true"), (TokenType::False, "false"),
                            (TokenType::And, "and"), (TokenType::Or, "or"),
                            (TokenType::Plus, "+"),
                            (TokenType::Divide, "/"), (TokenType::Mult, "*"),
                            (TokenType::Match, "match"), (TokenType::Where, "where"),
                            (TokenType::Return, "return"), (TokenType::CloseParenthesis, ")"),
                            (TokenType::OpenParenthesis, "("), (TokenType::Colon, ":"),
                            (TokenType::OpenBrace, "{"), (TokenType::CloseBrace, "}"),
                            (TokenType::LeftSourceRel, "-["), (TokenType::RightTargetRel, "]->"),
                            (TokenType::LeftTargetRel, "<-["), (TokenType::RightSourceRel, "]-"),
                            (TokenType::UndirectedRel, "{"), (TokenType::Create, "create"),
                            (TokenType::Comma, ","), (TokenType::Equals, "="),
                            (TokenType::Pipe, "|"), (TokenType::Minus, "-")],
            input: input, position: 0, line: 0, column: 0, lookahead: 0}
    }
    pub  fn  next_token(&mut self) -> LexerResult<Token<'a>> {
        
        self.position = self.position + self.lookahead;
        if self.position >= self.input.len() {
            return Err(LexerError::EndOfFile(self.position));
        }
        for c in self.input.get(self.position..self.input.len()).unwrap().chars() {
            if c.is_whitespace() {
                self.position += 1;
                continue;
            }
            if c.is_numeric() {
                let mut number_fsm = fsm::number_fsm::make_number_fsm();
                return match number_fsm.run(&self.input.get(self.position..self.input.len()).unwrap()) {
                    Some(numlen) =>{
                        self.lookahead = numlen.0;
                        match numlen.1 {
                            fsm::number_fsm::NumberState::Integer => make_token(TokenType::Integer, self.position, self.position + numlen.0, &self.input).ok_or(LexerError::NotFound),
                            fsm::number_fsm::NumberState::NumberWithFractionalPart => make_token(TokenType::Float, self.position, self.position + numlen.0, &self.input).ok_or(LexerError::NotFound),
                            fsm::number_fsm::NumberState::NumberWithExponent => make_token(TokenType::Float, self.position, self.position + numlen.0, &self.input).ok_or(LexerError::NotFound),
                            _ => Err(LexerError::WrongNumberFormat(self.position))
                        }
                    } ,
                    None => Err(LexerError::WrongNumberFormat(self.position)),
                };
            }
            for keyword in &self.keywords {
                match run_keyword_fsm(keyword.0, keyword.1, &self.input, self.position) {
                    Some(tok) => {
                        self.lookahead = tok.size();
                        return Ok(tok)
                    },
                    None => {},
                }
            }
            let mut string_fsm = fsm::string_fsm::make_string_fsm();
            match string_fsm.run(&self.input.get(self.position..self.input.len()).unwrap()) {
                Some(string_len) => {
                    self.lookahead = string_len.0;
                    return make_token(TokenType::StringType, self.position, self.position + string_len.0, &self.input).ok_or(LexerError::NotFound);
                },
                None => {},
            }
            let mut identifier_fsm = fsm::identifier_fsm::make_identifier_fsm();
            return match identifier_fsm.run(&self.input.get(self.position..self.input.len()).unwrap()) {
                Some(idlen) => {
                    self.lookahead = idlen.0;
                    make_token(TokenType::Identifier, self.position, self.position + idlen.0, &self.input).ok_or(LexerError::NotFound)
                } ,

                None => Err(LexerError::WrongIdentifierFormat(self.position)),
            };
        }
        Err(LexerError::NotFound)
    }

    pub fn has_next(&self) -> bool {
        self.position + self.lookahead < self.input.len()
    }

    pub fn get_tokens(&mut self) -> LexerResult<Vec<Token<'a>>> {
        let mut res = Vec::new();
        while self.has_next() {
            let token = self.next_token()?;
            res.push(token);
        }
        Ok(res)
    }
}


#[cfg(test)]
mod test_lexer {
    use super::*;
    #[test]
    fn test_run_keyword_fsm() {
        let expr = String::from("true or false");
        match run_keyword_fsm(TokenType::True, "true", &expr, 0) {
            Some(tok) => assert_eq!(tok.content, "true"),
            None => assert!(false),
        }
    }
    #[test]
    fn test_bool_expr() {
        let mut lexer = Lexer::new("true or false ");
        let tres0 = lexer.next_token();
        match tres0 {
            Ok(tok) => assert_eq!(tok.content, "true"),
            Err(_msg) => assert!(false),
        }
        let tres1 = lexer.next_token();
        match tres1 {
            Ok(tok) => assert_eq!(tok.content, "or"),
            Err(_msg) => assert!(false),
        }
        
        let tres2 = lexer.next_token();
        match tres2 {
            Ok(tok) => assert_eq!(tok.content, "false"),
            Err(_msg) => assert!(false),
        }
    }
    #[test]
    fn test_run_identifier_fsm() {
        let mut lexer = Lexer::new("thisidmyid or      n       12.00033e-08");
        let tres0 = lexer.next_token();
        match tres0 {
            Ok(tok) => {
                assert_eq!(tok.content, "thisidmyid");
                assert_eq!(tok.token_type, TokenType::Identifier);
            },
            Err(_msg) => assert!(false),
        }
        let tres1 = lexer.next_token();
        match tres1 {
            Ok(tok) => assert_eq!(tok.content, "or"),
            Err(_msg) => assert!(false),
        }

        match lexer.next_token() {
            Ok(tok) => assert_eq!(tok.content, "n"),
            Err(_msg) => assert!(false),
        }
        
        let tres2 = lexer.next_token();
        match tres2 {
            Ok(tok) => assert_eq!(tok.content, "12.00033e-08"),
            Err(_msg) => assert!(false),
        }

        match lexer.next_token() {
            Ok(_tok) => assert!(false),
            Err(_msg) => assert!(true),
        }
    }

    #[test]
    fn test_run_string_fsm() {
        let mut lexer = Lexer::new("'this is a string' or 'this is another string'");
        match lexer.next_token() {
            Ok(tok) => assert_eq!(tok.content, "'this is a string'"),
            Err(_msg) => assert!(false),
        }
        match lexer.next_token() {
            Ok(tok) => assert_eq!(tok.content, "or"),
            Err(_msg) => assert!(false),
        }
        match lexer.next_token() {
            Ok(tok) => assert_eq!(tok.content, "'this is another string'"),
            Err(_msg) => assert!(false),
        }
    }
}