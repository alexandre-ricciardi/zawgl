extern crate one_graph_server;
extern crate tokio;

#[tokio::main]
async fn main() {
    one_graph_server::run_server("127.0.0.1:8182").await;
}