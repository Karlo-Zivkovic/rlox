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
        let mut chunk = Chunk::new();
        let compiler = Compiler::new(source);

        if !compiler.compile(&mut chunk) {
            return InterpretResult::CompileError;
        }
        dbg!(&chunk);

        self.run()
    }

    fn run(&self) -> InterpretResult {
        // Simulate VM execution logic
        println!("Running VM...");
        InterpretResult::Ok
    }
}
