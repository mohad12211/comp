use crate::{ast, irc};

#[derive(Default)]
pub struct IrcGenerator {
    counter: usize,
}

impl IrcGenerator {
    pub fn gen_program<'a>(program: ast::Program<'a>) -> (irc::Program<'a>, i32) {
        let mut irc_generator = IrcGenerator { counter: 1 };
        (
            match program {
                ast::Program::Function(function) => {
                    irc::Program::Function(irc_generator.gen_function(function))
                }
            },
            (irc_generator.counter - 1) as i32 * 4,
        )
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
                let irc_operator = self.gen_unary(operator);
                instructions.push(irc::Instruction::Unary {
                    operator: irc_operator,
                    src,
                    dst: dst_var,
                });
                return dst;
            }
            ast::Expr::Binary {
                operator,
                left,
                right,
            } => {
                let v1 = self.gen_expr(left, instructions);
                let v2 = self.gen_expr(right, instructions);
                let dst_var = self.gen_temp();
                let dst = irc::Value::Var(dst_var);
                let irc_operator = self.gen_binary(operator);
                instructions.push(irc::Instruction::Binary {
                    operator: irc_operator,
                    src1: v1,
                    src2: v2,
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
                let value = self.gen_expr(&expr, &mut instructions);
                instructions.push(irc::Instruction::Ret(value));
                return instructions;
            }
        }
    }

    fn gen_temp(&mut self) -> usize {
        let old = self.counter;
        self.counter += 1;
        return old;
    }

    fn gen_unary(&mut self, operator: &ast::UnaryOp) -> irc::UnaryOp {
        match operator {
            ast::UnaryOp::Complement => irc::UnaryOp::Complement,
            ast::UnaryOp::Negate => irc::UnaryOp::Negate,
        }
    }

    fn gen_binary(&mut self, operator: &ast::BinaryOp) -> irc::BinaryOp {
        match operator {
            ast::BinaryOp::Add => irc::BinaryOp::Add,
            ast::BinaryOp::Subtract => irc::BinaryOp::Subtract,
            ast::BinaryOp::Multiply => irc::BinaryOp::Multiply,
            ast::BinaryOp::Divide => irc::BinaryOp::Divide,
            ast::BinaryOp::Remainder => irc::BinaryOp::Remainder,
        }
    }
}
