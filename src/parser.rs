#![allow(dead_code)]
use crate::lexer::{self, Tail};
use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub enum DataType {
    Picture(IdentifierType),
    Other,
}

#[derive(Debug)]
pub struct Data {
    pub level: i32,
    pub name: Arc<str>,
    pub data_type: DataType,
}

trait Derive {
    fn derive(val: &str) -> Self;
}

#[derive(Debug)]
pub enum Condition {
    GreaterThan,
    LessThan,
    EqualTo,
}

impl ToString for Condition {
    fn to_string(&self) -> String {
        match self {
            Condition::EqualTo => String::from("="),
            Condition::GreaterThan => String::from(">"),
            Condition::LessThan => String::from("<"),
        }
    }
}

impl Derive for Condition {
    fn derive(val: &str) -> Self {
        match val {
            "greater" => Condition::GreaterThan,
            "less" => Condition::LessThan,
            "equal" => Condition::EqualTo,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IdentifierType {
    Numeric(u32),
    Alphabetic,
    Alphanumeric(u32),
    ImplicitDecimal,
    Sign,
    AssumedDecimal,
}

impl IdentifierType {
    fn parse_type(string: Arc<str>) -> Self {
        let chars: Vec<char> = string.chars().collect();
        let num = if let Some(c) = chars.get(1) {
            if *c == '(' {
                let mut num = String::new();
                let mut index = 2;
                loop {
                    let digit = chars[index];
                    if digit == ')' {
                        break;
                    } else {
                        num.push(digit);
                        index += 1;
                        if index > 100 {
                            panic!("forgot closing )");
                        }
                    }
                }
                let amount: u32 = num
                    .parse()
                    .expect("defined var has () without a number inside");

                amount
            } else {
                // hack, need to parse for the period that ends the line. right now i am just
                // counting it and then subtracting 1
                chars.len() as u32 - 1
            }
        } else {
            1
        };

        if chars[0] == '9' {
            IdentifierType::Numeric(num)
        } else if chars[0] == 'x' {
            IdentifierType::Alphanumeric(num)
        } else {
            unimplemented!()
        }
    }
}

#[derive(Debug)]
pub struct Ident {
    pub name: Arc<str>,
    pub kind: IdentifierType,
}

impl Ident {
    pub fn new(name: &str, kind: IdentifierType) -> Self {
        Self {
            name: Arc::from(name),
            kind,
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Number(i32),
    Identifier(Ident),
    String(Arc<str>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Value::Number(i) => write!(f, "{}", i),
            Value::Identifier(ident) => write!(f, "{}", ident.name),
            Value::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

impl Derive for Value {
    fn derive(val: &str) -> Self {
        let try_parse = val.parse::<i32>();
        if let Ok(value) = try_parse {
            return Self::Number(value);
        }

        if val.starts_with('\"') && val.ends_with('\"') {
            let actual_string = val.strip_prefix('\"').unwrap().strip_suffix('\"').unwrap();
            return Self::String(Arc::from(actual_string));
        }

        Value::Identifier(Ident::new(val, IdentifierType::Numeric(0)))
    }
}

#[derive(Debug)]
pub struct Infix {
    pub left: Value,
    pub right: Ident,
}

#[derive(Debug)]
pub enum Instruction {
    Move(Infix),
    Add(Infix),
    Multiply(Infix),
    Print(Vec<Value>),
    Repeat {
        left: Value,
        condition: Condition,
        right: Value,
        insts: Vec<Instruction>,
    },
}

pub struct Parser<'a> {
    contents: &'a str,
    lines: VecDeque<&'a str>,
    look_up: Vec<Data>,
}

impl<'a> Parser<'a> {
    pub fn new(contents: &'a str) -> Self {
        Self {
            contents,
            lines: VecDeque::new(),
            look_up: vec![],
        }
    }

    pub fn parse(mut self) -> (Vec<Data>, Vec<Instruction>) {
        let pro_split: Vec<&str> = self.contents.split("procedure division.").collect();
        let data_split: Vec<&str> = pro_split[0].split("data division.").collect();

        let procedure = pro_split[1].trim_start();
        let data = data_split[1].trim_start();
        let variables = self.parse_data(data);
        self.look_up = variables;
        let instructions = self.parse_procedure(procedure);

        (self.look_up, instructions)
    }

    fn parse_data(&self, data_segment: &'a str) -> Vec<Data> {
        let working_storage_split: Vec<&str> =
            data_segment.split("working-storage section.").collect();

        let working_storage_section = working_storage_split[1];
        let working_storage_data: Vec<&str> = working_storage_section.lines().collect();
        let ws_data = self.parse_working_storage(working_storage_data);

        ws_data
    }

    fn parse_working_storage(&self, working_storage_lines: Vec<&str>) -> Vec<Data> {
        working_storage_lines
            .iter()
            .filter(|l| l.trim() != "")
            .map(|line| {
                let trimmed = line.trim_start();
                let variable = self.parse_variable(trimmed);

                return variable;
            })
            .collect()
    }

    fn parse_variable(&self, line: &str) -> Data {
        println!("{}", line);
        let words = get_words(line);

        let level: i32 = words[0].parse().expect("cannot convert level str into i32");
        let name = words[1].clone();
        let v_type = words[2].clone();
        let variable_definition = words[3].clone();
        let variable_identifier_type = IdentifierType::parse_type(variable_definition);
        let var_type = match &*v_type {
            "pic" => DataType::Picture(variable_identifier_type),
            _ => unimplemented!(),
        };

        Data {
            level,
            name,
            data_type: var_type,
        }
    }

    fn parse_procedure(&mut self, procedure: &'a str) -> Vec<Instruction> {
        let mut instructions = vec![];
        self.lines = procedure.lines().collect();
        loop {
            let line = if let Some(l) = self.lines.pop_front() {
                l
            } else {
                break;
            };

            // let words: Vec<&str> = line.split_whitespace().collect();
            let words: Vec<Arc<str>> = walk_line(line);
            let str_words: Vec<&str> = words.iter().map(|w| &**w).collect();
            let instruction = self.generate_instruction(str_words);
            instructions.push(instruction);
        }

        instructions
    }

    fn generate_instruction(&mut self, words: Vec<&str>) -> Instruction {
        let instruction = words[0];
        let tail = words.tail();

        match instruction {
            "move" | "add" | "multiply" => self.generate_infix_instruction(instruction, tail),
            "display" => self.generate_print(tail),
            "perform" => self.generate_perform(tail),
            _ => unimplemented!(),
        }
    }

    fn generate_print(&self, operands: &[&str]) -> Instruction {
        let values: Vec<Value> = operands.iter().map(|o| Value::derive(o)).collect();

        Instruction::Print(values)
    }

    fn generate_infix_instruction(&self, inst: &str, operands: &[&str]) -> Instruction {
        let src = operands[0];
        let dest = operands[2];

        let DataType::Picture(i_type) = self 
            .look_up
            .iter()
            .find(|v| &*v.name == dest)
            .unwrap()
            .data_type
            .clone()
        else {
            unimplemented!()
        };

        let infix = Infix {
            left: Value::derive(src),
            right: Ident::new(dest, i_type),
        };

        match inst {
            "move" => Instruction::Move(infix),
            "add" => Instruction::Add(infix),
            "multiply" => Instruction::Multiply(infix),
            _ => unreachable!(),
        }
    }

    fn generate_perform(&mut self, operands: &[&str]) -> Instruction {
        let operation = operands[0];
        let tail = operands.tail();

        match operation {
            "until" => self.generate_repeat(tail),
            _ => unimplemented!(),
        }
    }

    fn generate_repeat(&mut self, operands: &[&str]) -> Instruction {
        let left = Value::derive(operands[0]);
        let condition = Condition::derive(operands[1]);
        let right = Value::derive(operands[3]);

        let mut instructions = vec![];
        loop {
            let line = if let Some(l) = self.lines.pop_front() {
                l
            } else {
                panic!("Missing corresponding 'end-perform'");
            };

            if line.contains("end-perform.") {
                break;
            }

            // let words: Vec<&str> = line.split_whitespace().collect();
            let words: Vec<Arc<str>> = walk_line(line);
            let str_words: Vec<&str> = words.iter().map(|w| &**w).collect();
            let instruction = self.generate_instruction(str_words);
            instructions.push(instruction);
        }

        Instruction::Repeat {
            left,
            condition,
            right,
            insts: instructions,
        }
    }
}

fn get_words(mut line: &str) -> Vec<Arc<str>> {
    let mut words = vec![];

    loop {
        let trimmed_chars: Vec<char> = line.chars().collect();
        let take = if trimmed_chars[0] == '"' {
            lexer::take_string(line)
        } else {
            lexer::take_until(line, ' ')
        };

        if let Some((word, rest)) = take {
            words.push(word);
            line = rest.trim_start();
        } else {
            words.push(line.into());
            break;
        }
    }

    return words;
}

fn walk_line(line: &str) -> Vec<Arc<str>> {
    let trimmed = line.trim_start();
    let words = get_words(trimmed);

    return words;
}
