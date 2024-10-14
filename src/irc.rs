#[derive(Debug)]
pub enum Program<'a> {
    Function(Function<'a>),
}

#[derive(Debug, Clone)]
pub enum Value {
    Constant(i32),
    Var(String),
}

#[derive(Debug)]
pub enum UnaryOp {
    Complement,
    Negate,
    Not,
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
    BitAnd,
    Xor,
    BitOr,
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
}

#[derive(Debug)]
pub enum Instruction {
    Ret(Value),
    Unary {
        operator: UnaryOp,
        src: Value,
        dst: String,
    },
    Binary {
        operator: BinaryOp,
        src1: Value,
        src2: Value,
        dst: String,
    },
    Copy {
        src: Value,
        dst: String,
    },
    Jump {
        target: String,
    },
    JumpIfZero {
        condition: Value,
        target: String,
    },
    JumpIfNotZero {
        condition: Value,
        target: String,
    },
    Label(String),
}

#[derive(Debug)]
pub struct Function<'a> {
    pub name: &'a str,
    pub instructons: Vec<Instruction>,
}
