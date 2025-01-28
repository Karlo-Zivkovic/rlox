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
            let chunk = self.chunk.as_ref().expect("Chunk should be initialized");
            let opcode = &chunk.code[self.ip];
            self.ip += 1;

            match opcode {
                OpCode::Constant(index) => {
                    let constant = chunk.constants[*index as usize].clone();
                    self.stack.push(constant);
                }
                OpCode::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a + b));
                        }
                        _ => return InterpretResult::RuntimeError,
                    }
                }
                OpCode::Print => {
                    let value = self.stack.pop().unwrap();
                    println!("{:#?}", value);
                }
                OpCode::Return => return InterpretResult::Ok,
                _ => println!("other"),
            }
        }

        dbg!(&self.chunk);
        println!("Running VM...");
        InterpretResult::Ok
    }

    // fn read_constant(&self, index: &u8) -> Value {
    //     self.chunk
    //         .as_ref()
    //         .expect("Chunk should be initialized")
    //         .constants[*index as usize]
    //         .clone()
    // }

    // fn advance(&mut self) -> &OpCode {
    //     let chunk = self.chunk.as_ref().expect("Chunk should be initialized");
    //     let opcode = &chunk.code[self.ip];
    //     self.ip += 1;
    //     opcode
    // }
}
