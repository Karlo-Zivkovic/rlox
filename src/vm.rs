use crate::{
    chunk::{Chunk, OpCode, Value},
    compiler::Compiler,
};

pub struct VM {
    ip: usize,
    chunk: Option<Chunk>,
    stack: Vec<Value>,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new() -> Self {
        Self {
            ip: 0,
            chunk: None,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut chunk = Chunk::new();
        let compiler = Compiler::new(source);

        if !compiler.compile(&mut chunk) {
            return InterpretResult::CompileError;
        };

        self.chunk = Some(chunk);

        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        while self.ip
            < self
                .chunk
                .as_ref()
                .expect("Chunk should be initialized")
                .code
                .len()
        {
            match self.advance() {
                OpCode::Constant(index) => {
                    let constant = self.read_constant(index);
                    self.push(constant);
                    println!("Constant({})", index);
                }
                _ => println!("other"),
            }
        }
        println!("Running VM...");
        InterpretResult::Ok
    }

    fn read_constant(&self, index: &u8) -> Value {
        self.chunk
            .as_ref()
            .expect("Chunk should be initialized")
            .constants[self.ip]
    }

    fn advance(&mut self) -> &OpCode {
        let chunk = self.chunk.as_ref().expect("Chunk should be initialized");
        let opcode = &chunk.code[self.ip];
        self.ip += 1;
        opcode
    }
}
