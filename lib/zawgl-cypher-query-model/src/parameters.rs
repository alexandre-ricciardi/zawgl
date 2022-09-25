// MIT License
//
// Copyright (c) 2022 Alexandre RICCIARDI
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

use std::collections::HashMap;

use bson::{Bson, Document};
use zawgl_core::model::PropertyValue;

#[derive(Debug, Clone)]
pub enum ParameterValue {
    Parameters(Parameters),
    Value(PropertyValue),
}

pub type Parameters = HashMap<String, ParameterValue>;

pub fn build_parameters(params: &Document) -> Parameters {
    let mut parameters = Parameters::new();
    for param in params {
        match param.1 {
            Bson::Double(v) => {parameters.insert(param.0.to_string(), ParameterValue::Value(PropertyValue::PFloat(*v)));}
            Bson::String(v) => {parameters.insert(param.0.to_string(), ParameterValue::Value(PropertyValue::PString(v.to_string())));}
            Bson::Array(_) => todo!(),
            Bson::Document(v) => {parameters.insert(param.0.to_string(), ParameterValue::Parameters(build_parameters(v)));}
            Bson::Boolean(v) => {parameters.insert(param.0.to_string(), ParameterValue::Value(PropertyValue::PBool(*v)));}
            Bson::Null => todo!(),
            Bson::RegularExpression(_) => todo!(),
            Bson::JavaScriptCode(_) => todo!(),
            Bson::JavaScriptCodeWithScope(_) => todo!(),
            Bson::Int32(v) => {parameters.insert(param.0.to_string(), ParameterValue::Value(PropertyValue::PInteger(*v as i64)));}
            Bson::Int64(v) => {parameters.insert(param.0.to_string(), ParameterValue::Value(PropertyValue::PInteger(*v)));}
            Bson::Timestamp(_) => todo!(),
            Bson::Binary(_) => todo!(),
            Bson::ObjectId(_) => todo!(),
            Bson::DateTime(_) => todo!(),
            Bson::Symbol(_) => todo!(),
            Bson::Decimal128(_) => todo!(),
            Bson::Undefined => todo!(),
            Bson::MaxKey => todo!(),
            Bson::MinKey => todo!(),
            Bson::DbPointer(_) => todo!(),
        }
    }
    parameters
}