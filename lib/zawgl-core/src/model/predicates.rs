use super::*;

#[derive(Debug, Clone)]
pub enum PropertyPredicate {
    GreaterThan(PropertyValue),
    GeaterOrEqualTo(PropertyValue),
    LessThan(PropertyValue),
    LessOrEqualTo(PropertyValue),
    EqualTo(PropertyValue),
    Contain(Vec<PropertyValue>),
}

#[derive(Debug, Clone)]
pub struct NamedPropertyPredicate {
    pub name: String,
    pub predicate: PropertyPredicate,
}

impl NamedPropertyPredicate {
    pub fn new(name: &str, predicate: PropertyPredicate) -> Self {
        NamedPropertyPredicate{ name: String::from(name), predicate: predicate}
    }
}

impl PropertyPredicate {
    pub fn eval(&self, value: &PropertyValue) -> bool {
        match &self {
            PropertyPredicate::GreaterThan(v) => {
                v < value
            },
            PropertyPredicate::GeaterOrEqualTo(v) => {
                v <= value
            },
            PropertyPredicate::LessThan(v) => v > value,
            PropertyPredicate::LessOrEqualTo(v) => v >= value,
            PropertyPredicate::EqualTo(v) => v == value,
            PropertyPredicate::Contain(list) => list.contains(value),
        }
    }
}