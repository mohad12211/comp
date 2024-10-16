#[derive(Debug)]
pub enum Program<'a> {
    Function(Function<'a>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Constant(i32),
    Unary {
        operator: UnaryOp,
        right: Box<Expr>,
    },
    Binary {
        operator: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Var(String),
    Assignment {
        operator: AssignmentOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum AssignmentOp {
    Equal,
    PlusEqual,
}

#[derive(Debug, Clone, Copy)]
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
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
}

#[derive(Debug)]
pub enum Stmt {
    Return(Expr),
    Expression(Expr),
    Null,
}

#[derive(Debug)]
pub enum Decleration {
    Decleration { name: String, init: Option<Expr> },
}

#[derive(Debug)]
pub enum BlockItem {
    Statement(Stmt),
    Decleration(Decleration),
}

#[derive(Debug)]
pub struct Function<'a> {
    pub name: &'a str,
    pub body: Vec<BlockItem>,
}
