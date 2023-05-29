pub mod parameters;

use std::sync::{Arc, Mutex};
use std::{io::Cursor, collections::HashMap};

use futures_channel::mpsc::{UnboundedSender};
use futures_channel::oneshot::{Sender, Canceled};
use futures_util::{StreamExt};
use parameters::{Parameters, PropertyValue};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use bson::{Document, doc, Bson};
use uuid::Uuid;
use log::*;


type SharedChannelsMap = Arc<Mutex<HashMap<String, Sender<Document>>>>;

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
                        let doc = Document::from_reader(Cursor::new(msg.into_data())).expect("response");
                        let id = doc.get_str("request_id");
                        if let Ok(request_id) = id {
                            if let Some(tx) = clone.lock().unwrap().remove(request_id) {
                                let res = tx.send(doc);
                                if let Err(d) = res {
                                    error!("parsing document {}", d)
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

    pub async fn execute_cypher_request_with_parameters(&mut self, query: &str, params: Parameters) -> Result<Document, Canceled> {
        let uuid =  Uuid::new_v4();
        let (tx, rx) = futures_channel::oneshot::channel::<Document>();
        self.map_rx_channels.lock().unwrap().insert(uuid.to_string(), tx);
        tokio::spawn(send_request(self.request_tx.clone(), uuid.to_string(), query.to_string(), params));
        tokio::select! {
            message = self.error_rx.recv() => {
                match message {
                    Some(msg) => debug!("client error {}", msg),
                    None => panic!("should not happen"),
                }
                Err(Canceled)
            },
            document = rx => document,
        }
    }

    
    pub async fn execute_cypher_request(&mut self, query: &str) -> Result<Document, Canceled> {
        self.execute_cypher_request_with_parameters(query, Parameters::new()).await
    }
}

fn extract_value(value: PropertyValue) -> Bson {
    match value {
        PropertyValue::String(sv) => Bson::String(sv),
        PropertyValue::Integer(iv) => Bson::Int64(iv),
        PropertyValue::Float(fv) => Bson::Double(fv),
        PropertyValue::Bool(bv) => Bson::Boolean(bv),
        PropertyValue::Parameters(params) => Bson::Document(build_parameters(params)),
    }
}

fn build_parameters(params: Parameters) -> Document {
    let mut doc = Document::new();
    for (name, value) in params {
        doc.insert(name, extract_value(value));
    }
    doc
}

async fn send_request(tx: futures_channel::mpsc::UnboundedSender<Message>, id: String, query: String, params: Parameters) -> Option<()> {
    let mut msg = "!application/openCypher".as_bytes().to_vec();
    let doc = doc!{
        "request_id": String::from(id),
        "query" : query,
        "parameters": build_parameters(params),
    };
    doc.to_writer(&mut msg).ok()?;
    tx.unbounded_send(Message::binary(msg)).unwrap();
    Some(())
}
