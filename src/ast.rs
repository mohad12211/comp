use crate::token::Token;

#[derive(Debug)]
pub enum Program<'a> {
    Function(Function<'a>),
}

#[derive(Debug)]
pub enum Expr<'a> {
    Constant(i32),
    Unary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
}

#[derive(Debug)]
pub enum Stmt<'a> {
    Return(Expr<'a>),
}

#[derive(Debug)]
pub struct Function<'a> {
    pub name: Token<'a>,
    pub body: Stmt<'a>,
}
