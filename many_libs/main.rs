mod lexer;
mod parser;
mod syntax;

use std::{env, fs::read_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("File not provided");

    let file_string = read_to_string(file_path).expect("Unable to read file to string");

    let parser = parser::Parser::new(&file_string);
    let node = parser.parse();
    let syntax_node = syntax::SyntaxNode::new_root(node.clone());
    let formatted = format!("{:#?}", syntax_node);
    println!("{}", formatted[0..formatted.len() - 1].to_string())
}
