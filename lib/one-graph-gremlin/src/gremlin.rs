pub enum Step {
    AddV(AddVStep),
    V(MatchVStep),
    Has(HasPropertyStep),

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
    pub value: Vec<String>,
}

pub struct Gremlin {
    pub steps: Vec<Step>,
}