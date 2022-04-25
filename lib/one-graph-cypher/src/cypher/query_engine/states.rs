#[derive(PartialEq, Clone, Copy)]
pub enum VisitorState {
    Init,
    DirectiveCreate,
    DirectiveMatch,
    MatchPattern,
    CreatePattern,
    FunctionCall,
    FunctionArg,
    ReturnItem,
}
#[derive(PartialEq)]
pub enum VisitorPatternState {
    Init,
    Node,
    RelationshipLR,
    RelationshipRL,
    UndirectedRelationship,
    NodeProperty,
    DirectedRelationshipProperty,
    UndirectedRelationshipProperty,
}

pub enum IdentifierType {
    Variable,
    Label
}