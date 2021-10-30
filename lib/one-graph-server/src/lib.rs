extern crate one_graph_gremlin;

extern crate log;

extern crate tokio_tungstenite;
extern crate tokio;
extern crate tungstenite;
extern crate futures_util;

extern crate serde_json;

use futures_util::{
    SinkExt, StreamExt,
};
use tungstenite::Message;
use std::sync::RwLock;
use std::sync::Arc;
use log::*;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use serde_json::Value;
use std::result::Result;
use self::json_gremlin_request_handler::*;
mod result;
mod json_gremlin_request_handler;
use one_graph_gremlin::gremlin_engine::steps::gremlin_state::GremlinStateError;
use self::result::ServerError;
use one_graph_core::model::init::InitContext;
use one_graph_gremlin::gremlin_engine::GremlinDatabaseEngine;

async fn accept_connection<'a>(peer: SocketAddr, graph_engine: Arc<RwLock<GremlinDatabaseEngine<'a>>>, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, graph_engine, stream).await {
        match e {
            ServerError::WebsocketError(te) => match te {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => error!("Error processing connection: {}", err),
            },
            ServerError::ParsingError(err_msg) => error!("Parsing error: {}", err_msg),
            ServerError::HeaderError => error!("wrong header"),
            ServerError::GremlinError => error!("parsing gremlin request"),
            ServerError::DatabaseError(db_err) => match db_err {
                one_graph_gremlin::gremlin_engine::DatabaseError::GremlinError(g_err) => match g_err {
                    GremlinStateError::Invalid(step) => error!("invalid gremlin state {:?}", step),
                    GremlinStateError::WrongContext(err) => error!("wrong gremlin context {}", err),
                },
                one_graph_gremlin::gremlin_engine::DatabaseError::EngineError => error!("database engine error"),
                one_graph_gremlin::gremlin_engine::DatabaseError::ResponseError => error!("build gremlin response error"),
                one_graph_gremlin::gremlin_engine::DatabaseError::RequestError => error!("gremlin request error"),
            },
        }
    }
}


async fn handle_connection<'a>(peer: SocketAddr, graph_engine: Arc<RwLock<GremlinDatabaseEngine<'a>>>, stream: TcpStream) -> Result<(), ServerError> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let mut msg_fut = ws_receiver.next();
    loop {
        match msg_fut.await {
            Some(msg) => {
                let msg = msg.map_err(ServerError::WebsocketError)?;
                if msg.is_binary() {
                    let text_msg = msg.to_text().map_err(ServerError::WebsocketError)?;
                    let json_msg = text_msg.strip_prefix("!application/vnd.gremlin-v3.0+json").ok_or(ServerError::HeaderError)?;
                    let v: Value = serde_json::from_str(json_msg).map_err(|err| ServerError::ParsingError(err.to_string()))?;
                    let gremlin_reply = handle_gremlin_json_request(graph_engine.clone(), &v).map_err(|err| ServerError::DatabaseError(err))?;
                    let res_msg = serde_json::to_string(&gremlin_reply).map_err(|err| ServerError::ParsingError(err.to_string()))?;
                    let mut with_prefix = String::from("application/vnd.gremlin-v3.0+json");
                    with_prefix.push_str(&res_msg);
                    debug!("response msg: {}", res_msg);
                    let response = Message::Text(res_msg);
                    ws_sender.send(response).await.map_err(ServerError::WebsocketError)?;
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



pub async fn run_server(addr: &str, conf: InitContext<'static>) {
    let graph_engine = Arc::new(RwLock::new(GremlinDatabaseEngine::new(conf)));
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);
        tokio::spawn(accept_connection(peer, graph_engine.clone(), stream));
    }
}

