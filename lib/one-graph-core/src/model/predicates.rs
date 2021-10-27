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