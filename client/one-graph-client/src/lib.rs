use std::borrow::{BorrowMut, Borrow};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::{io::Cursor, collections::HashMap};

use futures_channel::mpsc::{UnboundedSender};
use futures_channel::oneshot::{Sender, Receiver, Canceled};
use futures_util::{future, pin_mut, StreamExt, SinkExt, TryFutureExt};
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

    pub async fn execute_cypher_request(&mut self, query: &'static str) -> Result<Document, Canceled> {
        let uuid =  Uuid::new_v4();
        let (tx, rx) = futures_channel::oneshot::channel::<Document>();
        self.map_rx_channels.lock().unwrap().insert(uuid.to_string(), tx);
        tokio::spawn(send_request(self.request_tx.clone(), uuid.to_string(), query));
        rx.await
    }
}

async fn send_request(tx: futures_channel::mpsc::UnboundedSender<Message>, id: String, query: &str) -> Option<()> {
    let mut msg = "!application/openCypher".as_bytes().to_vec();
    let doc = doc!{
        "request_id": String::from(id),
        "query" : query,
    };
    doc.to_writer(&mut msg).ok()?;
    tx.unbounded_send(Message::binary(msg)).unwrap();
    Some(())
}
