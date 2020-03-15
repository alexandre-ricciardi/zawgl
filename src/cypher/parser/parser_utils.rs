use super::*;
use super::super::lexer::*;

pub fn print_node(node: &Box<dyn Ast>, tokens: &Vec<Token>, depth: i32) {
    let mut ws = 0;
    while ws < depth {
        print!(" ");
        ws += 1;
    }
    match &node.token_type {
        Some(tok_type) => {
            let content = node.as_ref().token_value.as_ref();
            println!("|_>{:?}:{}", tok_type, content.unwrap());
        },
        None => {
            match &node.ast_tag {
                Some(tag) => println!("|_>{:?}", tag),
                None => {
                    println!("|_>{}", tokens[node.token_id.unwrap()].content)
                }    
            }
        }
    }
    
    for child in &(node.childs) {
        print_node(&child, tokens, depth + 1);
    }
}