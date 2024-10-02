mod error;
mod interpreter;
mod parser;

use std::env;
use std::fs;
use std::process;
use crate::error::BrainfuckError;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} --file <filename> or {} --cmd <brainfuck_code>", args[0], args[0]);
        process::exit(1);
    }

    let (flag, input) = (&args[1], &args[2]);
    let code = match flag.as_str() {
        "--file" => match fs::read_to_string(input) {
            Ok(content) => content,
            Err(_) => {
                eprintln!("Error: {}", BrainfuckError::FileNotFound(input.to_string()));
                process::exit(1);
            }
        },
        "--cmd" => input.to_string(),
        _ => {
            eprintln!("Error: {}", BrainfuckError::InvalidArguments("Invalid flag. Either use --file or --cmd.".to_string()));
            process::exit(1);
        }
    };

    match parser::parse(&code) {
        Ok(instructions) => {
            let mut interpreter = interpreter::Interpreter::new();
            if let Err(err) = interpreter.run(&instructions) {
                eprintln!("Runtime error: {}", err);
                process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("Parsing error: {}", err);
            process::exit(1);
        }
    }
}
