use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&'a mut self) -> Token<'a> {
        self.skip_whitespace();
        self.start = self.current;

        match self.advance() {
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
            c if c.is_alphabetic() => return self.identifier(),
            c if c.is_digit(10) => return self.number(),
            _ => return self.error_token("Unexpected character."),
        }
    }

    fn number(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }
        if self.peek() == Some('.') && self.peek_next().unwrap().is_digit(10) {
            self.advance();
            while let Some(c) = self.peek() {
                if c.is_digit(10) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        return self.make_token(TokenType::Number);
    }

    fn peek_next(&self) -> Option<char> {
        self.source[self.current + 2..].chars().next()
    }

    fn peek(&self) -> Option<char> {
        self.source[self.current + 1..].chars().next()
    }

    fn identifier(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c.is_alphabetic() || c.is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }
        return self.make_token(self.identifier_type());
    }

    fn identifier_type(&self) -> TokenType {
        match self.current_char() {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'f' => {
                if self.source.get(self.current + 1..self.current + 2) == Some("a") {
                    self.check_keyword(2, 3, "lse", TokenType::False)
                } else if self.source.get(self.current + 1..self.current + 2) == Some("o") {
                    self.check_keyword(2, 1, "r", TokenType::For)
                } else if self.source.get(self.current + 1..self.current + 2) == Some("u") {
                    self.check_keyword(2, 1, "n", TokenType::Fun)
                } else {
                    TokenType::Identifier
                }
            }
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            't' => {
                if self.source.get(self.current + 1..self.current + 2) == Some("h") {
                    self.check_keyword(2, 2, "is", TokenType::This)
                } else if self.source.get(self.current + 1..self.current + 2) == Some("r") {
                    self.check_keyword(2, 2, "ue", TokenType::True)
                } else {
                    TokenType::Identifier
                }
            }
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
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
        if self
            .source
            .get(self.current + start..self.current + start + len)
            == Some(suffix)
        {
            token_type
        } else {
            TokenType::Identifier
        }
    }

    fn string(&self) -> Token {
        loop {
            match self.current_char() {
                '"' => return self.make_token(TokenType::String),
                '\0' => return self.error_token("Unterminated string."),
                _ => continue,
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
        self.current_char() == '\0'
    }

    fn current_char(&self) -> char {
        self.source[self.current..].chars().next().unwrap()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.current_char() != expected {
            return false;
        }
        self.advance();
        return true;
    }

    fn lexeme(&self) -> &'a str {
        &self.source[self.start..self.current]
    }

    fn make_token(&self, token_type: TokenType) -> Token<'a> {
        Token {
            token_type,
            line: self.line,
            lexeme: &self.lexeme(),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.source[self.current..].chars().next().unwrap();
            match c {
                ' ' | '\r' | '\t' => {
                    self.current += 1;
                }
                '\n' => {
                    self.current += 1;
                    self.line += 1;
                }
                _ => break,
                // TODO: add logic for comments
            }
        }
    }

    fn advance(&mut self) -> char {
        let char = self.current_char();
        self.current += char.len_utf8();
        char
    }
}
