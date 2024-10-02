#[derive(Debug)]
pub enum Program<'a> {
    Function(Function<'a>),
}

#[derive(Debug, Clone, Copy)]
pub enum Register {
    AX,
    DX,
    CX,
    R10,
    R11,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mult,
    And,
    Or,
    Xor,
    Shl,
    Shr,
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Imm(i32),
    Register(Register),
    Pseudo(usize),
    Stack(usize),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Mov {
        src: Operand,
        dst: Operand,
    },
    Return,
    Unary {
        operator: UnaryOp,
        operand: Operand,
    },
    Binary {
        operator: BinaryOp,
        operand1: Operand,
        operand2: Operand,
    },
    Cmp {
        operand1: Operand,
        operand2: Operand,
    },
    Jmp(String),
    JumpCC {
        cond_code: CondCode,
        target: String,
    },
    SetCC {
        cond_code: CondCode,
        operand: Operand,
    },
    Label(String),
    Idiv(Operand),
    Cdq,
    AllocateStack(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum CondCode {
    E,
    NE,
    G,
    GE,
    L,
    LE,
}

#[derive(Debug)]
pub struct Function<'a> {
    pub name: &'a str,
    pub instructons: Vec<Instruction>,
}
