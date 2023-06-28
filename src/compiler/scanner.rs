use super::token::Token;
use super::token::TokenType;

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            current: 0,
            start: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

       

        let c = self.advance();

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        if self.is_alpha(c) {
            return self.identifier();
        }

        if self.is_digit(c) {
            return self.number();
        }

        match c {
            '\n' => {
                self.line += 1;
                self.advance();
            }
            '/' => {
                if self.peek_next() == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
            }
            _ => (),
        }

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => self.make_token(if self.next('=') {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            }),
            '=' => self.make_token(if self.next('=') {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            }),
            '<' => self.make_token(if self.next('=') {
                TokenType::LessEqual
            } else {
                TokenType::Less
            }),
            '>' => self.make_token(if self.next('=') {
                TokenType::GreaterEqual
            } else {
                TokenType::Greater
            }),
            '"' => self.string(),
            _ => self.error_token(&"Unexpected character"),
        }
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => self.advance(),
                _ => break,
            };
        }
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    fn next(&self, expected: char) -> bool {
        !self.is_at_end() && self.source.chars().nth(self.current).unwrap() != expected
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.source.chars().nth(self.current).is_none()
    }

    fn make_token(&self, kind: TokenType) -> Token {
        Token::new(kind, &self.source[self.start..self.current], self.line)
    }

    fn error_token<'b>(&self, message: &'b str) -> Token {
        Token::new(TokenType::ERROR, "Eror", self.line)
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return self.error_token(&"Underterminated string");
        } else {
            self.advance();
            return self.make_token(TokenType::String);
        }
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(&self, c: char) -> bool {
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
    }

    fn identifier(&mut self) -> Token {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        match self.source.chars().nth(self.start).unwrap() {
            'a' => return self.check_keyword(1, 2, "an", TokenType::And),
            'c' => return self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => return self.check_keyword(1, 3, "lse", TokenType::Else),
            'i' => return self.check_keyword(1, 1, "f", TokenType::If),
            'n' => return self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => return self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => return self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => return self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => return self.check_keyword(1, 4, "uper", TokenType::Super),
            'v' => return self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => return self.check_keyword(1, 4, "hile", TokenType::While),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1).unwrap() {
                        'a' => return self.check_keyword(2, 2, "an", TokenType::False),
                        'o' => return self.check_keyword(2, 1, "an", TokenType::For),
                        'u' => return self.check_keyword(2, 1, "an", TokenType::Fun),
                        _ => TokenType::Identifier
                    }
                } else {
                    TokenType::Identifier
                }
            }
            't' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1).unwrap() {
                        'h' => return self.check_keyword(2, 2, "is", TokenType::This),
                        'r' => return self.check_keyword(2, 2, "ue", TokenType::True),
                        _ => TokenType::Identifier
                    }
                } else {
                    TokenType::Identifier
                }
            },
            _ => TokenType::Identifier
        }
        
    }

    fn check_keyword(&self, start: usize, length: usize, rest: &str, kind: TokenType) -> TokenType {
        if self.current - self.start == start + length {
            return kind;
        }

        TokenType::Identifier

        //             use std::cmp::Ordering;

        // let scanner_start = &scanner.start[start..]; // Slice desde 'start' hasta el final
        // let rest = &rest[..length]; // Slice de 'rest' con longitud 'length'

        // let result = scanner_start.cmp(rest);

        // if result == Ordering::Equal {
        //     // Hacer algo si los slices son iguales
        // } else {
        //     // Hacer algo si los slices son diferentes
        // }
    }

    fn number(&mut self) -> Token {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        self.make_token(TokenType::Number)
    }
}

/*
use std::fmt::Display;

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

use std::any::Any;
use std::fmt::Display;
use super::token_type::TokenType;

#[allow(unused)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Box<dyn Any>>,
    line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: impl Into<String>, literal: Option<Box<dyn Any>>, line: usize) -> Token {
        Token {
            token_type,
            lexeme: lexeme.into(),
            literal,
            line,
        }
    }
}


 */
