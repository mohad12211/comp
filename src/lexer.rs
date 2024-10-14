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
            '+' if self.try_consume("+") => self.add_token(TokenKind::DoublePlus),
            '+' if self.try_consume("=") => self.add_token(TokenKind::PlusEqual),
            '+' => self.add_token(TokenKind::Plus),
            '*' if self.try_consume("=") => self.add_token(TokenKind::AsteriskEqual),
            '*' => self.add_token(TokenKind::Asterisk),
            '/' if self.try_consume("=") => self.add_token(TokenKind::ForwardSlashEqual),
            '/' => self.add_token(TokenKind::ForwardSlash),
            '%' if self.try_consume("=") => self.add_token(TokenKind::PercentEqual),
            '%' => self.add_token(TokenKind::Percent),
            '^' if self.try_consume("=") => self.add_token(TokenKind::CaretEqual),
            '^' => self.add_token(TokenKind::Caret),
            '&' if self.try_consume("&") => self.add_token(TokenKind::DoubleAmpersand),
            '&' if self.try_consume("=") => self.add_token(TokenKind::AmpersandEqual),
            '&' => self.add_token(TokenKind::Ampersand),
            '|' if self.try_consume("|") => self.add_token(TokenKind::DoubleBar),
            '|' if self.try_consume("=") => self.add_token(TokenKind::BarEqual),
            '|' => self.add_token(TokenKind::Bar),
            '>' if self.try_consume(">=") => self.add_token(TokenKind::RightShiftEqual),
            '>' if self.try_consume(">") => self.add_token(TokenKind::RightShift),
            '>' if self.try_consume("=") => self.add_token(TokenKind::GreaterEqual),
            '>' => self.add_token(TokenKind::Greater),
            '<' if self.try_consume("<=") => self.add_token(TokenKind::LeftShiftEqual),
            '<' if self.try_consume("<") => self.add_token(TokenKind::LeftShift),
            '<' if self.try_consume("=") => self.add_token(TokenKind::LessEqual),
            '<' => self.add_token(TokenKind::Less),
            '-' if self.try_consume("-") => self.add_token(TokenKind::DoubleHyphen),
            '-' if self.try_consume("=") => self.add_token(TokenKind::HyphenEqual),
            '-' => self.add_token(TokenKind::Hyphen),
            '!' if self.try_consume("=") => self.add_token(TokenKind::BangEqual),
            '!' => self.add_token(TokenKind::Bang),
            '=' if self.try_consume("=") => self.add_token(TokenKind::DoubleEqual),
            '=' => self.add_token(TokenKind::Equal),
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

    fn try_consume(&mut self, expected: &str) -> bool {
        if self.rest[self.len..].starts_with(expected) {
            for _ in 0..expected.len() {
                self.consume();
            }
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
            if !c.is_alphanumeric() && c != '_' {
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
