use crate::{ast, irc, token::TokenKind};

#[derive(Default)]
pub struct IrcGenerator {
    counter: usize,
}

impl IrcGenerator {
    pub fn gen_program<'a>(program: ast::Program<'a>) -> irc::Program<'a> {
        let mut irc_generator = IrcGenerator::default();
        match program {
            ast::Program::Function(function) => {
                irc::Program::Function(irc_generator.gen_function(function))
            }
        }
    }

    fn gen_function<'a>(&mut self, function: ast::Function<'a>) -> irc::Function<'a> {
        irc::Function {
            name: function.name,
            instructons: self.gen_stmt(function.body),
        }
    }

    fn gen_expr(
        &mut self,
        expr: &ast::Expr,
        instructions: &mut Vec<irc::Instruction>,
    ) -> irc::Value {
        match expr {
            ast::Expr::Constant(value) => irc::Value::Constant(*value),
            ast::Expr::Unary { operator, right } => {
                let src = self.gen_expr(right, instructions);
                let dst_var = self.gen_temp();
                let dst = irc::Value::Var(dst_var);
                let irc_operator = self.gen_unary(operator.kind);
                instructions.push(irc::Instruction::Unary {
                    operator: irc_operator,
                    src,
                    dst: dst_var,
                });
                return dst;
            }
        }
    }

    fn gen_stmt(&mut self, stmt: ast::Stmt) -> Vec<irc::Instruction> {
        match stmt {
            ast::Stmt::Return(expr) => {
                let mut instructions = Vec::new();
                self.gen_expr(&expr, &mut instructions);
                instructions.push(irc::Instruction::Ret(irc::Value::Var(self.counter)));
                return instructions;
            }
        }
    }

    fn gen_temp(&mut self) -> usize {
        let old = self.counter;
        self.counter += 1;
        return old;
    }

    fn gen_unary(&mut self, token_kind: TokenKind) -> irc::UnaryOp {
        match token_kind {
            TokenKind::Tilde => irc::UnaryOp::Complement,
            TokenKind::Hyphen => irc::UnaryOp::Negate,
            _ => unreachable!(),
        }
    }
}
