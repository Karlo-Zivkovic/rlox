use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    start: &'a str,
    current: &'a str,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            start: source,
            current: source,
            line: 1,
        }
    }

    pub fn scan_token(&'a mut self) -> Token<'a> {
        self.skip_whitespace();
        self.start = self.current;

        if let Some(c) = self.advance() {
            match c {
                '(' => return self.make_token(TokenType::LeftParen),
                ')' => return self.make_token(TokenType::RightParen),
                '{' => return self.make_token(TokenType::LeftBrace),
                '}' => return self.make_token(TokenType::RightBrace),
                ';' => return self.make_token(TokenType::Semicolon),
                ',' => return self.make_token(TokenType::Comma),
                '.' => return self.make_token(TokenType::Dot),
                '-' => return self.make_token(TokenType::Minus),
                '+' => return self.make_token(TokenType::Plus),
                '/' => return self.make_token(TokenType::Slash),
                '*' => return self.make_token(TokenType::Star),
                '!' => {
                    if self.match_char('=') {
                        return self.make_token(TokenType::BangEqual);
                    } else {
                        return self.make_token(TokenType::Bang);
                    }
                }
                '=' => {
                    if self.match_char('=') {
                        return self.make_token(TokenType::EqualEqual);
                    } else {
                        return self.make_token(TokenType::Equal);
                    }
                }
                '<' => {
                    if self.match_char('=') {
                        return self.make_token(TokenType::LessEqual);
                    } else {
                        return self.make_token(TokenType::Less);
                    }
                }
                '>' => {
                    if self.match_char('=') {
                        return self.make_token(TokenType::GreaterEqual);
                    } else {
                        return self.make_token(TokenType::Greater);
                    }
                }
                '"' => return self.string(),
                _ if self.is_at_end() => return self.make_token(TokenType::Eof),
                c if self.is_alpha(c) => return self.identifier(),
                c if self.is_digit(c) => return self.number(),
                _ => return self.error_token("Unexpected character."),
            }
        } else {
            return self.make_token(TokenType::Eof);
        }
    }

    fn number(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if self.is_digit(c) {
                self.advance();
            } else {
                break;
            }
        }
        if self.peek() == Some('.') && self.is_digit(self.peek_next().unwrap_or('X')) {
            self.advance();
            while let Some(c) = self.peek() {
                if self.is_digit(c) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        return self.make_token(TokenType::Number);
    }

    fn peek_next(&self) -> Option<char> {
        self.current.chars().next()
    }

    fn peek(&self) -> Option<char> {
        self.current.chars().next()
    }

    fn identifier(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if self.is_alpha(c) || self.is_digit(c) {
                self.advance();
            } else {
                break;
            }
        }
        return self.make_token(self.identifier_type());
    }

    fn identifier_type(&self) -> TokenType {
        match self.current.chars().next() {
            Some('a') => self.check_keyword(1, 2, "nd", TokenType::And),
            Some('c') => self.check_keyword(1, 4, "lass", TokenType::Class),
            Some('e') => self.check_keyword(1, 3, "lse", TokenType::Else),
            Some('f') => {
                if self.current.get(1..2) == Some("a") {
                    self.check_keyword(2, 3, "lse", TokenType::False)
                } else if self.current.get(1..2) == Some("o") {
                    self.check_keyword(2, 1, "r", TokenType::For)
                } else if self.current.get(1..2) == Some("u") {
                    self.check_keyword(2, 1, "n", TokenType::Fun)
                } else {
                    TokenType::Identifier
                }
            }
            Some('i') => self.check_keyword(1, 1, "f", TokenType::If),
            Some('n') => self.check_keyword(1, 2, "il", TokenType::Nil),
            Some('o') => self.check_keyword(1, 1, "r", TokenType::Or),
            Some('p') => self.check_keyword(1, 4, "rint", TokenType::Print),
            Some('r') => self.check_keyword(1, 5, "eturn", TokenType::Return),
            Some('s') => self.check_keyword(1, 4, "uper", TokenType::Super),
            Some('t') => {
                if self.current.get(1..2) == Some("h") {
                    self.check_keyword(2, 2, "is", TokenType::This)
                } else if self.current.get(1..2) == Some("r") {
                    self.check_keyword(2, 2, "ue", TokenType::True)
                } else {
                    TokenType::Identifier
                }
            }
            Some('v') => self.check_keyword(1, 2, "ar", TokenType::Var),
            Some('w') => self.check_keyword(1, 4, "hile", TokenType::While),
            _ => TokenType::Identifier,
        }
    }

    fn check_keyword(
        &self,
        start: usize,
        len: usize,
        suffix: &str,
        token_type: TokenType,
    ) -> TokenType {
        if self.current.get(start..start + len) == Some(suffix) {
            token_type
        } else {
            TokenType::Identifier
        }
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_digit(10)
    }
    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic()
    }

    fn string(&self) -> Token {
        loop {
            if let Some(c) = self.current.chars().next() {
                match c {
                    '"' => return self.make_token(TokenType::String),
                    '\0' => return self.error_token("Unterminated string."),
                    _ => continue,
                }
            }
        }
    }

    fn error_token(&self, message: &'a str) -> Token<'a> {
        Token {
            token_type: TokenType::Error,
            line: self.line,
            lexeme: message,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current == '\0'.to_string()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.current != expected.to_string() {
            return false;
        }
        self.advance();
        return true;
    }

    fn make_token(&self, token_type: TokenType) -> Token<'a> {
        Token {
            token_type,
            line: self.line,
            lexeme: &self.start[..self.current.len()],
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current.chars().next() {
            match c {
                ' ' | '\r' | '\t' => {
                    self.current = &self.current[1..];
                }
                '\n' => {
                    self.current = &self.current[1..];
                    self.line += 1;
                }
                _ => break,
            }
        }
    }

    fn advance(&mut self) -> Option<char> {
        let char = self.current.chars().next();
        if let Some(_) = char {
            self.current = &self.current[1..];
        }
        char
    }
}
