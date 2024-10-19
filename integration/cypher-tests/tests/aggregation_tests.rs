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

use serde_json::json;
use zawgl_client::Client;
use cypher_tests::{run_test, extract_node_id};

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_aggregation() {
    run_test("test_aggregation", 11182, _test_aggregation).await;
}

async fn _test_aggregation(mut client: Client) {
    let result1 = client.execute_cypher_request("create (test:Person) return test").await;
    let result2 = client.execute_cypher_request("create (test:Person) return test").await;
    if let (Ok(d1), Ok(d2))  = (result1, result2) {
        println!("{}", d1.to_string());
        let id1 = extract_node_id(d1).expect("node id");
        let id2 = extract_node_id(d2).expect("node id");
        for i in 0..100 {
            let p = json!({
                "pid1": id1,
                "pid2": id2,
                "weight": i
            });
            let result = client.execute_cypher_request_with_parameters("match (s:Person) where id(s) = $pid1 match (t:Person) where id(t) = $pid2 create (s)-[:IsFriendOf]->(new:Person {weight: $weight})-[:IsFriendOf]->(t) return new, s, t", p).await;
            let res = result.expect("new person");
            println!("{}", res.to_string());
        }
    }
    let result = client.execute_cypher_request("match (test:Person)-[:IsFriendOf]->(new:Person)-[:IsFriendOf]->(t:Person) return test, sum(new.weight) as sum").await;
    if let Ok(d) = result {
        println!("{}", d.to_string());
    } else {
        assert!(false, "no response")
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_aggregation_issue() {
    run_test("test_aggregation_issue", 11183, _test_aggregation_issue).await;
}

async fn _test_aggregation_issue(mut client: Client) {
    let result1 = client.execute_cypher_request("create (test:Person) return test").await;
    let result2 = client.execute_cypher_request("create (test:Person) return test").await;
    if let (Ok(d1), Ok(d2))  = (result1, result2) {
        let id1 = extract_node_id(d1).expect("node id");
        let id2 = extract_node_id(d2).expect("node id");
        for _ in 0..100 {

            let p = json!({
                "pid1": id1,
                "pid2": id2
            });
            let result = client.execute_cypher_request_with_parameters("match (s:Person) where id(s) = $pid1 match (t:Person) where id(t) = $pid2 return s, t", p).await;
            let res = result.expect("new person");
            println!("{}", res.to_string());
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_aggregation_match_issue() {
    run_test("test_aggregation_match_issue", 11184, _test_aggregation_match_issue).await;
}

async fn _test_aggregation_match_issue(mut client: Client) {
    let result1 = client.execute_cypher_request("create (test:Person) return test").await;
    let result2 = client.execute_cypher_request("create (test:Person) return test").await;
    if let (Ok(d1), Ok(d2))  = (result1, result2) {
        let id1 = extract_node_id(d1).expect("node id");
        let id2 = extract_node_id(d2).expect("node id");
        for _ in 0..100 {
            for _ in 0..10 {
                let r = client.execute_cypher_request("create (test:Person) return test").await;
                let _res = r.expect("new person");

            }
            let p = json!({
                "pid1": id1,
                "pid2": id2
            });
            let result = client.execute_cypher_request_with_parameters("match (s:Person) where id(s) = $pid1 return s", p).await;
            let res = result.expect("new person");
            println!("{}", res.to_string());
            let id = extract_node_id(res).expect("node id");
            assert_eq!(id, id1)
        }
    }
}


#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_aggregation_1() {
    run_test("test_aggregation_1", 11185, _test_aggregation_1).await;
}

async fn _test_aggregation_1(mut client: Client) {
    for _ in 0..10 {
        let result = client.execute_cypher_request("create (test:Person) return test").await;
        if let Ok(doc) = result {
            println!("{}", doc.to_string());
            let id = extract_node_id(doc).expect("node id");
            for __ in 0..10 {
                let p = json!({
                    "pid": id,
                    "weight": 1
                });
                let r1 = client.execute_cypher_request_with_parameters("match (s:Person) where id(s) = $pid create (s)-[:IsFriendOf]->(new:Person {weight: $weight}) return new, s", p).await;
                let res = r1.expect("new person");
                println!("{}", res.to_string());
                assert_eq!(1, res["result"]["graphs"].as_array().unwrap().len());
            }
        }
    }

    let result = client.execute_cypher_request("match (test:Person)-[:IsFriendOf]->(new:Person) return test, sum(new.weight) as sum").await;
    if let Ok(d) = result {
        println!("{}", d.to_string());
        let values = d["result"]["values"].as_array().expect("values");
        assert_eq!(10, values.len());
        for value in values {
            let sum = value.as_array().expect("row")[1]["sum"].as_f64().expect("sum");
            assert_eq!(10., sum);
        }
    } else {
        assert!(false, "no response")
    }
}


#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_aggregation_2() {
    run_test("test_aggregation_2", 11186, _test_aggregation_2).await;
}

async fn _test_aggregation_2(mut client: Client) {
    for _ in 0..10 {
        let result = client.execute_cypher_request("create (test:Person) return test").await;
        if let Ok(doc) = result {
            let id = extract_node_id(doc).expect("node id");
            for __ in 0..10 {
                let p = json!({
                    "pid": id,
                    "weight": 1
                });
                let result = client.execute_cypher_request_with_parameters("match (s:Person) where id(s) = $pid 
                                                                            create (s)-[:IsFriendOf]->(new:Person {weight: $weight})
                                                                            create (new)-[:IsFriendOf]->(new1:Person {weight: $weight})
                                                                            create (new)-[:IsFriendOf]->(new2:Person {weight: $weight})
                                                                            return s", p).await;

                let res = result.expect("new person");
                println!("{}", res.to_string());
            }
        }
    }

    let result = client.execute_cypher_request("match (test:Person)-[:IsFriendOf]->(new:Person)-[:IsFriendOf]->(new1:Person) return test, new, sum(new1.weight) as sum").await;
    if let Ok(d) = result {
        let values = d["result"]["values"].as_array().expect("values");
        assert_eq!(100, values.len());
        for value in values {
            let sum = value.as_array().expect("row")[2]["sum"].as_f64().expect("the sum");
            assert_eq!(2., sum);
        }
    } else {
        assert!(false, "no response")
    }
}