use std::iter;

use crate::{ast, irc};

#[derive(Default)]
pub struct IrcGenerator {
    counter: usize,
}

impl IrcGenerator {
    pub fn gen_program(program: ast::Program<'_>, counter: usize) -> (irc::Program<'_>, usize) {
        let mut irc_generator = Self { counter };
        (
            match program {
                ast::Program::Function(function) => {
                    irc::Program::Function(irc_generator.gen_function(function))
                }
            },
            (irc_generator.counter - 1) * 4,
        )
    }

    fn gen_function<'a>(&mut self, function: ast::Function<'a>) -> irc::Function<'a> {
        irc::Function {
            name: function.name,
            instructons: function
                .body
                .into_iter()
                .flat_map(|block_item| self.gen_block_item(block_item))
                .chain(iter::once(irc::Instruction::Ret(irc::Value::Constant(0))))
                .collect(),
        }
    }

    fn gen_block_item(&mut self, block_item: ast::BlockItem) -> Vec<irc::Instruction> {
        match block_item {
            ast::BlockItem::Statement(stmt) => self.gen_stmt(stmt),
            ast::BlockItem::Decleration(ast::Decleration::Decleration { name, init }) => {
                let Some(init) = init else {
                    return Vec::new();
                };
                let mut instructions = Vec::new();
                let value = self.gen_expr(init, &mut instructions);
                instructions.push(irc::Instruction::Copy {
                    src: value,
                    dst: name,
                });
                instructions
            }
        }
    }

    fn gen_expr(
        &mut self,
        expr: ast::Expr,
        instructions: &mut Vec<irc::Instruction>,
    ) -> irc::Value {
        match expr {
            ast::Expr::Constant(value) => irc::Value::Constant(value),
            ast::Expr::Unary { operator, right } => {
                let src = self.gen_expr(*right, instructions);
                let dst_var = self.gen_temp();
                let dst = irc::Value::Var(dst_var.clone());
                let irc_operator = Self::gen_unary(operator);
                instructions.push(irc::Instruction::Unary {
                    operator: irc_operator,
                    src,
                    dst: dst_var,
                });
                dst
            }
            ast::Expr::Binary {
                operator: ast::BinaryOp::And,
                left,
                right,
            } => {
                let result_var = self.gen_temp();
                let result = irc::Value::Var(result_var.clone());
                let v1 = self.gen_expr(*left, instructions);
                let false_label = self.gen_label("and_false");
                let end_label = self.gen_label("and_end");

                instructions.push(irc::Instruction::JumpIfZero {
                    condition: v1,
                    target: false_label.clone(),
                });
                let v2 = self.gen_expr(*right, instructions);
                instructions.push(irc::Instruction::JumpIfZero {
                    condition: v2,
                    target: false_label.clone(),
                });
                instructions.push(irc::Instruction::Copy {
                    src: irc::Value::Constant(1),
                    dst: result_var.clone(),
                });
                instructions.push(irc::Instruction::Jump {
                    target: end_label.clone(),
                });
                instructions.push(irc::Instruction::Label(false_label));
                instructions.push(irc::Instruction::Copy {
                    src: irc::Value::Constant(0),
                    dst: result_var,
                });
                instructions.push(irc::Instruction::Label(end_label));
                result
            }
            ast::Expr::Binary {
                operator: ast::BinaryOp::Or,
                left,
                right,
            } => {
                let result_var = self.gen_temp();
                let result = irc::Value::Var(result_var.clone());
                let v1 = self.gen_expr(*left, instructions);
                let true_label = self.gen_label("or_true");
                let end_label = self.gen_label("or_end");

                instructions.push(irc::Instruction::JumpIfNotZero {
                    condition: v1,
                    target: true_label.clone(),
                });
                let v2 = self.gen_expr(*right, instructions);
                instructions.push(irc::Instruction::JumpIfNotZero {
                    condition: v2,
                    target: true_label.clone(),
                });
                instructions.push(irc::Instruction::Copy {
                    src: irc::Value::Constant(0),
                    dst: result_var.clone(),
                });
                instructions.push(irc::Instruction::Jump {
                    target: end_label.clone(),
                });
                instructions.push(irc::Instruction::Label(true_label));
                instructions.push(irc::Instruction::Copy {
                    src: irc::Value::Constant(1),
                    dst: result_var,
                });
                instructions.push(irc::Instruction::Label(end_label));
                result
            }
            ast::Expr::Binary {
                operator,
                left,
                right,
            } => {
                let v1 = self.gen_expr(*left, instructions);
                let v2 = self.gen_expr(*right, instructions);
                let dst_var = self.gen_temp();
                let dst = irc::Value::Var(dst_var.clone());
                let irc_operator = Self::gen_binary(operator);
                instructions.push(irc::Instruction::Binary {
                    operator: irc_operator,
                    src1: v1,
                    src2: v2,
                    dst: dst_var,
                });
                dst
            }
            ast::Expr::Var(name) => irc::Value::Var(name),
            ast::Expr::Assignment { left, right } => {
                let ast::Expr::Var(name) = *left else {
                    unreachable!("Semantic analysis")
                };
                let result = self.gen_expr(*right, instructions);
                instructions.push(irc::Instruction::Copy {
                    src: result,
                    dst: name.clone(),
                });
                irc::Value::Var(name)
            }
        }
    }

    fn gen_stmt(&mut self, stmt: ast::Stmt) -> Vec<irc::Instruction> {
        match stmt {
            ast::Stmt::Return(expr) => {
                let mut instructions = Vec::new();
                let value = self.gen_expr(expr, &mut instructions);
                instructions.push(irc::Instruction::Ret(value));
                instructions
            }
            ast::Stmt::Expression(expr) => {
                let mut instructions = Vec::new();
                self.gen_expr(expr, &mut instructions);
                instructions
            }
            ast::Stmt::Null => Vec::new(),
        }
    }

    fn gen_temp(&mut self) -> String {
        let temp = format!("temp.{counter}", counter = self.counter);
        self.counter += 1;
        temp
    }

    fn gen_label(&mut self, prefix: &str) -> String {
        let label = format!("{prefix}{counter}", counter = self.counter);
        self.counter += 1;
        label
    }

    fn gen_unary(operator: ast::UnaryOp) -> irc::UnaryOp {
        match operator {
            ast::UnaryOp::Complement => irc::UnaryOp::Complement,
            ast::UnaryOp::Negate => irc::UnaryOp::Negate,
            ast::UnaryOp::Not => irc::UnaryOp::Not,
        }
    }

    fn gen_binary(operator: ast::BinaryOp) -> irc::BinaryOp {
        match operator {
            ast::BinaryOp::Add => irc::BinaryOp::Add,
            ast::BinaryOp::Subtract => irc::BinaryOp::Subtract,
            ast::BinaryOp::Multiply => irc::BinaryOp::Multiply,
            ast::BinaryOp::Divide => irc::BinaryOp::Divide,
            ast::BinaryOp::Remainder => irc::BinaryOp::Remainder,
            ast::BinaryOp::LeftShift => irc::BinaryOp::LeftShift,
            ast::BinaryOp::RightShift => irc::BinaryOp::RightShift,
            ast::BinaryOp::BitAnd => irc::BinaryOp::BitAnd,
            ast::BinaryOp::Xor => irc::BinaryOp::Xor,
            ast::BinaryOp::BitOr => irc::BinaryOp::BitOr,
            ast::BinaryOp::Equal => irc::BinaryOp::Equal,
            ast::BinaryOp::NotEqual => irc::BinaryOp::NotEqual,
            ast::BinaryOp::LessThan => irc::BinaryOp::LessThan,
            ast::BinaryOp::LessOrEqual => irc::BinaryOp::LessOrEqual,
            ast::BinaryOp::GreaterThan => irc::BinaryOp::GreaterThan,
            ast::BinaryOp::GreaterOrEqual => irc::BinaryOp::GreaterOrEqual,
            ast::BinaryOp::And | ast::BinaryOp::Or => unreachable!(),
        }
    }
}
