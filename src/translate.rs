use crate::parser::{Condition, Data, DataType, IdentifierType, Infix, Instruction, Value};

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

    let text = if let IdentifierType::Alphanumeric(_) = infix.right.kind {
        format!("{} = {}.to_zeroed_string();\n", infix.right.name, left)
    } else {
        format!("{} = {}.into();\n", infix.right.name, left)
    };

    (text, possible_idents_to_generate)
}

fn generate_add(infix: Infix) -> String {
    let needs_ref = match &infix.left {
        Value::Number(_) => "",
        Value::Identifier(_) => "&",
        Value::String(_) => "",
    };

    let (left, _) = value_to_string(infix.left);

    return format!("{} += {needs_ref}{};\n", infix.right.name, left);
}

fn generate_multiply(infix: Infix) -> String {
    let (left, _) = value_to_string(infix.left);

    return format!(
        "{name} *= {name}.convert(&{mult});\n",
        name = infix.right.name,
        mult = left
    );
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

fn translate_core(instructions: Vec<Instruction>) -> (Vec<Arc<str>>, String) {
    let mut defined_variables = vec![];
    let mut operations = String::new();

    for inst in instructions {
        match inst {
            Instruction::Move(infix) => {
                let (operation_text, possible_idents) = generate_move(infix);

                operations += operation_text.as_str();
                for ident in possible_idents {
                    if !defined_variables.contains(&ident) {
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
                let (defined, instruction_text) = translate_core(insts);
                for ident in defined {
                    if !defined_variables.contains(&ident) {
                        defined_variables.push(ident);
                    }
                }
                operations += operation_text.as_str();
                operations += instruction_text.as_str();
                operations += "}\n"
            }
        }
    }

    (defined_variables, operations)
}

pub fn translate(data: Vec<Data>, instructions: Vec<Instruction>) -> String {
    let (used_variables, operations) = translate_core(instructions);
    let mut variable_definitions = String::new();
    // let s = data.iter().fold(0, |acc, x| {
    //     if let DataType::Picture(ident_type) = &x.data_type {
    //         match ident_type {
    //             IdentifierType::Numeric(size) | IdentifierType::Alphanumeric(size) => {
    //                 if acc < *size {
    //                     return *size;
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }
    //
    //     acc
    // });

    for var in data {
        let type_str = match var.data_type {
            DataType::Picture(ident_type) => match ident_type {
                IdentifierType::Numeric(s) => {
                    format!(": Num<{}> = Num::zero();", s)
                }
                IdentifierType::Alphanumeric(s) => {
                    format!("= String::from(\"{}\")", "0".repeat(s as usize))
                }
                _ => unreachable!(),
            },
            _ => unimplemented!(),
        };
        if used_variables.contains(&var.name) {
            variable_definitions += &format!("let mut {}{};\n", var.name, type_str);
        } else {
            variable_definitions += &format!("let {}{};\n", var.name, type_str);
        }
    }

    return format!(
        "#![allow(unused)]\n\nuse conum::{{Num, NumFrom}};\nfn main() {{\n{}\n{}}}",
        variable_definitions, operations
    );
}
