use crate::{
    chunk::Chunk,
    scanner::Scanner,
    token::{Token, TokenType},
};

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

    fn advance(&mut self, scanner: &'a mut Scanner<'a>) {
        self.previous = self.current.take();

        loop {
            let token = scanner.scan_token();
            if token.token_type != TokenType::Error {
                self.current = Some(token);
                break;
            }
        }
    }
}

pub struct Compiler {}

impl Compiler {
    fn new() -> Self {
        Self {}
    }

    pub fn compile(source: &str, chunk: &Chunk) -> bool {
        let mut scanner = Scanner::new(source);
        let mut parser = Parser::new();
        parser.had_error = false;
        parser.panic_mode = false;

        parser.advance(&mut scanner);
        return !parser.had_error;
    }
}
