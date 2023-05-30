// MIT License
// Copyright (c) 2023 Alexandre RICCIARDI
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

use log::LevelFilter;
use simple_logger::SimpleLogger;
use log::*;
use zawgl_client::Client;
use std::sync::Once;
use cypher_tests::run_test;

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        SimpleLogger::new().with_level(LevelFilter::Debug).init().unwrap();
    });
}

#[tokio::test]
async fn test_cypher_error_0() {
    run_test("test_cypher_syntax_error", 9182, test_cypher_syntax_error).await;
}

async fn test_cypher_syntax_error(mut client: Client) {
    let r = client.execute_cypher_request("create (n:Person)) return n").await;
    if let Ok(d) = r {
        d.get_str("error").expect("error");
    } else {
        assert!(false, "no response from server")
    }
}