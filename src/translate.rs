use crate::parser::{Condition, Infix, Instruction, Value};

use std::sync::Arc;

fn value_to_string(left: Value) -> (String, Option<Arc<str>>) {
    match left {
        Value::Number(i) => (format!("{}", i), None),
        Value::Identifier(ident) => (String::from(&*ident.name), Some(ident.name.clone())),
        Value::String(str) => (String::from(&*str), None),
    }
}

fn generate_move(infix: Infix) -> (String, Vec<Arc<str>>) {
    let mut possible_idents_to_generate = vec![];
    possible_idents_to_generate.push(infix.right.name.clone());

    let (left, push_infix) = value_to_string(infix.left);
    if let Some(inf) = push_infix {
        possible_idents_to_generate.push(inf);
    }

    let text = format!("{} = {};\n", infix.right.name, left);
    (text, possible_idents_to_generate)
}

fn generate_add(infix: Infix) -> String {
    let (left, _) = value_to_string(infix.left);

    return format!("{} += {};\n", infix.right.name, left);
}

fn generate_multiply(infix: Infix) -> String {
    let (left, _) = value_to_string(infix.left);

    return format!("{} *= {};\n", infix.right.name, left);
}

fn generate_repeat(left: Value, condition: Condition, right: Value) -> String {
    let (left_string, _) = value_to_string(left);
    let (right_string, _) = value_to_string(right);
    format!(
        "loop {{\nif {} {} {}{{\nbreak;}}",
        left_string,
        condition.to_string(),
        right_string
    )
}

fn translate_core(instructions: Vec<Instruction>) -> (String, Vec<Arc<str>>, String) {
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
            Instruction::Multiply(infix) => {
                let operation_text = generate_multiply(infix);
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
            Instruction::Repeat {
                left,
                condition,
                right,
                insts,
            } => {
                let operation_text = generate_repeat(left, condition, right);
                let (_, defined, instruction_text) = translate_core(insts);
                for ident in defined {
                    if !defined_variables.contains(&ident) {
                        variable_definitions += format!("let mut {};\n", ident).as_str();
                        defined_variables.push(ident);
                    }
                }
                operations += operation_text.as_str();
                operations += instruction_text.as_str();
                operations += "}\n"
            }
        }
    }

    (variable_definitions, defined_variables, operations)
}

pub fn translate(instructions: Vec<Instruction>) -> String {
    let (variable_definitions, _, operations) = translate_core(instructions);
    return format!("fn main() {{\n{}\n{}}}", variable_definitions, operations);
}
