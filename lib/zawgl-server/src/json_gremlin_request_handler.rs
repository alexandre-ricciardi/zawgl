// MIT License
//
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

use zawgl_gremlin::gremlin::ToJson;
use zawgl_gremlin::handler::GremlinError;
use zawgl_gremlin::handler::handle_gremlin_request;
use zawgl_gremlin::json_gremlin_request_builder::*;
use zawgl_tx_handler::request_handler::RequestHandler;
use zawgl_tx_handler::tx_handler::TxHandler;
use serde_json::Value;

pub fn handle_gremlin_json_request<'a>(tx_handler: TxHandler, graph_request_handler: RequestHandler<'a>, value: &Value) -> Result<Value, GremlinError> {
    let gremlin_request = build_gremlin_request_from_json(value)?;
    let res = handle_gremlin_request(tx_handler, graph_request_handler, &gremlin_request)?;
    Ok(res.to_json())
}