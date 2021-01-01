extern crate one_graph_core;
extern crate log;

extern crate tokio_tungstenite;
extern crate tokio;
extern crate tungstenite;
extern crate futures_util;

extern crate serde_json;

use futures_util::{
    SinkExt, StreamExt,
};
use log::*;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use simple_logger::SimpleLogger;
use serde_json::Value;
use std::result::Result;

#[derive(Debug)]
pub enum ServerError {
    ParsingError(String),
    WebsocketError(tungstenite::Error),
}


async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            ServerError::WebsocketError(te) => match te {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => error!("Error processing connection: {}", err),
            },
            ServerError::ParsingError(err_msg) => error!("Parsing error: {}", err_msg),
        }
        
    }
}


async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<(), ServerError> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let mut msg_fut = ws_receiver.next();
    loop {
        match msg_fut.await {
            Some(msg) => {
                let msg = msg.map_err(ServerError::WebsocketError)?;
                if msg.is_binary() {
                    let v: Value = serde_json::from_str(&msg.to_text().map_err(ServerError::WebsocketError)?).map_err(|err| ServerError::ParsingError(err.to_string()))?;
                    
                }
                if msg.is_text() || msg.is_binary() {
                    ws_sender.send(msg).await?;
                } else if msg.is_close() {
                    break;
                }
                msg_fut = ws_receiver.next(); // Receive next WebSocket message.
            }
            None => break, // WebSocket stream terminated.
        }
    }

    Ok(())
}

pub async fn run_server(addr: &str) {
    SimpleLogger::new().init().unwrap();
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream));
    }
}

