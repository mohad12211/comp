use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Tilde,
    Hyphen,
    DoubleHyphen,
    Plus,
    Asterisk,
    ForwardSlash,
    Percent,
    And,
    Or,
    Xor,
    LeftShift,
    RightShift,

    // Literals.
    Identifier,
    Constant,

    // Keywords.
    Int,
    Void,
    Return,
}

impl TokenKind {
    pub fn precedence(&self) -> Option<usize> {
        match self {
            TokenKind::Hyphen => Some(45),
            TokenKind::Plus => Some(45),
            TokenKind::Asterisk => Some(50),
            TokenKind::ForwardSlash => Some(50),
            TokenKind::Percent => Some(50),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
