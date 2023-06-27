use crate::parser::{Infix, Instruction, Value};

use std::sync::Arc;

fn src_to_string(src: Value) -> (String, Option<Arc<str>>) {
    match src {
        Value::Number(i) => (format!("{}", i), None),
        Value::Identifier(ident) => (String::from(&*ident.0), Some(ident.0.clone())),
        Value::String(str) => (String::from(&*str), None),
    }
}

fn generate_move(infix: Infix) -> (String, Vec<Arc<str>>) {
    let mut possible_idents_to_generate = vec![];
    possible_idents_to_generate.push(infix.dest.0.clone());

    let (src, push_infix) = src_to_string(infix.src);
    if let Some(inf) = push_infix {
        possible_idents_to_generate.push(inf);
    }

    let text = format!("{} = {};\n", infix.dest.0, src);
    (text, possible_idents_to_generate)
}

fn generate_add(infix: Infix) -> String {
    let (src, _) = src_to_string(infix.src);

    return format!("{} += {};\n", infix.dest.0, src);
}

pub fn translate(instructions: Vec<Instruction>) -> String {
    let mut variable_definitions = String::new();
    let mut defined_variables = vec![];
    let mut operations = String::new();

    for inst in instructions {
        match inst {
            Instruction::Move(infix) => {
                let (operation_text, possible_idents) = generate_move(infix);

                operations += operation_text.as_str();
                for ident in possible_idents {
                    if !defined_variables.contains(&ident) {
                        variable_definitions += format!("let mut {};\n", ident).as_str();
                        defined_variables.push(ident);
                    }
                }
            }
            Instruction::Add(infix) => {
                let operation_text = generate_add(infix);
                operations += operation_text.as_str();
            }
            Instruction::Print(values) => {
                let mut print_string = String::from("println!(\"");
                let mut arguments_string = String::new();
                for v in values {
                    print_string += "{}";
                    arguments_string += format!(", format!(\"{{}}\", {})", v).as_str();
                }

                print_string += "\"";
                arguments_string += ");\n";
                operations += format!("{}{}", print_string, arguments_string).as_str();
            }
        }
    }

    return format!("fn main() {{\n{}\n{}}}", variable_definitions, operations);
}
