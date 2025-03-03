use crate::{
    chunk::{Chunk, OpCode, Value},
    compiler::Compiler,
};
use std::collections::{hash_map, HashMap};

pub struct VM {
    ip: usize,
    chunk: Option<Chunk>,
    stack: Vec<Value>,
    globals_table: HashMap<String, Value>,
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
            globals_table: HashMap::new(),
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

    fn peek(&self, distance: usize) -> &Value {
        &self.stack[self.stack.len() - 1 - distance]
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
            dbg!(&self.chunk);
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
                OpCode::Divide => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a / b));
                        }
                        _ => return InterpretResult::RuntimeError
                    }

                }
                OpCode::Print => {
                    let value = self.stack.pop().unwrap();
                    match value {
                        Value::String(str) => println!("{:#?}", str),
                        Value::Number(numb) => println!("{}", numb),
                        Value::Boolean(bool) => println!("{}", bool),
                        _ => println!("Nil"),
                    }
                }

                OpCode::DefineGlobal(index) => {
                    if let Value::String(key) = &chunk.constants[*index as usize] {
                        let value = self.peek(0).clone();
                        self.globals_table.insert(key.clone(), value);
                        self.stack.pop();
                    } else {
                        // Handle error case - the constant wasn't a string
                        return InterpretResult::RuntimeError;
                    }
                }

                OpCode::GetGlobal(index) => {
                    if let Value::String(key) = &chunk.constants[*index as usize] {
                        match self.globals_table.get(key) {
                            Some(value) => {
                                self.stack.push(value.clone());
                            }
                            None => {
                                eprintln!("Undefined variable '{}'", key);
                                return InterpretResult::RuntimeError;
                            }
                        }
                    } else {
                        return InterpretResult::RuntimeError;
                    }
                }

                OpCode::Return => return InterpretResult::Ok,
                _ => println!("other"),
            }
        }
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
