pub mod error;
pub mod parser_utils;
pub mod pattern_parser_delegate;
pub mod properties_parser_delegate;
pub mod cypher_parser;

use super::lexer::*;
use self::error::*;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum AstTag  {
    Create,
    Node,
    Property,
    RelDirectedLR,
    RelDirectedRL,
    RelUndirected,
    Variable,
    Label,
}

pub trait AstVisitor {
    fn enter_create(&mut self, node: &AstTagNode);
    fn enter_node(&mut self, node: &AstTagNode);
    fn enter_relationship(&mut self, node: &AstTagNode);
    fn enter_property(&mut self, node: &AstTagNode);
    fn enter_integer_value(&mut self, value: Option<i64>);
    fn enter_float_value(&mut self, value: Option<f64>);
    fn enter_string_value(&mut self, value: &str);
    fn enter_bool_value(&mut self, value: Option<bool>);
    fn enter_identifier(&mut self, key: &str);
    fn enter_variable(&mut self);
    fn enter_label(&mut self);
}

pub trait Ast : fmt::Display {
    fn append(&mut self, ast: Box<dyn Ast>);
    fn accept(&self, visitor: &mut dyn AstVisitor);
    fn get_childs(&self) -> &Vec<Box<dyn Ast>>;
}

pub struct AstTokenNode {
    pub token_id: usize,
    pub token_value: String,
    pub childs: Vec<Box<dyn Ast>>,
    pub token_type: TokenType,
}

pub struct AstTagNode {
    pub ast_tag: Option<AstTag>,
    pub childs: Vec<Box<dyn Ast>>,
}

impl AstTagNode {
    pub fn new_empty() -> Self {
        AstTagNode {childs: Vec::new(), ast_tag: None}
    }
    pub fn new_tag(ast_tag: AstTag) -> Self {
        AstTagNode {childs: Vec::new(), ast_tag: Some(ast_tag)}
    }
}

impl Ast for AstTagNode {
    fn append(&mut self, ast: Box<dyn Ast>) {
        self.childs.push(ast)    
    }
    fn get_childs(&self) -> &Vec<Box<dyn Ast>> {
        &self.childs
    }
    fn accept(&self, visitor: &mut dyn AstVisitor) {
        match self.ast_tag.as_ref() {
            Some(ast_tag) => {
                match ast_tag {
                    AstTag::Create => {
                        visitor.enter_create(&self);
                    },
                    AstTag::RelDirectedLR |
                    AstTag::RelDirectedRL |
                    AstTag::RelUndirected => {
                        visitor.enter_relationship(&self);
                    },
                    AstTag::Node => {
                        visitor.enter_node(&self);
                    },
                    AstTag::Property => {
                        visitor.enter_property(&self);
                    },
                    AstTag::Variable => {
                        visitor.enter_variable();
                    },
                    AstTag::Label => {
                        visitor.enter_label();
                    }
                    _ => {}
                }
            },
            None => {}
        }
        
    }
}

impl AstTokenNode {
    pub fn new_token(token_id: usize, token_value: String, token_type: TokenType) -> Self {
        AstTokenNode {token_id: token_id, token_value: token_value, childs: Vec::new(), token_type: token_type}
    }
}

impl Ast for AstTokenNode {
    fn append(&mut self, ast: Box<dyn Ast>) {
        self.childs.push(ast)    
    }
    fn get_childs(&self) -> &Vec<Box<dyn Ast>> {
        &self.childs
    }
    fn accept(&self, visitor: &mut dyn AstVisitor) {
        match self.token_type {
            TokenType::StringType => {
                visitor.enter_string_value(&self.token_value);
            },
            TokenType::Float => {
                let res = self.token_value.parse::<f64>().ok();
                visitor.enter_float_value(res);
            },
            TokenType::Integer => {
                let res = self.token_value.parse::<i64>().ok();
                visitor.enter_integer_value(res);
            },
            TokenType::True |
            TokenType::False => {
                let res = self.token_value.parse::<bool>().ok();
                visitor.enter_bool_value(res);
            },
            TokenType::Identifier => visitor.enter_identifier(&self.token_value),
            _ => {}
        }
    }
}

impl fmt::Display for AstTokenNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{}", self.token_type, self.token_value)
    }
}

impl fmt::Display for AstTagNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.ast_tag.as_ref() {
            Some(tag) => {
                write!(f, "{:?}", tag)
            },
            _ => write!(f, "")
        }
        
    }
}

pub fn walk_ast(visitor: &mut dyn AstVisitor, ast: &Box<dyn Ast>) {
    ast.accept(visitor);
    for child in ast.get_childs() {
        walk_ast(visitor, &child);
    }
}

pub struct Parser  {
    tokens: Vec<Token>,
    pub index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {tokens : tokens, index: 0}
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    } 

    pub fn require(&mut self, token_type: TokenType) -> ParserResult<usize> {
        if !self.check(token_type) {
            return Err(ParserError::SyntaxError);
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
        if self.tokens[self.index].token_type == token_type {
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
        self.tokens[self.index].token_type == token_type
    }

    pub fn next_token_type(& self, token_type: TokenType) -> bool {
        self.tokens.len() > self.index && self.tokens[self.index + 1].token_type == token_type
    }

}


fn make_ast_token(parser: &Parser) -> Box<AstTokenNode> {
    let token_id = parser.index - 1;
    let token = &parser.get_tokens()[token_id];
    Box::new(AstTokenNode::new_token(token_id, token.content.to_owned(), token.token_type ))
}


#[cfg(test)]
mod test_parser {
    use super::*;

    fn run(qry: &str) {
        let mut lexer = Lexer::new(qry);
        match lexer.get_tokens() {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                let root = cypher_parser::parse(&mut parser);
                parser_utils::print_node(&root.unwrap(), parser.get_tokens(), 0);
            },
            Err(value) => assert!(false)
        }
    }

    #[test]
    fn test_create() {
        run("CREATE (n:Person)");
    }
    #[test]
    fn test_create_labels() {
        run("CREATE (n:Person:Friend:Etc)");
    }

    #[test]
    fn test_create_graph() {
        run("CREATE (n:Person)-[r:FRIEND_OF]->(m:Person)");
    }

    #[test]
    fn test_properties_node() {
        run("CREATE (n:Person { name: 'hello', value: 'world' })");
    }

}

