use logos_derive::Logos;

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
enum Token {
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    String,
}

fn main() {
    println!("Hello, world!");
}
