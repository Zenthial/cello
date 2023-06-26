mod lexer;

use std::{env, fs::read_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("File not provided");

    let file_string = read_to_string(file_path).expect("Unable to read file to string");

    let mut lexer = lexer::Lexer::new(&file_string);
    while let Some(tok) = lexer.next() {
        println!("{:?} ", tok);
    }
}
