use std::fmt::Display;

use crate::{
    ast::{BinaryOp, BlockItem, Decleration, Expr, Function, Program, Stmt, UnaryOp},
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
    InvalidFactor {
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
            ParseError::InvalidFactor { line } => {
                write!(f, "Invalid Factor at line: {line}.")
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
        let name = self.expect(TokenKind::Identifier)?.lexeme;
        self.expect(TokenKind::LeftParen)?;
        self.expect(TokenKind::Void)?;
        self.expect(TokenKind::RightParen)?;
        self.expect(TokenKind::LeftBrace)?;
        let mut body = Vec::new();
        while self
            .tokens
            .first()
            .is_some_and(|token| !matches!(token.kind, TokenKind::RightBrace))
        {
            let block_item = self.block_item()?;
            body.push(block_item);
        }
        let _right_brace_token = self.consume();
        Ok(Function { name, body })
    }

    fn block_item(&mut self) -> Result<BlockItem, ParseError> {
        match self.tokens.first().map(|token| token.kind) {
            Some(TokenKind::Int) => Ok(BlockItem::Decleration(self.decleration()?)),
            _ => Ok(BlockItem::Statement(self.statement()?)),
        }
    }

    fn decleration(&mut self) -> Result<Decleration, ParseError> {
        self.expect(TokenKind::Int)?;
        let name = self.expect(TokenKind::Identifier)?.lexeme.to_string();
        let mut init = None;
        if self.try_consume(TokenKind::Equal).is_some() {
            init = Some(self.expression(0)?);
        }
        self.expect(TokenKind::Semicolon)?;
        Ok(Decleration::Decleration { name, init })
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
        let token = self
            .tokens
            .first()
            .expect("Should only be called when you know the next token");
        self.tokens = &self.tokens[1..];
        *token
    }

    fn try_consume(&mut self, expected: TokenKind) -> Option<Token<'de>> {
        if let Some(token) = self.tokens.first().filter(|token| token.kind == expected) {
            self.tokens = &self.tokens[1..];
            Some(*token)
        } else {
            None
        }
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.try_consume(TokenKind::Return).is_some() {
            let return_value = self.expression(0)?;
            self.expect(TokenKind::Semicolon)?;
            Ok(Stmt::Return(return_value))
        } else if self.try_consume(TokenKind::Semicolon).is_some() {
            Ok(Stmt::Null)
        } else {
            let expr = self.expression(0)?;
            self.expect(TokenKind::Semicolon)?;
            Ok(Stmt::Expression(expr))
        }
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        match self.tokens.first().map(|token| token.kind) {
            Some(TokenKind::Constant) => Ok(Expr::Constant(
                self.expect(TokenKind::Constant)?
                    .lexeme
                    .parse()
                    .expect("Lexer should only parse valid integers"),
            )),
            Some(TokenKind::Tilde) => self.unary(UnaryOp::Complement),
            Some(TokenKind::Hyphen) => self.unary(UnaryOp::Negate),
            Some(TokenKind::Bang) => self.unary(UnaryOp::Not),
            Some(TokenKind::LeftParen) => {
                let _paren_token = self.consume();
                let inner = self.expression(0)?;
                self.expect(TokenKind::RightParen)?;
                Ok(inner)
            }
            Some(TokenKind::Identifier) => {
                let name = self.consume().lexeme.to_string();
                Ok(Expr::Var(name))
            }
            _ => Err(ParseError::InvalidFactor {
                line: self
                    .tokens
                    .first()
                    .map_or(self.get_last_line(), |token| token.line),
            }),
        }
    }

    fn unary(&mut self, operator: UnaryOp) -> Result<Expr, ParseError> {
        let _operator_token = self.consume();
        let right = self.factor()?.into();
        Ok(Expr::Unary { operator, right })
    }

    fn operator_and_precedence(token: &Token) -> Option<(Option<BinaryOp>, usize)> {
        match token.kind {
            TokenKind::DoubleBar => Some((Some(BinaryOp::Or), 5)),
            TokenKind::DoubleAmpersand => Some((Some(BinaryOp::And), 10)),
            TokenKind::Bar => Some((Some(BinaryOp::BitOr), 15)),
            TokenKind::Caret => Some((Some(BinaryOp::Xor), 20)),
            TokenKind::Ampersand => Some((Some(BinaryOp::BitAnd), 25)),
            TokenKind::DoubleEqual => Some((Some(BinaryOp::Equal), 30)),
            TokenKind::BangEqual => Some((Some(BinaryOp::NotEqual), 30)),
            TokenKind::Greater => Some((Some(BinaryOp::GreaterThan), 35)),
            TokenKind::GreaterEqual => Some((Some(BinaryOp::GreaterOrEqual), 35)),
            TokenKind::Less => Some((Some(BinaryOp::LessThan), 35)),
            TokenKind::LessEqual => Some((Some(BinaryOp::LessOrEqual), 35)),
            TokenKind::LeftShift => Some((Some(BinaryOp::LeftShift), 40)),
            TokenKind::RightShift => Some((Some(BinaryOp::RightShift), 40)),
            TokenKind::Hyphen => Some((Some(BinaryOp::Subtract), 45)),
            TokenKind::Plus => Some((Some(BinaryOp::Add), 45)),
            TokenKind::Asterisk => Some((Some(BinaryOp::Multiply), 50)),
            TokenKind::ForwardSlash => Some((Some(BinaryOp::Divide), 50)),
            TokenKind::Percent => Some((Some(BinaryOp::Remainder), 50)),
            TokenKind::Equal => Some((None, 1)),
            _ => None,
        }
    }

    fn expression(&mut self, min_prec: usize) -> Result<Expr, ParseError> {
        let mut left = self.factor()?;
        while let Some((operator, prec)) = self
            .tokens
            .first()
            .and_then(|token| Self::operator_and_precedence(token))
            .filter(|&(_, prec)| prec >= min_prec)
        {
            if let Some(operator) = operator {
                let _operator_token = self.consume();
                let right = self.expression(prec + 1)?;
                left = Expr::Binary {
                    operator,
                    left: left.into(),
                    right: right.into(),
                };
            } else {
                let _equal_token = self.consume();
                let right = self.expression(prec)?;
                left = Expr::Assignment {
                    left: left.into(),
                    right: right.into(),
                }
            }
        }
        Ok(left)
    }

    fn get_last_line(&self) -> usize {
        self.lexer.tokens.last().map_or(0, |token| token.line)
    }
}
