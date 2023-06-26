use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

#[derive(
    Logos, Debug, Clone, Copy, PartialEq, Hash, PartialOrd, Ord, Eq, FromPrimitive, ToPrimitive,
)]
#[logos(skip r"[ \t\r\n\f]")]
pub enum Token {
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    StringLiteral = 0,

    // Punctuation
    #[token(".")]
    Period,
    #[token(",")]
    Comma,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,

    // Operators
    #[token("<=")]
    LessEq,
    #[token(">=")]
    GreaterEq,

    #[token("=")]
    // reminder that cobol uses = instead of ==
    Equality,
    #[token(">")]
    LeftAlligator,
    #[token("<")]
    RightAlligator,

    #[regex(r#"[0-9]+"#)]
    Int,

    #[regex("[A-Z]+ DIVISION")]
    Division,

    #[regex("[a-zA-Z_-]+")]
    Identifier,

    Root,
    DivisionRoot,
}

#[derive(Debug)]
pub struct Lexeme {
    pub token: Token,
    pub kind: Arc<str>,
}

impl Display for Lexeme {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?} - {}", self.token, self.kind)
    }
}

pub struct Lexer<'a> {
    source: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source_str: &'a str) -> Self {
        Self {
            source: Token::lexer(source_str),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(tok) = self.source.next()? {
            return Some(Lexeme {
                token: tok,
                kind: Arc::from(self.source.slice()),
            });
        } else {
            return None;
        }
    }
}
