mod parameters;

use std::borrow::{BorrowMut, Borrow};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::{io::Cursor, collections::HashMap};

use futures_channel::mpsc::{UnboundedSender};
use futures_channel::oneshot::{Sender, Receiver, Canceled};
use futures_util::{future, pin_mut, StreamExt, SinkExt, TryFutureExt};
use parameters::{Parameters, PropertyValue};
use parking_lot::ReentrantMutex;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream, MaybeTlsStream};
use bson::{Bson, Document, doc};
use uuid::Uuid;
use log::*;


type SharedChannelsMap = Arc<Mutex<HashMap<String, Sender<Document>>>>;

pub struct Client {
    request_tx: UnboundedSender<Message>,
    map_rx_channels: SharedChannelsMap,
}

impl Client {

    pub async fn new(address: &str) -> Self {
        let url = url::Url::parse(address).unwrap();
        let (ws_stream, _) = connect_async(&url).await.expect("Failed to connect");
        let (write, read) = ws_stream.split();
        let (request_tx, request_rx) = futures_channel::mpsc::unbounded();
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
                                tx.send(doc);
                            }
                        }
                    },
                    Err(_) => {
                        debug!("ws closed")
                    },
                }
            }).await
        });
        Client{request_tx: request_tx, map_rx_channels: map.clone()}
    }

    pub async fn execute_cypher_request_with_parameters(&mut self, query: &str, params: Parameters) -> Result<Document, Canceled> {
        let uuid =  Uuid::new_v4();
        let (tx, rx) = futures_channel::oneshot::channel::<Document>();
        self.map_rx_channels.lock().unwrap().insert(uuid.to_string(), tx);
        tokio::spawn(send_request(self.request_tx.clone(), uuid.to_string(), query.to_string(), params));
        rx.await
    }

    
    pub async fn execute_cypher_request(&mut self, query: &str) -> Result<Document, Canceled> {
        self.execute_cypher_request_with_parameters(query, Parameters::new()).await
    }
}

fn extract_value(name: String, value: PropertyValue) -> Document {
    match value {
        PropertyValue::String(sv) => doc!{
            name: sv
        },
        PropertyValue::Integer(iv) => doc!{
            name: iv
        },
        PropertyValue::Float(fv) => doc!{
            name: fv
        },
        PropertyValue::Bool(bv) => doc!{
            name: bv
        },
        PropertyValue::Parameters(params) => doc!{
            name: build_parameters(params)
        },
    }
}

fn build_parameters(params: Parameters) -> Vec<Document> {
    let mut res = Vec::new();
    for (name, value) in params {
        res.push(extract_value(name, value));
    }
    res
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
