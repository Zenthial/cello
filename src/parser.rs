#![allow(dead_code)]
use std::sync::Arc;

#[derive(Debug)]
pub struct Ident(pub Arc<str>);

impl Ident {
    pub fn new(name: &str) -> Self {
        Self(Arc::from(name))
    }
}

#[derive(Debug)]
pub enum Value {
    Number(i32),
    Identifier(Ident),
    String(Arc<str>),
}

impl Value {
    pub fn derive(val: &str) -> Self {
        let try_parse = val.parse::<i32>();
        if let Ok(value) = try_parse {
            return Self::Number(value);
        }

        if val.starts_with('\"') && val.ends_with('\"') {
            let actual_string = val.strip_prefix('\"').unwrap().strip_suffix('\"').unwrap();
            return Self::String(Arc::from(actual_string));
        }

        Value::Identifier(Ident::new(val))
    }
}

#[derive(Debug)]
pub struct Infix {
    pub src: Value,
    pub dest: Ident,
}

#[derive(Debug)]
pub enum Instruction {
    Move(Infix),
    Add(Infix),
}

pub struct Parser {
    contents: Arc<str>,
}

impl Parser {
    pub fn new(contents: &str) -> Self {
        Self {
            contents: Arc::from(contents.to_lowercase()),
        }
    }

    pub fn parse(&self) -> Vec<Instruction> {
        let split: Vec<&str> = self.contents.split("procedure division.").collect();

        let procedure = split.get(1).unwrap().trim_start();
        self.parse_procedure(procedure)
    }

    fn parse_procedure(&self, procedure: &str) -> Vec<Instruction> {
        let mut instructions = vec![];
        for line in procedure.lines() {
            let words: Vec<&str> = line.split_whitespace().collect();
            let instruction = self.generate_instruction(words);
            instructions.push(instruction);
        }

        instructions
    }

    fn generate_instruction(&self, words: Vec<&str>) -> Instruction {
        let instruction = words[0];

        match instruction {
            "move" | "add" => self.generate_infix_instruction(instruction, &words[1..]),
            _ => unimplemented!(),
        }
    }

    fn generate_infix_instruction(&self, inst: &str, operands: &[&str]) -> Instruction {
        let src = operands[0];
        let dest = operands[2];

        let infix = Infix {
            src: Value::derive(src),
            dest: Ident::new(dest),
        };

        match inst {
            "move" => Instruction::Move(infix),
            "add" => Instruction::Add(infix),
            _ => unreachable!(),
        }
    }
}
