use crate::{
    token::{Token, TokenKind},
    Error,
};

pub struct Lexer<'de> {
    pub tokens: Vec<Token<'de>>,
    rest: &'de str,
    len: usize,
    line: usize,
}

impl<'de> Lexer<'de> {
    pub fn new(source: &'de str) -> Self {
        Self {
            tokens: Vec::new(),
            rest: source,
            len: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<(), Error> {
        while !self.rest.is_empty() {
            self.consume_token()?;
        }
        Ok(())
    }

    pub fn consume_token(&mut self) -> Result<(), Error> {
        let c = self.consume();
        match c {
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),
            ';' => self.add_token(TokenKind::Semicolon),
            '~' => self.add_token(TokenKind::Tilde),
            '+' => self.add_token(TokenKind::Plus),
            '*' => self.add_token(TokenKind::Asterisk),
            '/' => self.add_token(TokenKind::ForwardSlash),
            '%' => self.add_token(TokenKind::Percent),
            '&' => self.add_token(TokenKind::Ampersand),
            '|' => self.add_token(TokenKind::Bar),
            '^' => self.add_token(TokenKind::Caret),
            '>' if self.try_consume('>') => {
                self.add_token(TokenKind::RightShift);
            }
            '<' if self.try_consume('<') => {
                self.add_token(TokenKind::LeftShift);
            }
            '-' => {
                if self.try_consume('-') {
                    self.add_token(TokenKind::DoubleHyphen);
                } else {
                    self.add_token(TokenKind::Hyphen);
                }
            }
            ' ' | '\t' => {}
            '\n' => self.line += 1,
            c if c.is_ascii_digit() => self.number()?,
            c if c.is_alphabetic() => self.identifier(),
            _ => {
                return Err(Error::InvalidToken(format!(
                    "Unexpected character '{}' at line {}",
                    c, self.line
                )))
            }
        }
        self.rest = &self.rest[self.len..];
        self.len = 0;
        Ok(())
    }

    fn consume(&mut self) -> char {
        let c = self.rest[self.len..].chars().next().unwrap();
        let len_utf8 = c.len_utf8();
        self.len += len_utf8;
        c
    }

    fn try_consume(&mut self, expected: char) -> bool {
        if self.rest[self.len..].starts_with(expected) {
            self.consume();
            true
        } else {
            false
        }
    }

    fn add_token(&mut self, kind: TokenKind) {
        let lexeme = &self.rest[..self.len];
        self.tokens.push(Token {
            kind,
            lexeme,
            line: self.line,
        });
    }

    fn number(&mut self) -> Result<(), Error> {
        while let Some(c) = self.rest[self.len..].chars().next() {
            if !c.is_ascii_digit() {
                break;
            }
            self.consume();
        }
        if self.rest[self.len..].starts_with(|c: char| c.is_alphabetic()) {
            return Err(Error::Lexer(format!(
                "Invalid identifier at line {}",
                self.line
            )));
        }
        self.add_token(TokenKind::Constant);
        Ok(())
    }

    fn identifier(&mut self) {
        while let Some(c) = self.rest[self.len..].chars().next() {
            if !c.is_alphanumeric() {
                break;
            }
            self.consume();
        }
        let lexeme = &self.rest[..self.len];
        let kind = Self::get_keyword(lexeme).unwrap_or(TokenKind::Identifier);
        self.add_token(kind);
    }

    fn get_keyword(lexeme: &str) -> Option<TokenKind> {
        match lexeme {
            "int" => Some(TokenKind::Int),
            "void" => Some(TokenKind::Void),
            "return" => Some(TokenKind::Return),
            _ => None,
        }
    }
}
