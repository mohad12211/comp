use crate::token::Token;

#[derive(Debug)]
pub enum Program<'a> {
    Function(Function<'a>),
}

#[derive(Debug)]
pub enum Expr {
    Constant(i32),
}

#[derive(Debug)]
pub enum Stmt {
    Return(Expr),
}

#[derive(Debug)]
pub struct Function<'a> {
    pub name: Token<'a>,
    pub body: Stmt,
}
