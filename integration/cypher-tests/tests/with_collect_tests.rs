// MIT License
// Copyright (c) 2024 Alexandre RICCIARDI
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

use zawgl_client::{Client, parameters::{Parameters, Value}};
use cypher_tests::{run_test, extract_node_id};

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_with_collect() {
    run_test("test_with_collect", 12182, _test_with_collect).await;
}

async fn _test_with_collect(mut client: Client) {
    let mut p = Parameters::new();
    p.insert("weight".to_string(), Value::Integer(1));
    let _ = client.execute_cypher_request_with_parameters("create (s:Person)
                                                                create (s)-[:IsFriendOf]->(new:Person {weight: $weight})
                                                                create (new)-[:IsFriendOf]->(new1:Person {weight: $weight})
                                                                create (new)-[:IsFriendOf]->(new2:Person {weight: $weight})
                                                                return s", p).await;
    let res = client.execute_cypher_request("match (s:Person)-[:IsFriendOf]->(new:Person)-[:IsFriendOf]->(end:Person)
                                                                return collect(end) 
                                                                ").await;
    println!("{}", res.expect("persons").to_string());
}