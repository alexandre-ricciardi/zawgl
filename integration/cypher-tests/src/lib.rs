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

use zawgl_core::{model::init::InitContext, test_utils::build_dir_path_and_rm_old};
use zawgl_client::Client;
use std::future::Future;

pub async fn run_test<F, T>(db_name: &str, port: i32, lambda: F) where F : FnOnce(Client) -> T, T : Future<Output = ()> + Send {

    println!("BEGIN RUN {}", db_name);
    let db_dir = build_dir_path_and_rm_old(db_name).expect("error");
    
    let ctx = InitContext::new(&db_dir).expect("can't create database context");
    let (tx_start, rx_start) = tokio::sync::oneshot::channel::<()>();
    let address = format!("localhost:{}", port);
    
    let (tx_run, rx_run, commit_loop) = zawgl_server::keep_commit_loop(500);

    let server = zawgl_server::run_server(&address, ctx, || {
        if let Err(_) = tx_start.send(()) {
            println!("error starting database");
        }
    }, rx_run);

    let error_cb = || async {
        assert!(false, "error server");
    };
    let server_address = format!("ws://localhost:{}", port);
    
    let trigger = || async {
        match rx_start.await {
            Ok(_) => async move {
                let client = Client::new(&server_address).await;
                lambda(client).await;
                if let Err(_) = tx_run.send(false).await {
                    println!("Error stoping database")
                };
            }.await,
            Err(_) => error_cb().await,
        }
    };
    tokio::select! {
        _ = server => 0,
        _ = trigger()  => 0,
        _ = commit_loop => 0,
    };
   
}
