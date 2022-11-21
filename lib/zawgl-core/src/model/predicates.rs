// MIT License
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
        NamedPropertyPredicate{ name: String::from(name), predicate}
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