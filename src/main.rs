mod error;
mod interpreter;
mod parser;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
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
