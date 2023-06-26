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

        // there should be 3-4 divisions
        for _ in 0..3 {
            self.parse_division();
        }

        if self.cursor != self.lexemes.len() {
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

                self.parse_functionality(token.token.clone(), token.kind.clone());
            }

            self.builder.finish_node();
        }
    }

    fn parse_functionality(&mut self, token: Token, text: Arc<str>) {
        match token {
            Token::Add | Token::Move | Token::Multiply => {
                self.builder.start_node(Language::kind_to_raw(token));
                self.cursor += 1;
                if !self.parse_infix() {
                    panic!("infix did not have proper arguments");
                }
                self.builder.finish_node();
            }
            _ => self.add_token(token, text),
        }
    }

    fn parse_infix(&mut self) -> bool {
        let left = &self.lexemes[self.cursor].clone();
        let infix = &self.lexemes[self.cursor + 1].clone();
        let right = &self.lexemes[self.cursor + 2].clone();

        let binding = infix.kind.to_lowercase();
        let infix_str = binding.as_str();
        match infix_str {
            "by" | "to" => {
                self.add_token(left.token, left.kind.clone());
                self.add_token(infix.token, infix.kind.clone());
                self.add_token(right.token, right.kind.clone());
            }
            _ => {
                return false;
            }
        }

        self.cursor += 3;
        return true;
    }

    fn add_token(&mut self, tok: Token, text: Arc<str>) {
        self.builder.token(Language::kind_to_raw(tok), &text);
        self.cursor += 1;
    }
}
