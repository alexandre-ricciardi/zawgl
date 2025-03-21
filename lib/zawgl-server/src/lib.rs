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

extern crate log;

extern crate tokio_tungstenite;
extern crate tokio;
extern crate futures_util;

extern crate serde_json;

use futures_util::{
    SinkExt, StreamExt,
};
use serde_json::json;
use serde_json::Value;
use settings::Settings;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::mpsc::Receiver;
use tokio_tungstenite::tungstenite::Utf8Bytes;
use zawgl_core::model::init::DatabaseInitContext;
use zawgl_front::cypher::query_engine::CypherError;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::sync::Mutex;
use zawgl_front::tx_handler::request_handler::GraphRequestHandler;
use zawgl_front::tx_handler::handler::GraphTxHandler;
use parking_lot::ReentrantMutex;
use tokio_tungstenite::tungstenite::Message;
use std::cell::RefCell;
use std::sync::Arc;
use log::*;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use std::result::Result;
use crate::open_cypher_request_handler::handle_open_cypher_request;
use tokio::sync::oneshot::{self, Sender};
use tokio::task::JoinSet;
mod result;
pub mod settings;
mod open_cypher_request_handler;
use self::result::ServerError;
use zawgl_core::model::init::InitContext;

type ResponseMessage = (Value, Sender<Result<Value, CypherError>>);

async fn accept_connection(stream: TcpStream, msg_tx: UnboundedSender<ResponseMessage>) {
    if let Err(e) = handle_connection(stream, msg_tx).await {
        match e {
            ServerError::Websocket(te) => match te {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => error!("error processing connection: {}", err),
            },
            ServerError::Parsing(err_msg) => error!("parsing error: {}", err_msg),
            ServerError::Header => error!("wrong header"),
            ServerError::CypherTx(err) => error!("tx error {}", err),
            ServerError::Concurrency => error!("concurrency error"),
        }
    }
}


async fn handle_connection(stream: TcpStream, msg_tx: UnboundedSender<ResponseMessage>) -> Result<(), ServerError> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let mut msg_fut = ws_receiver.next();
        while let Some(msg) = msg_fut.await {
            let msg = msg.map_err(ServerError::Websocket)?;
            if msg.is_text() {
                let open_cypher_prefix = "!application/openCypher";
                if let Ok(data) = msg.into_text() {
                    if data.len() > open_cypher_prefix.len() &&  &data[..open_cypher_prefix.len()] == open_cypher_prefix {
                        let doc = Value::from_str(&data[open_cypher_prefix.len()..]).map_err(|err| ServerError::Parsing(err.to_string()))?;
                        debug!("incoming message {}", doc.to_string());
                        let (tx, rx) = oneshot::channel();
                        msg_tx.send((doc, tx)).map_err(|_| ServerError::Concurrency)?;
                        let cypher_reply = rx.await.map_err(|_| ServerError::Concurrency)?;
                        let response = cypher_reply.map_err(ServerError::CypherTx)?.to_string();
                        debug!("response message {}", response);
                        let response_msg = Message::Text(Utf8Bytes::from(response));
                        ws_sender.send(response_msg).await.map_err(ServerError::Websocket)?;
                    } else {
                        return Err(ServerError::Header);
                    }
                }
                
            }
            else if msg.is_close() {
                break;
            }
            msg_fut = ws_receiver.next(); // Receive next WebSocket message.
        }

    Ok(())
}

type GraphHandler = Arc<Mutex<GraphRequestHandler>>;

fn make_handlers(conf: &InitContext) -> Vec<GraphHandler> {
    conf.dbs_ctx.iter().map(|ctx| Arc::new(Mutex::new(GraphRequestHandler::new(ctx.clone())))).collect::<Vec<_>>()
}

fn make_handlers_map(db_names: Vec<String>, handlers: &Arc<Mutex<Vec<GraphHandler>>>) -> HashMap<String, GraphHandler> {
    handlers.lock().unwrap().iter().zip(&db_names).map(|(gh, db_name)| (db_name.to_string(), Arc::clone(&gh))).collect::<HashMap<_, _>>()
}

pub async fn run_server<F>(addr: &str, conf: InitContext, callback: F, mut rx_run: Receiver<bool>) -> JoinSet<()> where F : FnOnce() -> () {
        
        
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Websocket listening on: {}", addr);
    let (msg_tx, mut msg_rx) = tokio::sync::mpsc::unbounded_channel::<ResponseMessage>();
    
    let graph_request_handlers = make_handlers(&conf);
    let db_names = conf.dbs_ctx.iter().map(|ctx| ctx.db_name.to_string()).collect();
    let mut set = JoinSet::new();
    let gh_handlers = Arc::new(Mutex::new(graph_request_handlers));
    let gh_handlers_for_commit = Arc::clone(&gh_handlers);
    set.spawn(async move {
        let mut gh_tx_map = make_handlers_map(db_names, &gh_handlers);
        let tx_handler = Arc::new(ReentrantMutex::new(RefCell::new(GraphTxHandler::new())));
        while let Some((doc, sender)) = msg_rx.recv().await {
            let db_name = doc.get("database");
            if let Some(db) = db_name {
                if let Some(db_str_name) = db.as_str() {
                    let gh = gh_tx_map.get(db_str_name);
                    if let Some(graph_handler) = gh {
                        let cypher_reply = handle_open_cypher_request(Arc::clone(&tx_handler), Arc::clone(graph_handler), &doc);
                        if let Err(_err) = sender.send(cypher_reply) {
                            error!("sending reply");
                            break;
                        }         
                    }
                }
            } else {
                let create = doc.get("create");
                if let Some(create_db_name) = create {
                    if let Some(db_name) = create_db_name.as_str() {
                        if !gh_tx_map.contains_key(db_name) {
                            let ctx = DatabaseInitContext::new(&conf.root, db_name);
                            if let Some(db_ctx) = ctx {
                                let gh = Arc::new(Mutex::new(GraphRequestHandler::new(db_ctx)));
                                gh_handlers.lock().unwrap().push(Arc::clone(&gh));
                                gh_tx_map.insert(db_name.to_string(), gh);
                                if let Ok(_) = fs::create_dir_all(format!("{}/{}", &conf.root, db_name)) {
                                    if let Err(_err) = sender.send(Ok(json!("database created"))) {
                                        error!("sending reply");
                                        break;
                                    }
                                } else {
                                    error!("Failed to create database directory");
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    set.spawn(async move {
        while let Some(run) = rx_run.recv().await {
            let graph_request_handlers_commit_clone = gh_handlers_for_commit.lock().unwrap().iter().map(|gh| Arc::clone(gh)).collect::<Vec<GraphHandler>>();
            for gh in graph_request_handlers_commit_clone {
                gh.lock().unwrap().commit();
                if !run {
                    break;
                }
            }
        }
    });

    callback();

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);
        tokio::spawn(accept_connection(stream, msg_tx.clone()));
    }
    
    set
}

pub fn keep_commit_loop(period: u64) -> (tokio::sync::mpsc::Sender<bool>, Receiver<bool>) {
    let (tx_run, rx_run) = tokio::sync::mpsc::channel::<bool>(1);
    let tx_commit = tx_run.clone();
    tokio::spawn(async move {
        let sleep_duration = std::time::Duration::from_millis(period);
        loop {
            std::thread::sleep(sleep_duration);
            if let Err(_) = tx_commit.send(true).await {
                break;
            }
        }
    });
    (tx_run, rx_run)
}