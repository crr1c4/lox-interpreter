use std::fmt::Debug;

#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenType,
    source: &'a str,
    line: usize,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenType, source: &'a str, line: usize) -> Self {
        Self { source, line, kind }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn token_type(&self) -> &TokenType {
        &self.kind
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    ERROR,
    EOF,
}
