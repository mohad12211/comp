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
    InvalidExpression {
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
            ParseError::InvalidExpression { line } => {
                write!(f, "Invalid Expression at line: {line}.")
            }
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
        if let Some(token) = self.tokens.first() {
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
        let token = self.tokens.first();
        if token.is_some_and(|token| token.kind == expected) {
            self.tokens = &self.tokens[1..];
            Ok(token.copied().expect("if condition 'is some' is true"))
        } else {
            Err(ParseError::UnexpectedToken {
                expected: Some(expected),
                got: token.map(|token| token.kind),
                line: token.map_or(self.get_last_line(), |token| token.line),
            })
        }
    }

    fn consume(&mut self) -> Token<'de> {
        // TODO: add proper expect
        let token = self.tokens.first().unwrap();
        self.tokens = &self.tokens[1..];
        *token
    }

    fn statement(&mut self) -> Result<Stmt<'de>, ParseError> {
        self.expect(TokenKind::Return)?;
        let return_value = self.expression()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Return(return_value))
    }

    fn expression(&mut self) -> Result<Expr<'de>, ParseError> {
        match self.tokens.first().map(|token| token.kind) {
            Some(TokenKind::Constant) => Ok(Expr::Constant(self.int()?)),
            Some(TokenKind::Tilde) | Some(TokenKind::Hyphen) => {
                let operator = self.consume();
                let right = self.expression()?.into();
                Ok(Expr::UnaryOp { operator, right })
            }
            Some(TokenKind::LeftParen) => {
                self.expect(TokenKind::LeftParen)?;
                let inner_expr = self.expression()?;
                self.expect(TokenKind::RightParen)?;
                Ok(inner_expr)
            }
            _ => Err(ParseError::InvalidExpression {
                line: self.get_last_line(),
            }),
        }
    }

    fn int(&mut self) -> Result<i32, ParseError> {
        Ok(self
            .expect(TokenKind::Constant)?
            .lexeme
            .parse()
            .expect("Lexer should only parse valid integers"))
    }

    fn get_last_line(&self) -> usize {
        self.lexer.tokens.last().map_or(0, |token| token.line)
    }
}
