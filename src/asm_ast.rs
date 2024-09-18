use crate::token::Token;

#[derive(Debug)]
pub enum Program<'a> {
    Function(Function<'a>),
}

#[derive(Debug)]
pub enum Operand {
    Imm(i32),
    Register,
}

#[derive(Debug)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}

#[derive(Debug)]
pub struct Function<'a> {
    pub name: Token<'a>,
    pub instructons: Vec<Instruction>,
}
