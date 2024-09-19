use crate::{
    token::{Token, TokenKind},
    Error,
};

pub struct Lexer<'de> {
    pub tokens: Vec<Token<'de>>,
    rest: &'de str,
    line: usize,
}

impl<'de> Lexer<'de> {
    pub fn new(source: &'de str) -> Self {
        Self {
            tokens: Vec::new(),
            rest: source,
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
        let char = self
            .rest
            .chars()
            .next()
            .expect("rest shouldn't be empty from self.tokenize condition");
        match char {
            char if char.is_alphabetic() => {
                let end = self
                    .rest
                    .find(|c: char| !c.is_alphabetic())
                    .ok_or(Error::Lexer(format!(
                        "Unfinsiehd identifier at line {}",
                        self.line
                    )))?;
                let lexeme = &self.rest[..end];
                self.add_token(
                    Self::get_keyword(lexeme).unwrap_or(TokenKind::Identifier),
                    end,
                );
            }
            char if char.is_ascii_digit() => {
                let end = self
                    .rest
                    .find(|c: char| !c.is_ascii_digit())
                    .ok_or(Error::Lexer(format!(
                        "Invalid number at line {}",
                        self.line
                    )))?;
                self.add_token(TokenKind::Constant, end);
                if self.rest.starts_with(|c: char| c.is_alphabetic()) {
                    return Err(Error::Lexer(format!(
                        "Invalid identifer at line {}",
                        self.line
                    )));
                };
            }
            '(' => self.add_token(TokenKind::LeftParen, char.len_utf8()),
            ')' => self.add_token(TokenKind::RightParen, char.len_utf8()),
            '{' => self.add_token(TokenKind::LeftBrace, char.len_utf8()),
            '}' => self.add_token(TokenKind::RightBrace, char.len_utf8()),
            ';' => self.add_token(TokenKind::Semicolon, char.len_utf8()),
            '~' => self.add_token(TokenKind::Tilde, char.len_utf8()),
            '-' => {
                if self.rest.chars().nth(1).is_some_and(|c| c == '-') {
                    self.add_token(TokenKind::DoubleHyphen, char.len_utf8() * 2);
                } else {
                    self.add_token(TokenKind::Hyphen, char.len_utf8());
                }
            }
            ' ' | '\t' => self.rest = &self.rest[char.len_utf8()..],
            '\n' => {
                self.line += 1;
                self.rest = &self.rest[char.len_utf8()..];
            }
            _ => {
                return Err(Error::InvalidToken(format!(
                    "Unexpected Character `{char}` at line {}",
                    self.line
                )))
            }
        };
        Ok(())
    }

    pub fn add_token(&mut self, kind: TokenKind, len: usize) {
        self.tokens.push(Token {
            kind,
            lexeme: &self.rest[..len],
            line: self.line,
        });
        self.rest = &self.rest[len..];
    }

    fn get_keyword(str: &str) -> Option<TokenKind> {
        match str {
            "int" => Some(TokenKind::Int),
            "void" => Some(TokenKind::Void),
            "return" => Some(TokenKind::Return),
            _ => None,
        }
    }
}
