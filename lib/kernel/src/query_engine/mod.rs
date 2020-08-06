use super::cypher::*;
use super::model::*;
use super::cypher::parser::*;
use super::graph::traits::{GraphContainerTrait};
use super::graph::*;

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
    curr_node: Option<NodeIndex>,
    curr_directed_relationship: Option<EdgeIndex>,
    curr_both_ways_relationship: Option<(EdgeIndex, EdgeIndex)>,
    curr_property_id: Option<usize>,
    curr_both_ways_property_ids: Option<(usize, usize)>,
    curr_identifier: Option<String>,
    state: VisitorState,
    id_type: Option<IdentifierType>
}

impl <'g> CypherAstVisitor {
    fn new() -> Self {
        CypherAstVisitor { request: None, curr_node: None, curr_directed_relationship: None, curr_both_ways_relationship: None,
            curr_property_id: None, state: VisitorState::Init, curr_both_ways_property_ids: None,
            curr_identifier: None, id_type: None }
    }
}

impl <'g> AstVisitor<'g> for CypherAstVisitor {
    fn enter_create(&mut self, node: &AstTagNode) {
        self.request = Some(Request::new(Directive::CREATE));
        self.state = VisitorState::DirectiveCreate;
    }
    fn enter_match(&mut self, node: &AstTagNode) {
        self.request = Some(Request::new(Directive::MATCH));
        self.state = VisitorState::DirectiveMatch;
    }
    fn enter_node(&mut self, node: &AstTagNode) {
        
        match self.state {
            VisitorState::DirectiveCreate |
            VisitorState::DirectiveMatch => {
                let node = Node::new();
                self.curr_node = self.request.as_mut().map(|req| req.pattern.add_node(node));
            },
            _ => {}
        }    
        self.state = VisitorState::Node;
    }
    fn enter_relationship(&mut self, node: &AstTagNode) {
        
        let prev_node = self.curr_node;
        let pnode = Node::new();
        self.curr_node = self.request.as_mut().map(|req| req.pattern.add_node(pnode));
        let source_target = prev_node.and_then(|p| self.curr_node.map(|c| (p, c)));
        self.curr_directed_relationship = node.ast_tag.and_then(|ast_tag| {
            match ast_tag {
                AstTag::RelDirectedLR => {
                    self.state = VisitorState::RelationshipLR;
                    source_target.and_then(|st| self.request.as_mut().map(|req| req.pattern.add_relationship(Relationship::new(), st.0, st.1)))
                },
                AstTag::RelDirectedRL => {
                    self.state = VisitorState::RelationshipRL;
                    source_target.and_then(|st| self.request.as_mut().map(|req| req.pattern.add_relationship(Relationship::new(), st.1, st.0)))
                },
                _ => None
            }
        });
        self.curr_both_ways_relationship = node.ast_tag.and_then(|ast_tag| {
            match ast_tag {
                AstTag::RelUndirected => {
                    self.state = VisitorState::UndirectedRelationship;
                    source_target.and_then(|st| self.request.as_mut().map(|req| (req.pattern.add_relationship(Relationship::new(), st.0, st.1), req.pattern.add_relationship(Relationship::new(), st.1, st.0))))
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
                    let rel = req.pattern.get_relationship_mut(&rel_id);
                    let pvec = &mut rel.get_properties_mut();
                    pvec.push(Property::new());
                    rel.get_properties_ref().len() - 1
                })
            }
        }
        if self.state == VisitorState::UnirectedRelationshipProperty {
            if let Some(both_ways) = self.curr_both_ways_relationship {
                self.curr_both_ways_property_ids = self.request.as_mut().map(|req| {
                    let pattern = &mut req.pattern;
                    let mut sizes: (usize, usize) = (0, 0);
                    {
                        let rel0 = pattern.get_relationship_mut(&both_ways.0);
                        rel0.get_properties_mut().push(Property::new());
                        sizes.0 = rel0.get_properties_ref().len() - 1;
                    }
                    let rel1 = pattern.get_relationship_mut(&both_ways.1);
                    rel1.get_properties_mut().push(Property::new());
                    sizes.1 = rel1.get_properties_ref().len() - 1;
                    sizes
                })
            }
        }
        if self.state == VisitorState::NodeProperty {
            if let Some(node_id) = self.curr_node {
                self.curr_property_id = self.request.as_mut().map(|req| {
                    let pnode = req.pattern.get_node_mut(&node_id);
                    let pvec = pnode.get_properties_mut();
                    pvec.push(Property::new());
                    pvec.len() - 1
                })
            }
        }
        
            
    }
    fn enter_integer_value(&mut self, value: Option<i64>) {
        if let Some(req) = &mut self.request {
            match self.state {
                VisitorState::DirectedRelationshipProperty => {
                    if let Some(rel_id) = self.curr_directed_relationship {
                        let rel = req.pattern.get_relationship_mut(&rel_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.get_properties_mut()[prop_id];
                            prop.set_value(value.map(|v| PropertyValue::PInteger(v)));
                        }
                    }
                },
                VisitorState::NodeProperty => {
                    if let Some(node_id) = self.curr_node {
                        let node = req.pattern.get_node_mut(&node_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut node.get_properties_mut()[prop_id];
                            prop.set_value(value.map(|v| PropertyValue::PInteger(v)));
                        }
                    }
                },
                VisitorState::UnirectedRelationshipProperty => {
                    if let Some(rel_ids) = self.curr_both_ways_relationship {
                        {
                            let rel = req.pattern.get_relationship_mut(&rel_ids.0);
                            if let Some(prop_id) = self.curr_property_id {
                                let prop = &mut rel.get_properties_mut()[prop_id];
                                prop.set_value(value.map(|v| PropertyValue::PInteger(v)));
                            }
                        }
                        let rel = req.pattern.get_relationship_mut(&rel_ids.1);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.get_properties_mut()[prop_id];
                            prop.set_value(value.map(|v| PropertyValue::PInteger(v)));
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
                        let rel = req.pattern.get_relationship_mut(&rel_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.get_properties_mut()[prop_id];
                            prop.set_value(value.map(|v| PropertyValue::PFloat(v)));
                        }
                    }
                },
                VisitorState::NodeProperty => {
                    if let Some(node_id) = self.curr_node {
                        let node = req.pattern.get_node_mut(&node_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut node.get_properties_mut()[prop_id];
                            prop.set_value(value.map(|v| PropertyValue::PFloat(v)));
                        }
                    }
                },
                VisitorState::UnirectedRelationshipProperty => {
                    if let Some(rel_ids) = self.curr_both_ways_relationship {
                        {
                            let rel = req.pattern.get_relationship_mut(&rel_ids.0);
                            if let Some(prop_id) = self.curr_property_id {
                                let prop = &mut rel.get_properties_mut()[prop_id];
                                prop.set_value(value.map(|v| PropertyValue::PFloat(v)));
                            }
                        }
                        let rel = req.pattern.get_relationship_mut(&rel_ids.1);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.get_properties_mut()[prop_id];
                            prop.set_value(value.map(|v| PropertyValue::PFloat(v)));
                        }
                    }
                },
                _ => {}
            }
        }
    }
    fn enter_string_value(&mut self, value: Option<&str>) {
        if let Some(req) = &mut self.request {
            match self.state {
                VisitorState::DirectedRelationshipProperty => {
                    if let Some(rel_id) = self.curr_directed_relationship {
                        let rel = req.pattern.get_relationship_mut(&rel_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.get_properties_mut()[prop_id];
                            prop.set_value(value.map(|sv|PropertyValue::PString(String::from(sv))));
                        }
                    }
                },
                VisitorState::NodeProperty => {
                    if let Some(node_id) = self.curr_node {
                        let node = req.pattern.get_node_mut(&node_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut node.get_properties_mut()[prop_id];
                            prop.set_value(value.map(|sv|PropertyValue::PString(String::from(sv))));
                        }
                    }
                },
                VisitorState::UnirectedRelationshipProperty => {
                    if let Some(rel_ids) = self.curr_both_ways_relationship {
                        {
                            let rel = req.pattern.get_relationship_mut(&rel_ids.0);
                            if let Some(prop_id) = self.curr_property_id {
                                let prop = &mut rel.get_properties_mut()[prop_id];
                                prop.set_value(value.map(|sv|PropertyValue::PString(String::from(sv))));
                            }
                        }
                        let rel = req.pattern.get_relationship_mut(&rel_ids.1);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.get_properties_mut()[prop_id];
                            prop.set_value(value.map(|sv|PropertyValue::PString(String::from(sv))));
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
                        let rel = req.pattern.get_relationship_mut(&rel_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.get_properties_mut()[prop_id];
                            prop.set_value(value.map(|v| PropertyValue::PBool(v)));
                        }
                    }
                },
                VisitorState::NodeProperty => {
                    if let Some(node_id) = self.curr_node {
                        let node = req.pattern.get_node_mut(&node_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut node.get_properties_mut()[prop_id];
                            prop.set_value(value.map(|v| PropertyValue::PBool(v)));
                        }
                    }
                },
                VisitorState::UnirectedRelationshipProperty => {
                    if let Some(rel_ids) = self.curr_both_ways_relationship {
                        {
                            let rel = req.pattern.get_relationship_mut(&rel_ids.0);
                            if let Some(prop_id) = self.curr_property_id {
                                let prop = &mut rel.get_properties_mut()[prop_id];
                                prop.set_value(value.map(|v| PropertyValue::PBool(v)));
                            }
                        }
                        let rel = req.pattern.get_relationship_mut(&rel_ids.1);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.get_properties_mut()[prop_id];
                            prop.set_value(value.map(|v| PropertyValue::PBool(v)));
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
                        let node = req.pattern.get_node_mut(&node_id);
                        match self.id_type {
                            Some(IdentifierType::Variable) => {
                                node.set_var(key);
                            },
                            Some(IdentifierType::Label) => {
                                node.get_labels_mut().push(String::from(key));
                            },
                            _ => {}
                        } 
                    }
                },
                VisitorState::RelationshipRL |
                VisitorState::RelationshipLR => {
                    if let Some(rel_id) = self.curr_directed_relationship {
                        let rel = req.pattern.get_relationship_mut(&rel_id);
                        
                        match self.id_type {
                            Some(IdentifierType::Variable) => {
                                rel.set_var(key);
                            },
                            Some(IdentifierType::Label) => {
                                rel.get_labels_mut().push(String::from(key));
                            },
                            _ => {}
                        } 
                    }
                },
                VisitorState::UndirectedRelationship => {
                    if let Some(rel_ids) = self.curr_both_ways_relationship {
                        {
                            let rel = req.pattern.get_relationship_mut(&rel_ids.0);
                            match self.id_type {
                                Some(IdentifierType::Variable) => {
                                    rel.set_var(key);
                                },
                                Some(IdentifierType::Label) => {
                                    rel.get_labels_mut().push(String::from(key));
                                },
                                _ => {}
                            } 
                        }
                        let rel = req.pattern.get_relationship_mut(&rel_ids.1);
                        match self.id_type {
                            Some(IdentifierType::Variable) => {
                                rel.set_var(key);
                            },
                            Some(IdentifierType::Label) => {
                                rel.get_labels_mut().push(String::from(key));
                            },
                            _ => {}
                        } 
                        
                    }
                },
                VisitorState::DirectedRelationshipProperty => {
                    if let Some(rel_id) = self.curr_directed_relationship {
                        let rel = req.pattern.get_relationship_mut(&rel_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.get_properties_mut()[prop_id];
                            prop.set_name(key);
                        }
                    }
                },
                VisitorState::NodeProperty => {
                    if let Some(node_id) = self.curr_node {
                        let node = req.pattern.get_node_mut(&node_id);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut node.get_properties_mut()[prop_id];
                            prop.set_name(key);
                        }
                    }
                },
                VisitorState::UnirectedRelationshipProperty => {
                    if let Some(rel_ids) = self.curr_both_ways_relationship {
                        {
                            let rel = req.pattern.get_relationship_mut(&rel_ids.0);
                            if let Some(prop_id) = self.curr_property_id {
                                let prop = &mut rel.get_properties_mut()[prop_id];
                                prop.set_name(key);
                            }
                        }
                        let rel = req.pattern.get_relationship_mut(&rel_ids.1);
                        if let Some(prop_id) = self.curr_property_id {
                            let prop = &mut rel.get_properties_mut()[prop_id];
                            prop.set_name(key);
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
    fn test_create_0() {
        let request = process_query("CREATE (n:Person)");
        if let  Some(req) = request {
            let node = req.pattern.get_node_ref(&NodeIndex::new(0));
            assert_eq!(node.get_var(), &Some(String::from("n")));
            assert_eq!(node.get_labels_ref()[0], String::from("Person"));
        } else {
            assert!(false, "no request found");
        }
        
    }

    #[test]
    fn test_create_1() {
        let request = process_query("CREATE (n:Person:Parent {test: 'Hello', case: 4.99})");
        if let  Some(req) = request {
            let node = req.pattern.get_node_ref(&NodeIndex::new(0));
            assert_eq!(node.get_var(), &Some(String::from("n")));
            assert_eq!(node.get_labels_ref()[0], String::from("Person"));
            assert_eq!(node.get_labels_ref()[1], String::from("Parent"));
            assert_eq!(node.get_properties_ref()[0].get_name(), &Some(String::from("test")));
            assert_eq!(node.get_properties_ref()[0].get_value(), &Some(PropertyValue::PString(String::from("Hello"))));
            assert_eq!(node.get_properties_ref()[1].get_name(), &Some(String::from("case")));
            //assert_eq!(node.properties[1].value, Some(PropertyValue::PFloat(4.99)));
            
        } else {
            assert!(false, "no request found");
        }
        
    }

    #[test]
    fn test_create_2() {
        let request = process_query("CREATE (n:Person:Parent)-[r:FRIEND_OF]->(p:Person)");
        if let  Some(req) = request {
            let node = req.pattern.get_node_ref(&NodeIndex::new(0));
            assert_eq!(node.get_var(), &Some(String::from("n")));
            assert_eq!(node.get_labels_ref()[0], String::from("Person"));
            let rel = req.pattern.get_relationship_ref(&EdgeIndex::new(0));
            assert_eq!(rel.get_var(), &Some(String::from("r")));
            assert_eq!(rel.get_labels_ref()[0], String::from("FRIEND_OF"));
            
            
        } else {
            assert!(false, "no request found");
        }
        
    }
}