use crate::{chunk::Chunk, scanner::Scanner, token::Token, vm::InterpretResult};

struct Parser<'a> {
    current: Option<Token<'a>>,
    previous: Option<Token<'a>>,
    had_error: bool,
    panic_mode: bool,
}

impl<'a> Parser<'a> {
    fn new() -> Self {
        Self {
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
        }
    }
}

// THINK ABOUT THIS AND CONTINUE WITH IT
pub fn compile(source: &str, chunk: &Chunk) -> InterpretResult {
    let scanner = Scanner::new(source);
    let parser = Parser::new();
    parser.had_error = false;
    parser.panic_mode = false;
}
