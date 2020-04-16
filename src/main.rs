extern crate log;
mod cypher;
mod graph;
mod model;
mod repository;
mod query_engine;
mod conf;
mod matcher;

fn main() {
    let lexer = cypher::lexer::Lexer::new("true or false");
    //println!("Hello, world! {}", lexer.next_token().to_string());
}
