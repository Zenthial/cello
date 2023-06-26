use logos::Logos;

use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
#[logos(skip r"[ \t\r\n\f]")]
pub enum Token {
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    StringLiteral,

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

    #[regex("[a-zA-Z_-]+")]
    Identifier,
}

#[derive(Debug)]
pub struct Lexeme {
    token: Token,
    kind: Arc<str>,
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

    pub fn next(&mut self) -> Option<Lexeme> {
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
