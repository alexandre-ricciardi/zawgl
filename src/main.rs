mod cypher;
mod graph;
mod model;
mod store;
mod query_engine;

fn main() {
    let lexer = cypher::lexer::Lexer::new("true or false");
    //println!("Hello, world! {}", lexer.next_token().to_string());
}
