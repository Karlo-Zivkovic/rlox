use crate::{chunk::Chunk, compiler::Compiler};

pub struct VM {}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&self, source: &str) -> InterpretResult {
        let chunk = Chunk::new();

        if !Compiler::compile(source, &chunk) {
            return InterpretResult::CompileError;
        }

        let result = self.run();
        result
    }

    fn run(&self) -> InterpretResult {
        // Simulate VM execution logic
        println!("Running VM...");
        InterpretResult::Ok
    }
}
