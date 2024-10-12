pub mod parameters;

use std::sync::{Arc, Mutex};
use std::{io::Cursor, collections::HashMap};

use futures_channel::mpsc::UnboundedSender;
use futures_channel::oneshot::{Sender, Canceled};
use futures_util::StreamExt;
use parameters::{Parameters, Value};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use json::object;
use json::object::Object;
use json::parse;
use json::JsonValue;
use uuid::Uuid;
use log::*;


type SharedChannelsMap = Arc<Mutex<HashMap<String, Sender<Object>>>>;

/// Zawgl graph database client
pub struct Client {
    request_tx: UnboundedSender<Message>,
    map_rx_channels: SharedChannelsMap,
    error_rx: tokio::sync::mpsc::UnboundedReceiver<String>,
}

impl Client {

    pub async fn new(address: &str) -> Self {
        let url = url::Url::parse(address).unwrap();
        let (ws_stream, _) = connect_async(&url).await.expect("Failed to connect");
        let (write, read) = ws_stream.split();
        let (request_tx, request_rx) = futures_channel::mpsc::unbounded();
        let (error_tx, error_rx) = tokio::sync::mpsc::unbounded_channel();
        tokio::spawn(request_rx.map(Ok).forward(write));
        let map: SharedChannelsMap = Arc::new(Mutex::new(HashMap::new()));
        
        let clone = Arc::clone(&map);
        tokio::spawn(async move {
            read.for_each(|message| async {
                match message {
                    Ok(msg) => {
                        let doc = parse(&msg.into_text().expect("json message")).expect("response");
                        if let JsonValue::Object(json_query) = doc {
                            let request_id = json_query.get("request_id").expect("request id").as_str().expect("str reuqest id");
                            if let Some(tx) = clone.lock().unwrap().remove(request_id) {
                                let res = tx.send(json_query);
                                if let Err(d) = res {
                                    error!("parsing document {}", d.dump())
                                }
                            }
                        }
                    },
                    Err(_) => {
                        let res = error_tx.send("ws closed".to_string());
                        if let Err(er) = res {
                            debug!("error occured {}", er)
                        }
                    },
                }
            }).await
        });
        Client{request_tx, map_rx_channels: map.clone(), error_rx}
    }
    
    /// Executes a cypher request with parameters
    pub async fn execute_cypher_request_with_parameters(&mut self, query: &str, params: Parameters) -> Result<Object, Canceled> {
        let uuid =  Uuid::new_v4();
        let (tx, rx) = futures_channel::oneshot::channel::<Object>();
        self.map_rx_channels.lock().unwrap().insert(uuid.to_string(), tx);
        tokio::spawn(send_request(self.request_tx.clone(), uuid.to_string(), query.to_string(), params));
        tokio::select! {
            message = self.error_rx.recv() => {
                match message {
                    Some(msg) => error!("client error {}", msg),
                    None => panic!("should not happen"),
                }
                Err(Canceled)
            },
            document = rx => document,
        }
    }

    /// Executes a cypher request
    pub async fn execute_cypher_request(&mut self, query: &str) -> Result<Object, Canceled> {
        self.execute_cypher_request_with_parameters(query, Parameters::new()).await
    }
}

fn extract_value(value: Value) -> JsonValue {
    match value {
        Value::String(sv) => JsonValue::from(sv),
        Value::Integer(iv) => object!{ "integer": JsonValue::from(iv)},
        Value::Float(fv) => object!{ "float": JsonValue::from(fv)},
        Value::Bool(bv) => JsonValue::from(bv),
        Value::Parameters(params) => JsonValue::from(build_parameters(params)),
    }
}

fn build_parameters(params: Parameters) -> Object {
    let mut doc = Object::new();
    for (name, value) in params {
        doc.insert(&name, extract_value(value));
    }
    doc
}

async fn send_request(tx: futures_channel::mpsc::UnboundedSender<Message>, id: String, query: String, params: Parameters) -> Option<()> {
    let mut msg = "!application/openCypher".as_bytes().to_vec();
    let doc = object!{
        "request_id": id,
        "query" : query,
        "parameters": build_parameters(params),
    };
    doc.write(&mut msg).ok()?;
    tx.unbounded_send(Message::binary(msg)).unwrap();
    Some(())
}
