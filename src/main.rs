mod generate;
mod lexer;
mod parser;
mod translate;

use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("File not provided");

    let file_string = read_to_string(file_path)
        .expect("Unable to read file to string")
        .to_lowercase(); // we lowercase the file here because doing it inside the parser will
                         // result in annoying lifetime errors

    let mut parser = parser::Parser::new(&file_string);
    let (data, ast) = parser.parse();
    let file = translate::translate(data, ast);

    if let Err(e) = generate::generate(file) {
        panic!("{}", e);
    }
}
