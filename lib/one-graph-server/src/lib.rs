extern crate one_graph_core;
extern crate log;

extern crate tokio_tungstenite;
extern crate tokio;
extern crate tungstenite;
extern crate futures_util;

use futures_util::{
    future::{select, Either},
    SinkExt, StreamExt,
};
use log::*;
use std::{net::SocketAddr, time::Duration};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use tungstenite::{Message, Result};
use simple_logger::SimpleLogger;

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // Echo incoming WebSocket messages.

    let mut msg_fut = ws_receiver.next();
    loop {
        match msg_fut.await {
            Some(msg) => {
                let msg = msg?;
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

