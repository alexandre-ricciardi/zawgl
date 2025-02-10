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
    address: String,
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
            read.for_each(|message| receive(message, Arc::clone(&map))).await
        });
        Client{request_tx, map_rx_channels: Arc::clone(&clone), address: address.to_string()}
    }



    pub async fn reconnect(&mut self) {
        let (ws_stream, _) = connect_async(&self.address).await.expect("Failed to connect");
        let (write, read) = ws_stream.split();

        let (request_tx, request_rx) = futures_channel::mpsc::unbounded();
        
        tokio::spawn(request_rx.map(Ok).forward(write));

        let clone = Arc::clone(&self.map_rx_channels);
        tokio::spawn(async move {
            read.for_each(|message| receive(message, Arc::clone(&clone))).await
        });
        self.request_tx.close_channel();
        self.request_tx = request_tx;
    }
    
    /// Executes a cypher request with parameters
    pub async fn execute_cypher_request_with_parameters(&mut self, db: &str, query: &str, params: Value) -> Result<Value, Canceled> {
        let uuid =  Uuid::new_v4();
        let (tx, rx) = futures_channel::oneshot::channel::<Value>();
        self.map_rx_channels.lock().unwrap().insert(uuid.to_string(), tx);
        tokio::spawn(send_request(self.request_tx.clone(), db.to_string(), uuid.to_string(), query.to_string(), params));
        tokio::select! {
            document = rx => document,
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

async fn send_request(tx: futures_channel::mpsc::UnboundedSender<Message>, db: String, id: String, query: String, params: Value) -> Option<()> {
    let doc = json!({
        "database": db,
        "request_id": id,
        "query" : query,
        "parameters": params,
    });
    send(tx, doc).await
}

async fn send(tx: futures_channel::mpsc::UnboundedSender<Message>, msg: Value) -> Option<()> {
    let mut header = "!application/openCypher".to_string();
    header.push_str(&msg.to_string());
    tx.unbounded_send(Message::text(header.to_string())).ok()
}

async fn receive(message: Result<Message, tokio_tungstenite::tungstenite::Error>, map: SharedChannelsMap) {
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
            debug!("error occured {}", er);
        },
    }
}