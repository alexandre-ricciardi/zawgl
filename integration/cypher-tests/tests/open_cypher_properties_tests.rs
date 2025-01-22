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

use serde_json::json;
use zawgl_client::Client;
use cypher_tests::run_test;

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_cypher_create() {
    run_test("test_create", 7182, test_create).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_create_match() {
    run_test("test_filter_on_property_value", 7183, test_filter_on_property_value).await;
}


async fn test_create(mut client: Client) {
    let r = client.execute_cypher_request("create (charlie:Person { name:'Charlie Sheen' }) return charlie").await;
    if let Ok(d) = r {
        println!("{}", d.to_string());
    }
    let r = client.execute_cypher_request("match (n:Person) return n").await;
    if let Ok(d) = r {
        println!("{}", d.to_string());
        let graphs = d["result"]["graphs"].as_array().expect("result");
        assert_eq!(graphs.len(), 1);
        for g in graphs {
            let nodes = g["nodes"].as_array().expect("nodes");
            assert_eq!(nodes.len(), 1);
            for node in nodes {
                let props = node["properties"].as_object().expect("properties");
                assert_eq!(props.len(), 1);
                assert_eq!(props["name"].as_str().unwrap(), "Charlie Sheen");
            }
        }
    } else {
        assert!(false, "no response")
    }
}


async fn test_filter_on_property_value(mut client: Client) {
    for age in 20..50 {
        let params = json!({
            "age": age
        });
        let r = client.execute_cypher_request_with_parameters("create (charlie:Person { age: $age }) return charlie", params).await;
        if let Ok(d) = r {
            println!("{}", d.to_string());
        }
    }

    let r = client.execute_cypher_request("match (n:Person) where n.age > 40 return n").await;
    if let Ok(d) = r {
        println!("{}", d.to_string());
        let graphs = d["result"]["graphs"].as_array().expect("graphs");
        assert_eq!(graphs.len(), 9);
        for g in graphs {
            let nodes = g["nodes"].as_array().expect("nodes");
            assert_eq!(nodes.len(), 1);
            for node in nodes {
                let props = node["properties"].as_object().expect("properties");
                assert_eq!(props.len(), 1);
            }
        }
    } else {
        assert!(false, "no response")
    }
}
