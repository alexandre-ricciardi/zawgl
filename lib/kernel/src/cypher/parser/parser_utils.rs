use super::*;
use super::super::lexer::*;

pub fn print_node(node: &Box<dyn Ast>, tokens: &Vec<Token>, depth: i32) {
    let mut ws = 0;
    while ws < depth {
        print!(" ");
        ws += 1;
    }
    println!("|_>{}", node);
    
    for child in node.get_childs() {
        print_node(&child, tokens, depth + 1);
    }
}