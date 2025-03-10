use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use futures_channel::mpsc::UnboundedSender;
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
    request_tx: Arc<Mutex<UnboundedSender<Message>>>,
    map_rx_channels: SharedChannelsMap,
    staled: Arc<Mutex<bool>>,
}

impl Client {

    pub async fn new(address: &str) -> Self {
        let (request_tx, request_rx) = futures_channel::mpsc::unbounded();
        let map: SharedChannelsMap = Arc::new(Mutex::new(HashMap::new()));
        let staled = Arc::new(Mutex::new(false));
        if let Ok((ws_stream, _)) = connect_async(address).await {
            let (write, read) = ws_stream.split();
            tokio::spawn(request_rx.map(Ok).forward(write));
            let clone = Arc::clone(&map);
            let staled_cloned = Arc::clone(&staled);
            tokio::spawn(async move {
                read.for_each(|message| receive(Arc::clone(&staled_cloned), message, Arc::clone(&clone))).await
            });
        } else {
            *staled.lock().unwrap() = true;
        }
        Client{request_tx: Arc::new(Mutex::new(request_tx)), map_rx_channels: Arc::clone(&map), staled}
    }

    pub fn is_staled(&self) -> bool {
        *self.staled.lock().unwrap()
    }

    /// Executes a cypher request with parameters
    pub async fn execute_cypher_request_with_parameters(&mut self, db: &str, query: &str, params: Value) -> Result<Value, Canceled> {
        if !*self.staled.lock().unwrap() {
            let uuid =  Uuid::new_v4();
            let (tx, rx) = futures_channel::oneshot::channel::<Value>();
            self.map_rx_channels.lock().unwrap().insert(uuid.to_string(), tx);
            let res = tokio::spawn(send_request(Arc::clone(&self.request_tx), db.to_string(), uuid.to_string(), query.to_string(), params));
            if res.await.unwrap().is_none() {
                *self.staled.lock().unwrap() = true;
            }
            rx.await
        } else {
            Err(Canceled)
        }
    }

    /// Executes a cypher request
    pub async fn execute_cypher_request(&mut self, db: &str, query: &str) -> Result<Value, Canceled> {
        self.execute_cypher_request_with_parameters(db, query, json!({})).await
    }


    /// Create database request
    pub async fn create_database(&mut self, db_name: &str) {
        let create = json!({
            "create": db_name
        });
        send(self.request_tx.clone(), create).await;
    }
}

async fn send_request(tx: Arc<Mutex<UnboundedSender<Message>>>, db: String, id: String, query: String, params: Value) -> Option<()> {
    let doc = json!({
        "database": db,
        "request_id": id,
        "query" : query,
        "parameters": params,
    });
    send(tx, doc).await
}

async fn send(tx: Arc<Mutex<UnboundedSender<Message>>>, msg: Value) -> Option<()> {
    let mut header = "!application/openCypher".to_string();
    header.push_str(&msg.to_string());
    tx.lock().unwrap().unbounded_send(Message::text(header.to_string())).ok()
}

async fn receive(staled: Arc<Mutex<bool>>, message: Result<Message, tokio_tungstenite::tungstenite::Error>, map: SharedChannelsMap) {
    match message {
        Ok(msg) => {
            let doc: Value = from_str(&msg.into_text().expect("json message")).expect("response");
            let request_id = doc["request_id"].as_str().unwrap();
            if let Some(tx) = map.lock().unwrap().remove(request_id) {
                let res = tx.send(doc);
                if let Err(d) = res {
                    error!("parsing document {}", d.to_string())
                }
            }
        },
        Err(er) => {
            *staled.lock().unwrap() = true;
            debug!("error occured {}", er);
        },
    }
}