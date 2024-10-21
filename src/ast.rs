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
    Conditional {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum AssignmentOp {
    Equal,
    PlusEqual,
    SubtractEqual,
    MultipleEqual,
    DivideEqual,
    RemainderEqual,
    BitAndEqual,
    BitOrEqual,
    XorEqual,
    RightShiftEqual,
    LeftShiftEqual,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Complement,
    Negate,
    Not,
    PrefixInc,
    PrefixDec,
    PostFixInc,
    PostFixDec,
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
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Null,
    Goto(String),
    Label(String, Box<Stmt>),
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
