use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,

    // Literals.
    Identifier,
    Constant,

    // Keywords.
    Int,
    Void,
    Return,
}

#[derive(Debug, Clone)]
pub struct Token<'de> {
    pub kind: TokenKind,
    pub lexeme: &'de str,
    pub line: usize,
}

impl<'de> Token<'de> {
    pub fn new(kind: TokenKind, lexeme: &'de str, line: usize) -> Self {
        Token { kind, lexeme, line }
    }
}

impl TokenKind {
    pub fn same_kind(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}