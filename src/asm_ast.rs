use crate::token::Token;

#[derive(Debug)]
pub enum Program<'a> {
    Function(Function<'a>),
}

#[derive(Debug, Clone, Copy)]
pub enum Register {
    AX,
    R10,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Imm(i32),
    Register(Register),
    Pseudo(usize),
    Stack(i32),
}

#[derive(Debug)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Return,
    Unary { operator: UnaryOp, operand: Operand },
    AllocateStack(i32),
}

#[derive(Debug)]
pub struct Function<'a> {
    pub name: Token<'a>,
    pub instructons: Vec<Instruction>,
}
