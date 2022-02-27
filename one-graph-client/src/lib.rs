use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use bson::{Bson, Document, doc};
use uuid::Uuid;

pub struct Client {
    address: String,
}

async fn send_request(tx: futures_channel::mpsc::UnboundedSender<Message>, query: &str) -> Option<()>{
    let mut msg = "!application/openCypher".as_bytes().to_vec();
    let doc = doc!{
        "request_id": Uuid::new_v4().to_urn().to_string(),
        "query" : query,
    };
    doc.to_writer(&mut msg).ok()?;
    tx.unbounded_send(Message::binary(msg)).unwrap();
    Some(())
}


impl Client {
    pub fn new(address: &str) -> Self {
        Client{address: String::from(address)}
    }

    pub async fn execute_cypher_request(&self, query: &'static str) {
        let url = url::Url::parse(&self.address).unwrap();
        let (request_tx, response_rx) = futures_channel::mpsc::unbounded();
        tokio::spawn(send_request(request_tx, query));
    
        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    
        let (write, read) = ws_stream.split();
    
        let stdin_to_ws = response_rx.map(Ok).forward(write);
        let ws_to_stdout = {
            read.for_each(|message| async {
                let data = message.unwrap().into_data();
                tokio::io::stdout().write_all(&data).await.unwrap();
            })
        };
    
        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;
    }
}