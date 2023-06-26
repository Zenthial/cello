use crate::lexer::{Lexeme, Lexer, Token};
use crate::syntax::Language;

use rowan::{GreenNode, GreenNodeBuilder, Language as _};

use std::sync::Arc;

pub struct Parser {
    lexemes: Box<[Lexeme]>,
    cursor: usize,
    builder: GreenNodeBuilder<'static>,
}

impl Parser {
    pub fn new(parse_str: &str) -> Self {
        Parser {
            lexemes: Lexer::new(parse_str).collect(),
            builder: GreenNodeBuilder::new(),
            cursor: 0,
        }
    }

    pub fn parse(mut self) -> GreenNode {
        self.builder.start_node(Language::kind_to_raw(Token::Root));

        // there should be 4 divisions
        for _ in 0..4 {
            self.parse_division();
        }

        self.builder.finish_node();

        self.builder.finish()
    }

    fn parse_division(&mut self) {
        let div_token = &self.lexemes[self.cursor];

        if div_token.token == Token::Division {
            self.builder
                .start_node(Language::kind_to_raw(Token::DivisionRoot));
            self.add_token(div_token.token.clone(), div_token.kind.clone());

            loop {
                if self.cursor >= self.lexemes.len() {
                    break;
                }
                let token = &self.lexemes[self.cursor];

                if token.token == Token::Division {
                    break;
                }

                self.add_token(token.token.clone(), token.kind.clone());
            }

            self.builder.finish_node();
        }
    }

    fn add_token(&mut self, tok: Token, text: Arc<str>) {
        self.builder.token(Language::kind_to_raw(tok), &text);
        self.cursor += 1;
    }
}
