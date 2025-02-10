// MIT License
// Copyright (c) 2025 Alexandre RICCIARDI
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

use zawgl_client::Client;
use cypher_tests::{extract_node_id, run_test};

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_create_fix() {
    run_test("test_create_with_props", 9191, test_create_with_props).await;
}

async fn test_create_with_props(mut client: Client) {
    let r = client.execute_cypher_request("default", "create (n:Person {name: 'blabla'}) return n").await;
    let id_0 = extract_node_id(r.unwrap());

    let r_1 = client.execute_cypher_request("default", "create (n:Person {name: 'blabla'}) return n").await;
    let id_1 = extract_node_id(r_1.unwrap());

    let r_2 = client.execute_cypher_request("default", "create (n:Person {name: 'blabla'}) return n").await;
    let id_2 = extract_node_id(r_2.unwrap());

    assert_ne!(id_0, id_1);
    assert_ne!(id_2, id_1);
}