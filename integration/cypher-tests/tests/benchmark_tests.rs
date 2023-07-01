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

use std::{time::Instant};

use zawgl_client::Client;
use cypher_tests::run_test;

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_benchmark_create() {
    run_test("benchmark_test", 6182, benchmark_test).await;
}

async fn benchmark_test(mut client: Client) {
    let result = client.execute_cypher_request("create (test:Person) return test").await;
    if let Err(r) = result {
        println!("{}", r.to_string());
    }
    let start = Instant::now();
    for i in 0..101000 {
        let result = client.execute_cypher_request("create (test:Person) return test").await;
        if let Err(r) = result {
            println!("{}", r.to_string());
            assert!(false, "error {}", i)
        }
        if i % 1000 == 0 {
            println!("created {} nodes", i);
        }
    }
    
    let duration = start.elapsed();
    println!("Time to create 1000 nodes: {:?}", duration)
}

//#[tokio::test]
async fn test_benchmark_createss() {
    let client = Client::new("ws://localhost:8182").await;
    benchmark_test(client).await;
}