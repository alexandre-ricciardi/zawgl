use super::cypher::*;
use super::model::*;
use super::cypher::parser::*;

pub fn process_query(query: &str) -> Option<Request> {
    let mut lexer = lexer::Lexer::new(query);
    match lexer.get_tokens() {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(tokens);
            let ast = parser::cypher_parser::parse(&mut parser).ok();
            let mut visitor = CypherAstVisitor::new();
            ast.as_ref().and_then(|ast_root_node| { ast_root_node.accept(&mut visitor); visitor.request })
        }
        Err(value) => None
    }
}

struct CypherAstVisitor {
    request: Option<Request>,
    curr_node: Option<Node>,
    curr_relationship: Option<Relationship>,
    curr_relationship_ast_tag: Option<AstTag>
}

impl CypherAstVisitor {
    fn new() -> Self {
        CypherAstVisitor { request: None, curr_node: None, curr_relationship: None, curr_relationship_ast_tag: None }
    }
}

impl AstVisitor for CypherAstVisitor {
    fn enter_create(&mut self, node: &AstTagNode) {
        self.request = Some(Request::new(Directive::CREATE));
    }
    fn enter_node(&mut self, node: &AstTagNode) {
        let prev_node = self.curr_node;
        self.curr_node = self.request.as_mut().map(|req| req.pattern.add_node());
        self.curr_relationship = self.request.as_mut().and_then(|req| {
            self.curr_relationship_ast_tag.map(|ast_tag| {
                match ast_tag {
                    AstTag::RelDirectedLR => {
                        req.pattern.add_relationship(prev_node.unwrap().id, self.curr_node.unwrap().id)
                    }
                }
            })
            
        });
    }
    fn enter_relationship(&mut self, node: &AstTagNode) {
        self.curr_relationship_ast_tag = node.ast_tag;
    }
    fn enter_property(&mut self, node: &AstTagNode) {

    }
}

#[cfg(test)]
mod test_query_engine {
    use super::*;

    #[test]
    fn test_create() {
        let req = process_query("CREATE (n:Person)");
    }
}