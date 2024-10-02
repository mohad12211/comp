use crate::token::Token;

#[derive(Debug)]
pub enum Program<'a> {
    Function(Function<'a>),
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Constant(i32),
    Var(usize),
}

#[derive(Debug)]
pub enum UnaryOp {
    Complement,
    Negate,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    LeftShift,
    RightShift,
    And,
    Xor,
    Or,
}

#[derive(Debug)]
pub enum Instruction {
    Ret(Value),
    Unary {
        operator: UnaryOp,
        src: Value,
        dst: usize,
    },
    Binary {
        operator: BinaryOp,
        src1: Value,
        src2: Value,
        dst: usize,
    },
}

#[derive(Debug)]
pub struct Function<'a> {
    pub name: Token<'a>,
    pub instructons: Vec<Instruction>,
}
