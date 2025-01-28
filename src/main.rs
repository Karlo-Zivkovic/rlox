use std::{env, fs, process};
use vm::{InterpretResult, VM};

pub mod chunk;
pub mod compiler;
pub mod scanner;
pub mod token;
pub mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <script>", args[0]);
        process::exit(64);
    }

    run_file(&args[1]);
}

fn run_file(path: &str) {
    let mut vm = VM::new();

    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Failed to read file '{}': {}", path, err);
            process::exit(65); // Exit with error code for file errors
        }
    };

    match vm.interpret(&source) {
        InterpretResult::Ok => {}
        InterpretResult::CompileError => {
            eprintln!("Compilation failed.");
            process::exit(65); // Exit code for compile errors
        }
        InterpretResult::RuntimeError => {
            eprintln!("Runtime error occurred.");
            process::exit(70); // Exit code for runtime errors
        }
    }
}
