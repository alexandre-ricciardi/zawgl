use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use futures_channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures_channel::oneshot::{Sender, Canceled};
use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use serde_json::{from_str, json, value::Value};
use uuid::Uuid;
use log::*;


type SharedChannelsMap = Arc<Mutex<HashMap<String, Sender<Value>>>>;

/// Zawgl graph database client
#[derive(Debug, Clone)]
pub struct Client {
    request_tx: UnboundedSender<Message>,
    map_rx_channels: SharedChannelsMap,
}

impl Client {

    pub async fn new(address: &str) -> Self {
        let (ws_stream, _) = connect_async(address).await.expect("Failed to connect");
        let (write, read) = ws_stream.split();
        let (request_tx, request_rx) = futures_channel::mpsc::unbounded();
        
        tokio::spawn(request_rx.map(Ok).forward(write));
        let map: SharedChannelsMap = Arc::new(Mutex::new(HashMap::new()));
        
        let clone = Arc::clone(&map);
        tokio::spawn(async move {
            read.for_each(|message| async {
                match message {
                    Ok(msg) => {
                        let doc: Value = from_str(&msg.into_text().expect("json message")).expect("response");
                        let request_id = doc["request_id"].as_str().unwrap();
                        if let Some(tx) = clone.lock().unwrap().remove(request_id) {
                            let res = tx.send(doc);
                            if let Err(d) = res {
                                error!("parsing document {}", d.to_string())
                            }
                        }
                    },
                    Err(er) => {
                        debug!("error occured {}", er);
                    },
                }
            }).await
        });
        Client{request_tx, map_rx_channels: Arc::clone(&map)}
    }
    
    /// Executes a cypher request with parameters
    pub async fn execute_cypher_request_with_parameters(&mut self, query: &str, params: Value) -> Result<Value, Canceled> {
        let uuid =  Uuid::new_v4();
        let (tx, rx) = futures_channel::oneshot::channel::<Value>();
        self.map_rx_channels.lock().unwrap().insert(uuid.to_string(), tx);
        tokio::spawn(send_request(self.request_tx.clone(), uuid.to_string(), query.to_string(), params));
        tokio::select! {
            document = rx => document,
        }
    }

    /// Executes a cypher request
    pub async fn execute_cypher_request(&mut self, query: &str) -> Result<Value, Canceled> {
        self.execute_cypher_request_with_parameters(query, json!({})).await
    }
}

async fn send_request(tx: futures_channel::mpsc::UnboundedSender<Message>, id: String, query: String, params: Value) -> Option<()> {
    let mut msg = "!application/openCypher".to_string();
    let doc = json!({
        "request_id": id,
        "query" : query,
        "parameters": params,
    });
    msg.push_str(&doc.to_string());
    tx.unbounded_send(Message::text(msg.to_string())).unwrap();
    Some(())
}
