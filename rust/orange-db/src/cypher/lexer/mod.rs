mod fsm;
use std;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    Number,
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
    Identifier,
}



#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    begin: usize,
    end: usize,
    content: String
}

impl Token {
    pub fn new(ttype: TokenType, beg: usize, end: usize, token_expr: &String) -> Token {
        Token {token_type: ttype, begin: beg, end: end, content: token_expr.clone()}
    }
    pub fn size(&self) -> usize {
        self.end - self.begin
    }
}

pub struct Lexer {
    keywords: Vec<(TokenType, &'static str)>,
    input: String,
    position: usize,
    line: usize,
    column: usize,
    lookahead: usize,
}

fn make_token(ttype: TokenType, beg: usize, end: usize, input: &String) -> Option<Token> {
    input.get(beg..end).map(|tok_expr| Token {token_type: ttype, begin: beg, end: end, content: String::from(tok_expr)})
}

fn run_keyword_fsm(tok_type: TokenType, keyword: &'static str, input: &String, index:usize) -> Option<Token> {
    let mut kfsm = fsm::keyword_fsm::make_keyword_ignorecase_fsm(keyword);
    kfsm.run(&input[index..input.len()]).map(|size| Token::new(tok_type, index, index + size, input))
}

#[derive(Debug, Clone)]
pub struct LexerError(&'static str);

pub type LexerResult<T> = std::result::Result<T, LexerError>;

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            keywords: vec![(TokenType::True, "true"), (TokenType::False, "false"),
                            (TokenType::And, "and"), (TokenType::Or, "or"),
                            (TokenType::Plus, "+"), (TokenType::Minus, "-"),
                            (TokenType::Divide, "/"), (TokenType::Mult, "*"),
                            (TokenType::Match, "match"), (TokenType::Where, "where"),
                            (TokenType::Return, "return"), (TokenType::CloseParenthesis, ")"),
                            (TokenType::OpenParenthesis, "(")],
            input: input.to_owned(), position: 0, line: 0, column: 0, lookahead: 0}
    }
    pub  fn  next_token(&mut self) -> LexerResult<Token> {
        
        self.position = self.position + self.lookahead;
        if self.position >= self.input.len() {
            return Err(LexerError("EOF"));
        }
        for c in self.input[self.position..self.input.len()].chars() {
            if c.is_whitespace() {
                self.position += 1;
                continue;
            }
            if c.is_numeric() {
                let mut number_fsm = fsm::number_fsm::make_number_fsm();
                return match number_fsm.run(&self.input[self.position..self.input.len()]) {
                    Some(numlen) =>{
                        self.lookahead = numlen;
                        Ok(Token::new(TokenType::Number, self.position, self.position + numlen, &self.input))
                    } ,
                    None => Err(LexerError("not found")),
                };
            }
            if c.is_alphabetic() {
                for keyword in &self.keywords {
                    match run_keyword_fsm(keyword.0, keyword.1, &self.input, self.position) {
                        Some(tok) => {
                            self.lookahead = tok.size();
                            return Ok(tok)
                        },
                        None => {},
                    }
                }
                let mut identifier_fsm = fsm::identifier_fsm::make_identifier_fsm();
                return match identifier_fsm.run(&self.input[self.position..self.input.len()]) {
                    Some(idlen) =>{
                        self.lookahead = idlen;
                        Ok(Token::new(TokenType::Identifier, self.position, self.position + idlen, &self.input))
                    } ,
                    None => Err(LexerError("not found")),
                };
            }
        }
        Err(LexerError("not found"))
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
        let mut lexer = Lexer::new("thisidmyid or             12.00033e-08");
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
        
        let tres2 = lexer.next_token();
        match tres2 {
            Ok(tok) => assert_eq!(tok.content, "12.00033e-08"),
            Err(_msg) => assert!(false),
        }

        match lexer.next_token() {
            Ok(_tok) => assert!(false),
            Err(msg) => assert_eq!(msg.0, "EOF"),
        }
    }
}