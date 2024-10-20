use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Tilde,
    Hyphen,
    HyphenEqual,
    DoubleHyphen,
    Plus,
    DoublePlus,
    PlusEqual,
    Asterisk,
    AsteriskEqual,
    ForwardSlash,
    ForwardSlashEqual,
    Percent,
    PercentEqual,
    Ampersand,
    AmpersandEqual,
    Bar,
    BarEqual,
    Caret,
    CaretEqual,
    LeftShift,
    LeftShiftEqual,
    RightShift,
    RightShiftEqual,
    Bang,
    Equal,
    DoubleAmpersand,
    DoubleBar,
    DoubleEqual,
    BangEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Question,
    Colon,

    // Literals.
    Identifier,
    Constant,

    // Keywords.
    Int,
    Void,
    Return,
    If,
    Else,
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
