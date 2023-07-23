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

use zawgl_client::{Client, parameters::{Parameters, Value}};
use cypher_tests::{run_test, extract_node_id};

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_aggregation() {
    run_test("test_aggregation", 11182, _test_aggregation).await;
}

async fn _test_aggregation(mut client: Client) {
    let result1 = client.execute_cypher_request("create (test:Person) return test").await;
    let result2 = client.execute_cypher_request("create (test:Person) return test").await;
    if let (Ok(d1), Ok(d2))  = (result1, result2) {
        let id1 = extract_node_id(d1).expect("node id");
        let id2 = extract_node_id(d2).expect("node id");
        for i in 0..100 {

            let mut p = Parameters::new();
            p.insert("pid1".to_string(), Value::Integer(id1));
            p.insert("pid2".to_string(), Value::Integer(id2));
            p.insert("weight".to_string(), Value::Integer(i));
            let result = client.execute_cypher_request_with_parameters("match (test:Person) where id(test) = $pid1 match (t:Person) where id(t) = $pid2 create (test:Person)-[:IsFriendOf]->(new:Person {weight: $weight})-[:IsFriendOf]->(t:Person) return new, test, t", p).await;
            let res = result.expect("new person");
            println!("{}", res.to_string());
        }
    }
    let result = client.execute_cypher_request("match (test:Person)-[:IsFriendOf]->(new:Person)-[:IsFriendOf]->(t:Person) return sum(new.weight) as sum").await;
    if let Ok(d) = result {
        println!("{}", d.to_string());
    } else {
        assert!(false, "no response")
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_aggregation_issue() {
    run_test("test_aggregation_issue", 11182, _test_aggregation_issue).await;
}

async fn _test_aggregation_issue(mut client: Client) {
    let result1 = client.execute_cypher_request("create (test:Person) return test").await;
    let result2 = client.execute_cypher_request("create (test:Person) return test").await;
    if let (Ok(d1), Ok(d2))  = (result1, result2) {
        let id1 = extract_node_id(d1).expect("node id");
        let id2 = extract_node_id(d2).expect("node id");
        for i in 0..100 {

            let mut p = Parameters::new();
            p.insert("pid1".to_string(), Value::Integer(id1));
            p.insert("pid2".to_string(), Value::Integer(id2));
            let result = client.execute_cypher_request_with_parameters("match (s:Person) where id(s) = $pid1 match (t:Person) where id(t) = $pid2 return s, t", p).await;
            let res = result.expect("new person");
            println!("{}", res.to_string());
        }
    }
}
