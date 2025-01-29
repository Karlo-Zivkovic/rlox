use crate::{
    chunk::{Chunk, OpCode, Value},
    scanner::Scanner,
    token::{Token, TokenType},
};
use std::collections::HashMap;
use std::sync::LazyLock;
use std::{cell::RefCell, process};

type ParseFn = fn(&Parser);

#[derive(Debug, Clone)]
pub struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

// struct Local<'a> {
//     token: Token<'a>,
// }

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Precedence {
    None,       // No precedence
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,    // Primary expressions
}
impl Precedence {
    pub fn next(&self) -> Option<Self> {
        match self {
            Precedence::None => Some(Precedence::Assignment),
            Precedence::Assignment => Some(Precedence::Or),
            Precedence::Or => Some(Precedence::And),
            Precedence::And => Some(Precedence::Equality),
            Precedence::Equality => Some(Precedence::Comparison),
            Precedence::Comparison => Some(Precedence::Term),
            Precedence::Term => Some(Precedence::Factor),
            Precedence::Factor => Some(Precedence::Unary),
            Precedence::Unary => Some(Precedence::Call),
            Precedence::Call => Some(Precedence::Primary),
            Precedence::Primary => None, // No next precedence
        }
    }
}

struct Parser<'a> {
    scanner: Scanner<'a>,
    chunk: RefCell<&'a mut Chunk>,
    current: RefCell<Option<Token<'a>>>,
    previous: RefCell<Option<Token<'a>>>,
    had_error: RefCell<bool>,
    panic_mode: RefCell<bool>,
    locals: RefCell<Vec<Token<'a>>>,
}

impl<'a> Parser<'a> {
    fn new(chunk: &'a mut Chunk, source: &'a str) -> Self {
        Self {
            chunk: RefCell::new(chunk),
            scanner: Scanner::new(source),
            current: RefCell::new(None),
            previous: RefCell::new(None),
            had_error: RefCell::new(false),
            panic_mode: RefCell::new(false),
            locals: RefCell::new(Vec::new()),
        }
    }

    fn current_token(&self) -> Option<Token<'a>> {
        self.current.borrow().clone()
    }
    fn previous_token(&self) -> Option<Token> {
        self.previous.borrow().clone()
    }

    fn run(&'a self) {
        *self.had_error.borrow_mut() = false;
        *self.panic_mode.borrow_mut() = false;
        self.advance();

        while !self.match_token_type(TokenType::Eof) {
            if self.match_token_type(TokenType::Var) {
                self.var_declaration();
            }
            self.statement();
        }
    }

    fn var_declaration(&'a self) {
        self.consume(TokenType::Identifier, "Expect variable name.");

        let token = self.previous_token().expect("Expected previous token");

        // TODO: implement with scope depth in mind
        // self.locals.borrow_mut().push(token.clone());

        let variable_index = self
            .chunk
            .borrow_mut()
            .add_constant(Value::String(token.lexeme.to_string()));

        if self.match_token_type(TokenType::Equal) {
            self.expression();
        } else {
            self.emit_byte(OpCode::Nil);
        }
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );
        self.emit_byte(OpCode::DefineGlobal(variable_index));
    }

    fn advance(&self) {
        *self.previous.borrow_mut() = self.current_token();

        loop {
            let token = self.scanner.scan_token();

            if token.token_type != TokenType::Error {
                *self.current.borrow_mut() = Some(token);
                break;
            }
        }
    }

    fn match_token_type(&'a self, token_type: TokenType) -> bool {
        if let Some(token) = &self.current_token() {
            if token.token_type == token_type {
                self.advance();
                return true;
            }
        }
        false
    }

    fn statement(&'a self) {
        if self.match_token_type(TokenType::Print) {
            self.print_statement();
        }
    }

    fn print_statement(&'a self) {
        self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        self.emit_byte(OpCode::Print);
    }

    fn consume(&'a self, token_type: TokenType, message: &str) {
        if let Some(token) = self.current_token() {
            if token.token_type == token_type {
                self.advance();
                return;
            }
        }
        self.error_at(self.current_token(), message);
    }

    fn emit_bytes(&self, byte1: OpCode, byte2: OpCode) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_byte(&self, opcode: OpCode) {
        self.chunk.borrow_mut().write(opcode);
    }

    fn expression(&self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn error_at(&self, token: Option<Token>, message: &str) {
        if *self.panic_mode.borrow() {
            return;
        }

        *self.panic_mode.borrow_mut() = true;

        match token {
            Some(token) => {
                eprint!("[line {}] Error", token.line);
                if token.token_type == TokenType::Eof {
                    eprint!(" at end");
                } else if token.token_type == TokenType::Error {
                    // Nothing
                } else {
                    eprint!(" at '{}'", &token.lexeme[0..token.lexeme.len()]);
                }
            }
            _ => return,
        };
        eprint!(": {}", message);
        *self.panic_mode.borrow_mut() = true;
    }

    fn parse_precedence(&self, precedence: Precedence) {
        self.advance();

        let previous_token_type = self.previous_token().unwrap().token_type;

        // Handle the prefix rule
        if let Some(rule) = self.get_rule(&previous_token_type) {
            if let Some(prefix_rule) = rule.prefix {
                prefix_rule(self);
            } else {
                self.error_at(self.previous_token(), "Expected an expression.");
                return;
            }
        } else {
            self.error_at(self.previous_token(), "Invalid token.");
            return;
        }

        // Parse infix expressions as long as precedence allows
        while precedence
            <= self
                .get_rule(&self.current_token().unwrap().token_type)
                .map(|rule| rule.precedence)
                .unwrap_or(Precedence::None)
        {
            self.advance();
            let previous_token_type = self.previous_token().unwrap().token_type;

            if let Some(rule) = self.get_rule(&previous_token_type) {
                if let Some(infix_rule) = rule.infix {
                    infix_rule(self);
                }
            }
        }
    }

    fn get_rule(&self, token_type: &TokenType) -> Option<&ParseRule> {
        RULES.get(token_type)
    }

    fn grouping(parser: &Parser) {}
    fn unary(parser: &Parser) {}

    fn binary(parser: &Parser) {
        let operator_type = parser.previous_token().unwrap().token_type;
        if let Some(rule) = parser.get_rule(&operator_type) {
            if let Some(next_precedence) = rule.precedence.next() {
                parser.parse_precedence(next_precedence);
            }
        };

        match operator_type {
            TokenType::BangEqual => {
                parser.emit_bytes(OpCode::Equal, OpCode::Not);
            }
            TokenType::EqualEqual => {
                parser.emit_byte(OpCode::Equal);
            }
            TokenType::Greater => {
                parser.emit_byte(OpCode::Greater);
            }
            TokenType::GreaterEqual => {
                parser.emit_bytes(OpCode::Less, OpCode::Not);
            }
            TokenType::Less => {
                parser.emit_byte(OpCode::Less);
            }
            TokenType::LessEqual => {
                parser.emit_bytes(OpCode::Greater, OpCode::Not);
            }
            TokenType::Plus => {
                parser.emit_byte(OpCode::Add);
            }
            TokenType::Minus => {
                parser.emit_byte(OpCode::Subtract);
            }
            TokenType::Star => {
                parser.emit_byte(OpCode::Multiply);
            }
            TokenType::Slash => {
                parser.emit_byte(OpCode::Divide);
            }
            _ => {
                // Handle unexpected cases (this should be unreachable)
                return;
            }
        }
    }

    fn resolve_local(&self, token: &Token) -> Option<u8> {
        self.locals
            .borrow()
            .iter()
            .rposition(|local| local.lexeme == token.lexeme)
            .map(|pos| pos as u8)
    }

    fn variable(parser: &Parser) {
        let token = parser.previous_token().expect("Expect previous token");
        // First determine if it's a local or global variable and get the index
        let (get_op, set_op) = {
            if let Some(local_index) = parser.resolve_local(&token) {
                (OpCode::GetLocal(local_index), OpCode::SetLocal(local_index))
            } else {
                let global_index = parser
                    .chunk
                    .borrow_mut()
                    .add_constant(Value::String(token.lexeme.to_string()));
                (
                    OpCode::GetGlobal(global_index),
                    OpCode::SetGlobal(global_index),
                )
            }
        };

        // Check if it's an assignment by peeking at the current token
        if let Some(current) = parser.current_token() {
            if current.token_type == TokenType::Equal {
                parser.advance(); // Consume the equals sign
                parser.expression(); // Now uses &self directly
                parser.emit_byte(set_op);
                return;
            }
        }

        // If not an assignment, emit the get operation
        parser.emit_byte(get_op);
    }

    fn string(parser: &Parser) {}

    fn number(parser: &Parser) {
        let source_str = parser.previous_token().unwrap().lexeme;
        let my_int = source_str.parse::<f64>().unwrap();

        let constant_index = parser
            .chunk
            .borrow_mut()
            .add_constant(Value::Number(my_int));

        parser.emit_byte(OpCode::Constant(constant_index));
    }

    // fn emit_constant(&self, value: Value) {}

    fn and_(parser: &Parser) {}
    fn literal(parser: &Parser) {}
    fn or_(parser: &Parser) {}
}

pub struct Compiler<'a> {
    source: &'a str,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub fn compile(&self, chunk: &mut Chunk) -> bool {
        let parser = Parser::new(chunk, &self.source);
        parser.run();
        // println! {"{:#?}", chunk};
        // process::exit(0);

        self.end_compiler(&parser);
        return !*parser.had_error.borrow();
    }

    fn end_compiler(&self, parser: &Parser) {
        parser.emit_byte(OpCode::Return);
    }
}

pub static RULES: LazyLock<HashMap<TokenType, ParseRule>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(
        TokenType::LeftParen,
        ParseRule {
            prefix: Some(Parser::grouping),
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::RightParen,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::LeftBrace,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::RightBrace,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::Comma,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::Dot,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::Minus,
        ParseRule {
            prefix: Some(Parser::unary),
            infix: Some(Parser::binary),
            precedence: Precedence::Term,
        },
    );
    map.insert(
        TokenType::Plus,
        ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Term,
        },
    );
    map.insert(
        TokenType::Semicolon,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::Slash,
        ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Factor,
        },
    );
    map.insert(
        TokenType::Star,
        ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Factor,
        },
    );
    map.insert(
        TokenType::Bang,
        ParseRule {
            prefix: Some(Parser::unary),
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::BangEqual,
        ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Equality,
        },
    );
    map.insert(
        TokenType::Equal,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::EqualEqual,
        ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Equality,
        },
    );
    map.insert(
        TokenType::Greater,
        ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Comparison,
        },
    );
    map.insert(
        TokenType::GreaterEqual,
        ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Comparison,
        },
    );
    map.insert(
        TokenType::Less,
        ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Comparison,
        },
    );
    map.insert(
        TokenType::LessEqual,
        ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Comparison,
        },
    );
    map.insert(
        TokenType::Identifier,
        ParseRule {
            prefix: Some(Parser::variable),
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::String,
        ParseRule {
            prefix: Some(Parser::string),
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::Number,
        ParseRule {
            prefix: Some(Parser::number),
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::And,
        ParseRule {
            prefix: None,
            infix: Some(Parser::and_),
            precedence: Precedence::And,
        },
    );
    map.insert(
        TokenType::Class,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::Else,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::False,
        ParseRule {
            prefix: Some(Parser::literal),
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::For,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::Fun,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::If,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::Nil,
        ParseRule {
            prefix: Some(Parser::literal),
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::Or,
        ParseRule {
            prefix: None,
            infix: Some(Parser::or_),
            precedence: Precedence::Or,
        },
    );
    map.insert(
        TokenType::Print,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::Return,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::Super,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::This,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::True,
        ParseRule {
            prefix: Some(Parser::literal),
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::Var,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::While,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::Error,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map.insert(
        TokenType::Eof,
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    );
    map
});
