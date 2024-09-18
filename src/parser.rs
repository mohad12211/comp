use std::fmt::Display;

use crate::{
    ast::{Expr, Function, Program, Stmt},
    lexer::Lexer,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: Option<TokenKind>,
        got: Option<TokenKind>,
        line: usize,
    },
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: not good error reporting
        match self {
            ParseError::UnexpectedToken {
                expected,
                got,
                line,
            } => write!(
                f,
                "Unexpected token at line: {line}, got: {got:?}, expected: {expected:?}"
            ),
        }
    }
}

pub struct Parser<'de> {
    lexer: &'de Lexer<'de>,
    tokens: &'de [Token<'de>],
}

impl<'de> Parser<'de> {
    pub fn new(lexer: &'de Lexer) -> Self {
        Self {
            lexer,
            tokens: &lexer.tokens,
        }
    }
    pub fn parse(&mut self) -> Result<Program<'de>, ParseError> {
        let program = self.program()?;
        if let Some(token) = self.tokens.get(0) {
            Err(ParseError::UnexpectedToken {
                expected: None,
                got: Some(token.kind),
                line: token.line,
            })
        } else {
            Ok(program)
        }
    }

    fn program(&mut self) -> Result<Program<'de>, ParseError> {
        Ok(Program::Function(self.function()?))
    }

    fn function(&mut self) -> Result<Function<'de>, ParseError> {
        self.expect(TokenKind::Int)?;
        let name = self.expect(TokenKind::Identifier)?;
        self.expect(TokenKind::LeftParen)?;
        self.expect(TokenKind::Void)?;
        self.expect(TokenKind::RightParen)?;
        self.expect(TokenKind::LeftBrace)?;
        let body = self.statement()?;
        self.expect(TokenKind::RightBrace)?;
        Ok(Function { name, body })
    }

    fn expect(&mut self, expected: TokenKind) -> Result<Token<'de>, ParseError> {
        let token = self.tokens.get(0);
        if token.is_some_and(|token| token.kind == expected) {
            self.tokens = &self.tokens[1..];
            Ok(token.copied().expect("if condition 'is some' is true"))
        } else {
            Err(ParseError::UnexpectedToken {
                expected: Some(expected),
                got: token.map(|token| token.kind),
                line: token.map(|token| token.line).unwrap_or(
                    self.lexer
                        .tokens
                        .last()
                        .map(|token| token.line)
                        .unwrap_or(0),
                ),
            })
        }
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::Return)?;
        let return_value = self.expression()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Return(return_value))
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        Ok(Expr::Constant(self.int()?))
    }

    fn int(&mut self) -> Result<i32, ParseError> {
        let token = self.expect(TokenKind::Constant)?;
        let value = token
            .lexeme
            .parse()
            .expect("Lexer should only parse valid integers");
        Ok(value)
    }
}
