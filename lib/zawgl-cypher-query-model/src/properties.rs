// MIT License
//
// Copyright (c) 2024 Alexandre RICCIARDI
//
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

use serde_json::{Number, Value};
use zawgl_core::model::PropertyValue;

use crate::ast::AstVisitorError;

pub fn convert_json_value(value: Value) -> Result<PropertyValue, AstVisitorError> {
    if value.is_string() {
        Ok(PropertyValue::PString(value.as_str().ok_or(AstVisitorError::SyntaxError)?.to_string()))
    } else {
        match value {
            Value::Null => todo!(),
            Value::Bool(b) => Ok(PropertyValue::PBool(b)),
            Value::Number(number) => {
                if number.is_u64() {
                    Ok(PropertyValue::PUInteger(number.as_u64().ok_or(AstVisitorError::SyntaxError)?))
                } else if number.is_f64() {
                    Ok(PropertyValue::PFloat(number.as_f64().ok_or(AstVisitorError::SyntaxError)?))
                } else if number.is_i64() {
                    Ok(PropertyValue::PInteger(number.as_i64().ok_or(AstVisitorError::SyntaxError)?))
                } else {
                    Err(AstVisitorError::SyntaxError)
                }
            },
            Value::String(s) => Ok(PropertyValue::PString(s)),
            Value::Array(_) => Err(AstVisitorError::SyntaxError),
            Value::Object(_) => Err(AstVisitorError::SyntaxError),
        }
    }

}

pub fn convert_property_value(value: PropertyValue) -> Result<Value, AstVisitorError> {
    match value {
        PropertyValue::PString(s) => Ok(Value::String(s)),
        PropertyValue::PInteger(i) => Ok(Value::Number(Number::from(i))),
        PropertyValue::PUInteger(u) => Ok(Value::Number(Number::from(u))),
        PropertyValue::PFloat(f) => Ok(Value::Number(Number::from_f64(f).ok_or(AstVisitorError::SyntaxError)?)),
        PropertyValue::PBool(b) => Ok(Value::Bool(b)),
    }
}