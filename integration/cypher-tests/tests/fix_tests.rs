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

use serde_json::json;
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

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_create_fix_1() {
    run_test("test_match_create_fix_1", 9192, test_match_create).await;
}

async fn test_match_create(mut client: Client) {
    let r = client.execute_cypher_request("default", "create (n:User {user_id: 'blabla'}) return n").await;
    assert_eq!(r.unwrap()["result"]["graphs"][0]["nodes"][0]["properties"]["user_id"], "blabla");
    let params = json!({
        "user_id": "blabla",
        "name": "acme"
    });
    let r_1 = client.execute_cypher_request_with_parameters("default", "match (u:User) where u.user_id = $user_id create (u)-[io:IsOwner]->(o:Owner)-[hc:HasCompany]->(n:Company {name: $name}) return n", params).await;
    assert_eq!(&r_1.clone().unwrap()["result"]["graphs"][0]["nodes"][0]["properties"]["name"], "acme");
    let id_1 = extract_node_id(r_1.unwrap());

    let r_2 = client.execute_cypher_request("default", "match (u:User)-[io:IsOwner]->(o:Owner)-[hc:HasCompany]->(n:Company {name: 'acme'}) return n").await;
    let id_2 = extract_node_id(r_2.unwrap());

    assert_eq!(id_2, id_1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_match_not_create_fix_1() {
    run_test("test_match_not_create_fix_1", 9193, _test_match_not_create_fix_1).await;
}

async fn _test_match_not_create_fix_1(mut client: Client) {
    let r = client.execute_cypher_request("default", "create (n:User {user_id: 'blabla test'}) return n").await;
    assert_eq!(r.unwrap()["result"]["graphs"][0]["nodes"][0]["properties"]["user_id"], "blabla test");
    let params = json!({
        "user_id": "blabla",
        "name": "acme"
    });
    let r_1 = client.execute_cypher_request_with_parameters("default", "match (u:User) where u.user_id = $user_id create (u)-[io:IsOwner]->(o:Owner)-[hc:HasCompany]->(n:Company {name: $name}) return n", params).await;

    let id_1 = extract_node_id(r_1.unwrap());
    assert_eq!(None, id_1);

}