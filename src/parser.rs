use std::fmt::Display;

use crate::{
    ast::{AssignmentOp, BinaryOp, BlockItem, Decleration, Expr, Function, Program, Stmt, UnaryOp},
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
            Self::UnexpectedToken {
                expected,
                got,
                line,
            } => write!(
                f,
                "Unexpected token at line: {line}, got: {got:?}, expected: {expected:?}"
            ),
            Self::InvalidExpression { line } => {
                write!(f, "Invalid Expression at line: {line}.")
            }
            Self::InvalidFactor { line } => {
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
        self.tokens.first().map_or_else(
            || Ok(program),
            |token| {
                Err(ParseError::UnexpectedToken {
                    expected: None,
                    got: Some(token.kind),
                    line: token.line,
                })
            },
        )
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
        let init = if self.try_consume(TokenKind::Equal).is_some() {
            Some(self.expression(0)?)
        } else {
            None
        };
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
        let right = match self.tokens.first().map(|token| token.kind) {
            Some(TokenKind::Constant) => Ok(Expr::Constant(
                self.expect(TokenKind::Constant)?
                    .lexeme
                    .parse()
                    .expect("Lexer should only parse valid integers"),
            )),
            Some(TokenKind::Tilde) => self.unary(UnaryOp::Complement),
            Some(TokenKind::Hyphen) => self.unary(UnaryOp::Negate),
            Some(TokenKind::Bang) => self.unary(UnaryOp::Not),
            Some(TokenKind::DoublePlus) => self.unary(UnaryOp::PrefixInc),
            Some(TokenKind::DoubleHyphen) => self.unary(UnaryOp::PrefixDec),
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
        };
        // TODO: I don't like this
        right.map(|right| {
            if self.try_consume(TokenKind::DoublePlus).is_some() {
                Expr::Unary {
                    operator: UnaryOp::PostFixInc,
                    right: right.into(),
                }
            } else if self.try_consume(TokenKind::DoubleHyphen).is_some() {
                Expr::Unary {
                    operator: UnaryOp::PostFixDec,
                    right: right.into(),
                }
            } else {
                right
            }
        })
    }

    fn unary(&mut self, operator: UnaryOp) -> Result<Expr, ParseError> {
        let _operator_token = self.consume();
        let right = self.factor()?.into();
        Ok(Expr::Unary { operator, right })
    }

    fn precedence(token: &Token) -> Option<usize> {
        match token.kind {
            TokenKind::Equal => Some(1),
            TokenKind::PlusEqual => Some(1),
            TokenKind::HyphenEqual => Some(1),
            TokenKind::AsteriskEqual => Some(1),
            TokenKind::ForwardSlashEqual => Some(1),
            TokenKind::PercentEqual => Some(1),
            TokenKind::AmpersandEqual => Some(1),
            TokenKind::BarEqual => Some(1),
            TokenKind::CaretEqual => Some(1),
            TokenKind::LeftShiftEqual => Some(1),
            TokenKind::RightShiftEqual => Some(1),
            TokenKind::DoubleBar => Some(5),
            TokenKind::DoubleAmpersand => Some(9),
            TokenKind::Bar => Some(15),
            TokenKind::Caret => Some(20),
            TokenKind::Ampersand => Some(25),
            TokenKind::DoubleEqual => Some(30),
            TokenKind::BangEqual => Some(30),
            TokenKind::Greater => Some(35),
            TokenKind::GreaterEqual => Some(35),
            TokenKind::Less => Some(35),
            TokenKind::LessEqual => Some(35),
            TokenKind::LeftShift => Some(40),
            TokenKind::RightShift => Some(40),
            TokenKind::Hyphen => Some(45),
            TokenKind::Plus => Some(45),
            TokenKind::Asterisk => Some(50),
            TokenKind::ForwardSlash => Some(50),
            TokenKind::Percent => Some(50),
            _ => None,
        }
    }

    fn expression(&mut self, min_prec: usize) -> Result<Expr, ParseError> {
        let mut left = self.factor()?;
        while let Some(prec) = self
            .tokens
            .first()
            .and_then(|token| Self::precedence(token))
            .filter(|&prec| prec >= min_prec)
        {
            // TODO: maybe use match
            let token = self.consume();
            if let Some(assignment_op) = Self::assignment_op(token.kind) {
                let right = self.expression(prec)?;
                left = Expr::Assignment {
                    left: left.into(),
                    right: right.into(),
                    operator: assignment_op,
                }
            } else if let Some(binary_op) = Self::binary_op(token.kind) {
                let right = self.expression(prec + 1)?;
                left = Expr::Binary {
                    operator: binary_op,
                    left: left.into(),
                    right: right.into(),
                };
            }
        }
        Ok(left)
    }

    fn assignment_op(token: TokenKind) -> Option<AssignmentOp> {
        match token {
            TokenKind::Equal => Some(AssignmentOp::Equal),
            TokenKind::PlusEqual => Some(AssignmentOp::PlusEqual),
            TokenKind::HyphenEqual => Some(AssignmentOp::SubtractEqual),
            TokenKind::AsteriskEqual => Some(AssignmentOp::MultipleEqual),
            TokenKind::ForwardSlashEqual => Some(AssignmentOp::DivideEqual),
            TokenKind::PercentEqual => Some(AssignmentOp::RemainderEqual),
            TokenKind::AmpersandEqual => Some(AssignmentOp::BitAndEqual),
            TokenKind::BarEqual => Some(AssignmentOp::BitOrEqual),
            TokenKind::CaretEqual => Some(AssignmentOp::XorEqual),
            TokenKind::RightShiftEqual => Some(AssignmentOp::RightShiftEqual),
            TokenKind::LeftShiftEqual => Some(AssignmentOp::LeftShiftEqual),
            _ => None,
        }
    }

    fn binary_op(token: TokenKind) -> Option<BinaryOp> {
        match token {
            TokenKind::DoubleBar => Some(BinaryOp::Or),
            TokenKind::DoubleAmpersand => Some(BinaryOp::And),
            TokenKind::Bar => Some(BinaryOp::BitOr),
            TokenKind::Caret => Some(BinaryOp::Xor),
            TokenKind::Ampersand => Some(BinaryOp::BitAnd),
            TokenKind::DoubleEqual => Some(BinaryOp::Equal),
            TokenKind::BangEqual => Some(BinaryOp::NotEqual),
            TokenKind::Greater => Some(BinaryOp::GreaterThan),
            TokenKind::GreaterEqual => Some(BinaryOp::GreaterOrEqual),
            TokenKind::Less => Some(BinaryOp::LessThan),
            TokenKind::LessEqual => Some(BinaryOp::LessOrEqual),
            TokenKind::LeftShift => Some(BinaryOp::LeftShift),
            TokenKind::RightShift => Some(BinaryOp::RightShift),
            TokenKind::Hyphen => Some(BinaryOp::Subtract),
            TokenKind::Plus => Some(BinaryOp::Add),
            TokenKind::Asterisk => Some(BinaryOp::Multiply),
            TokenKind::ForwardSlash => Some(BinaryOp::Divide),
            TokenKind::Percent => Some(BinaryOp::Remainder),
            _ => None,
        }
    }

    fn get_last_line(&self) -> usize {
        self.lexer.tokens.last().map_or(0, |token| token.line)
    }
}
