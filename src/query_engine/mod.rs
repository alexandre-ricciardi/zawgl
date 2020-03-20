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
            ast.as_ref().and_then(|ast_root_node| { parser::walk_ast(&mut visitor, ast_root_node); visitor.request })
        }
        Err(value) => None
    }
}

#[derive(PartialEq)]
enum VisitorState {
    Init,
    DirectiveCreate,
    DirectiveMatch,
    Pattern,
    Node,
    RelationshipLR,
    RelationshipRL,
    UndirectedRelationship,
    NodeProperty,
    DirectedRelationshipProperty,
    UnirectedRelationshipProperty
}

enum IdentifierType {
    Variable,
    Label
}

struct CypherAstVisitor {
    request: Option<Request>,
    curr_node: Option<usize>,
    curr_directed_relationship: Option<usize>,
    curr_both_ways_relationship: Option<(usize, usize)>,
    curr_property_id: Option<usize>,
    curr_both_ways_property_ids: Option<(usize, usize)>,
    curr_identifier: Option<String>,
    state: VisitorState,
    id_type: Option<IdentifierType>
}

impl CypherAstVisitor {
    fn new() -> Self {
        CypherAstVisitor { request: None, curr_node: None, curr_directed_relationship: None, curr_both_ways_relationship: None,
            curr_property_id: None, state: VisitorState::Init,curr_both_ways_property_ids: None,
            curr_identifier: None, id_type: None }
    }
}

impl AstVisitor for CypherAstVisitor {
    fn enter_create(&mut self, node: &AstTagNode) {
        self.request = Some(Request::new(Directive::CREATE));
        self.state = VisitorState::DirectiveCreate;
    }
    fn enter_node(&mut self, node: &AstTagNode) {
        
        match self.state {
            VisitorState::DirectiveCreate |
            VisitorState::DirectiveMatch => {
                self.curr_node = self.request.as_mut().map(|req| req.pattern.add_node());
            },
            _ => {}
        }    
        self.state = VisitorState::Node;
    }
    fn enter_relationship(&mut self, node: &AstTagNode) {
        
        let prev_node = self.curr_node;
        self.curr_node = self.request.as_mut().map(|req| req.pattern.add_node());
        let source_target = prev_node.and_then(|p| self.curr_node.map(|c| (p, c)));
        self.curr_directed_relationship = node.ast_tag.and_then(|ast_tag| {
            match ast_tag {
                AstTag::RelDirectedLR => {
                    self.state = VisitorState::RelationshipLR;
                    source_target.and_then(|st| self.request.as_mut().map(|req| req.pattern.add_relationship(st.0, st.1)))
                },
                AstTag::RelDirectedRL => {
                    self.state = VisitorState::RelationshipRL;
                    source_target.and_then(|st| self.request.as_mut().map(|req| req.pattern.add_relationship(st.1, st.0)))
                },
                _ => None
            }
        });
        self.curr_both_ways_relationship = node.ast_tag.and_then(|ast_tag| {
            match ast_tag {
                AstTag::RelUndirected => {
                    self.state = VisitorState::UndirectedRelationship;
                    source_target.and_then(|st| self.request.as_mut().map(|req| (req.pattern.add_relationship(st.0, st.1), req.pattern.add_relationship(st.1, st.0))))
                },
                _ => None
            }
        });
    }
    fn enter_property(&mut self, node: &AstTagNode) {
        match self.state {
            VisitorState::Node => self.state = VisitorState::NodeProperty,
            VisitorState::RelationshipRL |
            VisitorState::RelationshipLR => self.state = VisitorState::DirectedRelationshipProperty,
            VisitorState::UndirectedRelationship => self.state = VisitorState::UnirectedRelationshipProperty,
            _ => {}
        }
        if self.state == VisitorState::DirectedRelationshipProperty {
            if let Some(rel_id) = self.curr_directed_relationship {
                self.curr_property_id = self.request.as_mut().map(|req| {
                    let rel = req.pattern.get_relationship_mut(rel_id);
                    let pvec = &mut rel.properties;
                    pvec.push(Property::new());
                    rel.properties.len() - 1
                })
            }
        }
        if self.state == VisitorState::UnirectedRelationshipProperty {
            if let Some(both_ways) = self.curr_both_ways_relationship {
                self.curr_both_ways_property_ids = self.request.as_mut().map(|req| {
                    let pattern = &mut req.pattern;
                    let mut sizes: (usize, usize) = (0, 0);
                    {
                        let rel0 = pattern.get_relationship_mut(both_ways.0);
                        rel0.properties.push(Property::new());
                        sizes.0 = rel0.properties.len() - 1;
                    }
                    let rel1 = pattern.get_relationship_mut(both_ways.1);
                    rel1.properties.push(Property::new());
                    sizes.1 = rel1.properties.len() - 1;
                    sizes
                })
            }
        }
        if self.state == VisitorState::NodeProperty {
            if let Some(node_id) = self.curr_node {
                self.curr_property_id = self.request.as_mut().map(|req| {
                    let pnode = req.pattern.get_node_mut(node_id);
                    let pvec = &mut pnode.properties;
                    pvec.push(Property::new());
                    pnode.properties.len() - 1
                })
            }
        }
        
            
    }
    fn enter_integer_value(&mut self, value: Option<i64>) {
        if let Some(req) = &mut self.request {
            match self.state {
                VisitorState::DirectedRelationshipProperty => {
                    if let Some(rel_id) = self.curr_directed_relationship {
                        let rel = req.pattern.get_relationship_mut(rel_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.properties[prop_id];
                            prop.value = value.map(|v| PropertyValue::PInteger(v));
                        }
                    }
                },
                VisitorState::NodeProperty => {
                    if let Some(node_id) = self.curr_node {
                        let node = req.pattern.get_node_mut(node_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut node.properties[prop_id];
                            prop.value = value.map(|v| PropertyValue::PInteger(v));
                        }
                    }
                },
                VisitorState::UnirectedRelationshipProperty => {
                    if let Some(rel_ids) = self.curr_both_ways_relationship {
                        {
                            let rel = req.pattern.get_relationship_mut(rel_ids.0);
                            if let Some(prop_id) = self.curr_property_id {
                                let prop = &mut rel.properties[prop_id];
                                prop.value = value.map(|v| PropertyValue::PInteger(v));
                            }
                        }
                        let rel = req.pattern.get_relationship_mut(rel_ids.1);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.properties[prop_id];
                            prop.value = value.map(|v| PropertyValue::PInteger(v));
                        }
                    }
                },
                _ => {}
            }
        }
    }
    fn enter_float_value(&mut self, value: Option<f64>) {
        if let Some(req) = &mut self.request {
            match self.state {
                VisitorState::DirectedRelationshipProperty => {
                    if let Some(rel_id) = self.curr_directed_relationship {
                        let rel = req.pattern.get_relationship_mut(rel_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.properties[prop_id];
                            prop.value = value.map(|v| PropertyValue::PFloat(v));
                        }
                    }
                },
                VisitorState::NodeProperty => {
                    if let Some(node_id) = self.curr_node {
                        let node = req.pattern.get_node_mut(node_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut node.properties[prop_id];
                            prop.value = value.map(|v| PropertyValue::PFloat(v));
                        }
                    }
                },
                VisitorState::UnirectedRelationshipProperty => {
                    if let Some(rel_ids) = self.curr_both_ways_relationship {
                        {
                            let rel = req.pattern.get_relationship_mut(rel_ids.0);
                            if let Some(prop_id) = self.curr_property_id {
                                let prop = &mut rel.properties[prop_id];
                                prop.value = value.map(|v| PropertyValue::PFloat(v));
                            }
                        }
                        let rel = req.pattern.get_relationship_mut(rel_ids.1);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.properties[prop_id];
                            prop.value = value.map(|v| PropertyValue::PFloat(v));
                        }
                    }
                },
                _ => {}
            }
        }
    }
    fn enter_string_value(&mut self, value: &str) {
        if let Some(req) = &mut self.request {
            match self.state {
                VisitorState::DirectedRelationshipProperty => {
                    if let Some(rel_id) = self.curr_directed_relationship {
                        let rel = req.pattern.get_relationship_mut(rel_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.properties[prop_id];
                            prop.value = Some(PropertyValue::PString(String::from(value)));
                        }
                    }
                },
                VisitorState::NodeProperty => {
                    if let Some(node_id) = self.curr_node {
                        let node = req.pattern.get_node_mut(node_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut node.properties[prop_id];
                            prop.value = Some(PropertyValue::PString(String::from(value)));
                        }
                    }
                },
                VisitorState::UnirectedRelationshipProperty => {
                    if let Some(rel_ids) = self.curr_both_ways_relationship {
                        {
                            let rel = req.pattern.get_relationship_mut(rel_ids.0);
                            if let Some(prop_id) = self.curr_property_id {
                                let prop = &mut rel.properties[prop_id];
                                prop.value = Some(PropertyValue::PString(String::from(value)));
                            }
                        }
                        let rel = req.pattern.get_relationship_mut(rel_ids.1);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.properties[prop_id];
                            prop.value = Some(PropertyValue::PString(String::from(value)));
                        }
                    }
                },
                _ => {}
            }
        }
    }
    fn enter_bool_value(&mut self, value: Option<bool>) {
        if let Some(req) = &mut self.request {
            match self.state {
                VisitorState::DirectedRelationshipProperty => {
                    if let Some(rel_id) = self.curr_directed_relationship {
                        let rel = req.pattern.get_relationship_mut(rel_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.properties[prop_id];
                            prop.value = value.map(|v| PropertyValue::PBool(v));
                        }
                    }
                },
                VisitorState::NodeProperty => {
                    if let Some(node_id) = self.curr_node {
                        let node = req.pattern.get_node_mut(node_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut node.properties[prop_id];
                            prop.value = value.map(|v| PropertyValue::PBool(v));
                        }
                    }
                },
                VisitorState::UnirectedRelationshipProperty => {
                    if let Some(rel_ids) = self.curr_both_ways_relationship {
                        {
                            let rel = req.pattern.get_relationship_mut(rel_ids.0);
                            if let Some(prop_id) = self.curr_property_id {
                                let prop = &mut rel.properties[prop_id];
                                prop.value = value.map(|v| PropertyValue::PBool(v));
                            }
                        }
                        let rel = req.pattern.get_relationship_mut(rel_ids.1);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.properties[prop_id];
                            prop.value = value.map(|v| PropertyValue::PBool(v));
                        }
                    }
                },
                _ => {}
            }
        }
    }

    fn enter_label(&mut self) {
        self.id_type = Some(IdentifierType::Label);
    }

    fn enter_variable(&mut self) {
        self.id_type = Some(IdentifierType::Variable);
    }

    fn enter_identifier(&mut self, key: &str) {
        self.curr_identifier = Some(String::from(key));
        if let Some(req) = &mut self.request {
            match self.state {
                VisitorState::Node => {
                    if let Some(node_id) = self.curr_node {
                        let node = req.pattern.get_node_mut(node_id);
                        match self.id_type {
                            Some(IdentifierType::Variable) => {
                                node.var = Some(String::from(key));
                            },
                            Some(IdentifierType::Label) => {
                                node.labels.push(String::from(key));
                            },
                            _ => {}
                        } 
                    }
                },
                VisitorState::RelationshipRL |
                VisitorState::RelationshipLR => {
                    if let Some(rel_id) = self.curr_directed_relationship {
                        let rel = req.pattern.get_relationship_mut(rel_id);
                        
                        match self.id_type {
                            Some(IdentifierType::Variable) => {
                                rel.var = Some(String::from(key));
                            },
                            Some(IdentifierType::Label) => {
                                rel.labels.push(String::from(key));
                            },
                            _ => {}
                        } 
                    }
                },
                VisitorState::UndirectedRelationship => {
                    if let Some(rel_ids) = self.curr_both_ways_relationship {
                        {
                            let rel = req.pattern.get_relationship_mut(rel_ids.0);
                            match self.id_type {
                                Some(IdentifierType::Variable) => {
                                    rel.var = Some(String::from(key));
                                },
                                Some(IdentifierType::Label) => {
                                    rel.labels.push(String::from(key));
                                },
                                _ => {}
                            } 
                        }
                        let rel = req.pattern.get_relationship_mut(rel_ids.1);
                        match self.id_type {
                            Some(IdentifierType::Variable) => {
                                rel.var = Some(String::from(key));
                            },
                            Some(IdentifierType::Label) => {
                                rel.labels.push(String::from(key));
                            },
                            _ => {}
                        } 
                        
                    }
                },
                VisitorState::DirectedRelationshipProperty => {
                    if let Some(rel_id) = self.curr_directed_relationship {
                        let rel = req.pattern.get_relationship_mut(rel_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.properties[prop_id];
                            prop.name = Some(String::from(key));
                        }
                    }
                },
                VisitorState::NodeProperty => {
                    if let Some(node_id) = self.curr_node {
                        let node = req.pattern.get_node_mut(node_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut node.properties[prop_id];
                            prop.name = Some(String::from(key));
                        }
                    }
                },
                VisitorState::UnirectedRelationshipProperty => {
                    if let Some(rel_ids) = self.curr_both_ways_relationship {
                        {
                            let rel = req.pattern.get_relationship_mut(rel_ids.0);
                            if let Some(prop_id) = self.curr_property_id {
                                let prop = &mut rel.properties[prop_id];
                                prop.name = Some(String::from(key));
                            }
                        }
                        let rel = req.pattern.get_relationship_mut(rel_ids.1);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.properties[prop_id];
                            prop.name = Some(String::from(key));
                        }
                    }
                },
                _ => {}
            }
        }
        
    }
}

#[cfg(test)]
mod test_query_engine {
    use super::*;

    #[test]
    fn test_create() {
        let request = process_query("CREATE (n:Person)");
        if let  Some(req) = request {
            let node = req.pattern.get_node_ref(0);
            assert_eq!(node.var, Some(String::from("n")));
            assert_eq!(node.labels[0], String::from("Person"));
        } else {
            assert!(false, "no request found");
        }
        
    }
}