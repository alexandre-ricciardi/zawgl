pub enum Step {
    AddV(AddVStep),
    V(MatchVStep),
    Has(HasPropertyStep),
    Empty,
}

pub enum GValue {
    Integer(i32),
    Long(i64),
    String(String),
    Boolean(bool),
}

pub struct GList {
    pub values: Vec<GValue>,
}

pub struct AddVStep {
    pub label: String,
}

pub struct MatchVStep {
    pub vid: String,
}

pub struct HasPropertyStep {
    pub property_name: String,
    pub predicate: Predicate,
}

pub enum Predicate {
    Value(String),
    Within(WithinPredicate),
}

pub struct WithinPredicate {
    pub value: Vec<GValue>,
}

pub struct Gremlin {
    pub steps: Vec<Step>,
}