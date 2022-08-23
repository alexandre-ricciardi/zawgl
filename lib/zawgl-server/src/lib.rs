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

use bson::Document;
use futures_util::{
    SinkExt, StreamExt,
};
use zawgl_tx_handler::request_handler::GraphRequestHandler;
use zawgl_tx_handler::request_handler::RequestHandler;
use zawgl_tx_handler::tx_handler::GraphTxHandler;
use zawgl_tx_handler::tx_handler::TxHandler;
use parking_lot::ReentrantMutex;
use tokio_tungstenite::tungstenite::Message;
use std::cell::RefCell;
use std::sync::RwLock;
use std::sync::Arc;
use log::*;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use serde_json::Value;
use std::result::Result;
use crate::open_cypher_request_handler::handle_open_cypher_request;

//use self::json_gremlin_request_handler::*;
mod result;
//mod json_gremlin_request_handler;
mod open_cypher_request_handler;
use self::result::ServerError;
use zawgl_core::model::init::InitContext;

async fn accept_connection<'a>(peer: SocketAddr, tx_handler: TxHandler, graph_request_handler: RequestHandler<'a>, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, tx_handler, graph_request_handler, stream).await {
        match e {
            ServerError::WebsocketError(te) => match te {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => error!("Error processing connection: {}", err),
            },
            ServerError::ParsingError(err_msg) => error!("Parsing error: {}", err_msg),
            ServerError::HeaderError => error!("wrong header"),
            ServerError::CypherTxError(_) => todo!(),
        }
    }
}


async fn handle_connection<'a, 'b>(peer: SocketAddr, tx_handler: TxHandler, graph_request_handler: RequestHandler<'a>, stream: TcpStream) -> Result<(), ServerError> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let mut msg_fut = ws_receiver.next();
    loop {
        match msg_fut.await {
            Some(msg) => {
                let msg = msg.map_err(ServerError::WebsocketError)?;
                if msg.is_binary() {
                    let json_gremlin_prefix = "!application/vnd.gremlin-v3.0+json".as_bytes();
                    let open_cypher_prefix = "!application/openCypher".as_bytes();
                    let data = msg.into_data();
                    if data.len() > json_gremlin_prefix.len() && &data[..json_gremlin_prefix.len()] == json_gremlin_prefix {
                        //let v: Value = serde_json::from_reader(&data[json_gremlin_prefix.len()..]).map_err(|err| ServerError::ParsingError(err.to_string()))?;
                        //let gremlin_reply = handle_gremlin_json_request(tx_handler.clone(), graph_request_handler.clone(), &v).map_err(|err| ServerError::GremlinTxError(err))?;
                        //let res_msg = serde_json::to_string(&gremlin_reply).map_err(|err| ServerError::ParsingError(err.to_string()))?;
                        //debug!("gremlin response msg: {}", res_msg);
                        //let response = Message::Text(res_msg);
                        //ws_sender.send(response).await.map_err(ServerError::WebsocketError)?;
                    } else if data.len() > open_cypher_prefix.len() &&  &data[..open_cypher_prefix.len()] == open_cypher_prefix {
                        let doc = Document::from_reader(&data[open_cypher_prefix.len()..]).map_err(|err| ServerError::ParsingError(err.to_string()))?;
                        let cypher_reply = handle_open_cypher_request(tx_handler.clone(), graph_request_handler.clone(), &doc).map_err(|err| ServerError::CypherTxError(err))?;
                        let mut response_data = Vec::new();
                        cypher_reply.to_writer(&mut response_data).map_err(|err| ServerError::ParsingError(err.to_string()))?;
                        let response = Message::Binary(response_data);
                        ws_sender.send(response).await.map_err(ServerError::WebsocketError)?;
                    } else {
                        break;
                    }
                }
                else if msg.is_close() {
                    break;
                }
                msg_fut = ws_receiver.next(); // Receive next WebSocket message.
            }
            None => break, // WebSocket stream terminated.
        }
    }

    Ok(())
}



pub async fn run_server<F>(addr: &str, conf: InitContext<'static>, callback: F) where F : FnOnce() -> () {
    let tx_handler = Arc::new(ReentrantMutex::new(RefCell::new(GraphTxHandler::new())));
    let graph_request_handler = Arc::new(RwLock::new(GraphRequestHandler::new(conf)));
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);
    callback();
    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);
        tokio::spawn(accept_connection(peer, tx_handler.clone(), graph_request_handler.clone(), stream));
    }
}
