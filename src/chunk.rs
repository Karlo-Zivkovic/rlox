pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<LineEntry>,
    constants: Vec<Value>,
}

struct LineEntry {
    line: usize,
    run_length: usize,
}

// maybe define methods/trait on the enum
enum Value {
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
}
