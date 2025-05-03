// Main file

mod vm;
mod lexer;
mod parser;

use lexer::{Lexer};
use parser::*;
use vm::{Instruction, VM, generate};

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: c4_rust <source_file.c>");
        process::exit(1);
    }

    let filename = &args[1];
    let source_code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error: Could not open {}: {}", filename, err);
            process::exit(1);
        }
    };
    println!("Successfully read {} bytes from {}", source_code.len(), filename);

    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.tokenize();
    println!("Lexing successful. Found {} tokens.", tokens.len());

    let mut parser = Parser::new(tokens);
    let ast_nodes = parser.parse_program();
    println!("Parsing successful. Found {} nodes.", ast_nodes.len());

    let (mut instructions, functions) = generate(ast_nodes);

    // if let Some(main_function) = functions.get("main") {
    //     instructions.insert(0, Instruction::CALL(main_function.name.clone())); // Start execution from main
    // } else {
    //     panic!("No main function found!");
    // }

    let mut vm = VM::new(instructions, functions);
    let result = vm.run();
    println!("Instructions executed successfully with result: {}", result);
}