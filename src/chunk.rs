#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    lines: Vec<LineEntry>,
    pub constants: Vec<Value>,
}

#[derive(Debug)]
struct LineEntry {
    line: usize,
    run_length: usize,
}

// maybe define methods/trait on the enum
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, opcode: OpCode) {
        //self.code.push(opcode);
        match opcode {
            OpCode::Constant(index) => {
                self.code.push(OpCode::Constant(index)); // Add your base opcode
            }
            _ => {
                self.code.push(opcode);
            }
        }
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        //println!("Adding constant: {:?}", value);
        self.constants.push(value);
        (self.constants.len() - 1) as u8
    }
}

#[derive(Debug)]
pub enum OpCode {
    Constant(u8),
    Negate,
    ConstantLong,
    Print,
    Jump,
    JumpIfFalse,
    Loop,
    Return,
    Nil,
    True,
    False,
    Pop,
    GetLocal(u8),
    SetLocal(u8),
    GetGlobal(u8),
    SetGlobal(u8),
    DefineGlobal(u8),
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
}
