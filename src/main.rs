use std::{env, fs, process};
use vm::VM;

pub mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <script>", args[0]);
        process::exit(64);
    }

    if let Err(e) = run_file(&args[1]) {
        eprintln!("Error: {}", e);
        process::exit(65);
    }
}

fn run_file(filename: &str) -> Result<(), String> {
    let vm = VM::new();

    let source = fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read file '{}': {}", filename, err))?;

    match vm.interpret(&source) {
        Ok(()) => Ok(()),
        Err(err) => Err(format!("Runtime error: {}", err)),
    }
}
