use super::*;

pub fn print_node(node: &Box<AstNode>, tokens: &Vec<Token>, depth: i32) {
    let mut ws = 0;
    while ws < depth {
        print!("-");
        ws += 1;
    }
    println!(">{}", tokens[node.token_id].content);
    for child in &(node.childs) {
        print_node(&child, tokens, depth + 1);
    }
}