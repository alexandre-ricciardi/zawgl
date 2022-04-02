use one_graph_core::graph::traits::GraphContainerTrait;
use one_graph_core::graph::*;
use one_graph_core::model::*;
use super::states::*;
use super::parser::*;
pub struct PathBuilder {
    curr_node: Option<NodeIndex>,
    curr_directed_relationship: Option<EdgeIndex>,
    curr_both_ways_relationship: Option<(EdgeIndex, EdgeIndex)>,
    curr_identifier: Option<String>,
    pattern_state: VisitorPatternState,
    id_type: Option<IdentifierType>,
    curr_property_name: Option<String>,
    current_path: PropertyGraph,
    visitor_state: VisitorState
}

impl PathBuilder {
    pub fn new(state: VisitorState) -> Self {
        PathBuilder {curr_node: None, curr_directed_relationship: None, curr_both_ways_relationship: None,
            visitor_state: state, pattern_state: VisitorPatternState::Init,
            curr_identifier: None, id_type: None, curr_property_name: None, current_path: PropertyGraph::new() }
    }

    pub fn set_property_value(&mut self, property_value: Option<PropertyValue>) {
        match self.pattern_state {
            VisitorPatternState::DirectedRelationshipProperty |
            VisitorPatternState::UndirectedRelationshipProperty => {
                if let Some(rel_id) = self.curr_directed_relationship {
                    let rel = self.current_path.get_relationship_mut(&rel_id);
                    if let (Some(name), Some(value)) = (&self.curr_property_name, property_value) {
                        rel.get_properties_mut().push(Property::new(name.clone(), value))
                    }
                }
            },
            VisitorPatternState::NodeProperty => {
                if let Some(node_id) = self.curr_node {
                    let node = self.current_path.get_node_mut(&node_id);
                    if let (Some(name), Some(value)) = (&self.curr_property_name, property_value) {
                        node.get_properties_mut().push(Property::new(name.clone(), value.clone()))
                    }
                }
            }
            _ => {}
        }
    }

    pub fn enter_node(&mut self) {
        let mut n = Node::new();
        match self.visitor_state {
            VisitorState::CreatePattern => {
                n.set_status(Status::Create);
            }
            VisitorState::MatchPattern => {
                n.set_status(Status::Match);
            },
            _ => {}
        }
        self.curr_node = Some(self.current_path.add_node(n));
        self.pattern_state = VisitorPatternState::Node;
    }
    
    pub fn enter_relationship(&mut self, ast_tag: AstTag) {
        
        let prev_node = self.curr_node;
        let pnode = Node::new();
        self.curr_node = Some(self.current_path.add_node(pnode));
        let source_target = prev_node.and_then(|p| self.curr_node.map(|c| (p, c)));

        match ast_tag {
            AstTag::RelDirectedLR => {
                self.pattern_state = VisitorPatternState::RelationshipLR;
                self.curr_directed_relationship = source_target.map(|st| self.current_path.add_relationship(Relationship::new(), st.0, st.1))
            }
            AstTag::RelDirectedRL => {
                self.pattern_state = VisitorPatternState::RelationshipRL;
                self.curr_directed_relationship = source_target.map(|st| self.current_path.add_relationship(Relationship::new(), st.1, st.0))
            }
            AstTag::RelUndirected => {
                self.pattern_state = VisitorPatternState::UndirectedRelationship;
                self.curr_both_ways_relationship = source_target.map(|st| (self.current_path.add_relationship(Relationship::new(), st.0, st.1), self.current_path.add_relationship(Relationship::new(), st.1, st.0)));
            }
            _ => {}
        }
    }

    pub fn enter_property(&mut self) {
        match self.pattern_state {
            VisitorPatternState::Node => self.pattern_state = VisitorPatternState::NodeProperty,
            VisitorPatternState::RelationshipRL |
            VisitorPatternState::RelationshipLR => self.pattern_state = VisitorPatternState::DirectedRelationshipProperty,
            VisitorPatternState::UndirectedRelationship => self.pattern_state = VisitorPatternState::UndirectedRelationshipProperty,
            _ => {}
        }
    }

    pub fn enter_integer_value(&mut self, value: Option<i64>) {
        let pv = value.map(|v| PropertyValue::PInteger(v));
        self.set_property_value(pv);
    }
    pub fn enter_float_value(&mut self, value: Option<f64>) {
        let pv = value.map(|v| PropertyValue::PFloat(v));
        self.set_property_value(pv);
    }
    pub fn enter_string_value(&mut self, value: Option<&str>) {
        let pv = value.map(|sv|PropertyValue::PString(String::from(sv)));
        self.set_property_value(pv);
    }
    pub fn enter_bool_value(&mut self, value: Option<bool>) {
        let pv = value.map(|v| PropertyValue::PBool(v));
        self.set_property_value(pv);
    }
}
