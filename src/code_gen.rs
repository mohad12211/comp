use crate::{asm_ast, ast};

pub fn gen_program(program: ast::Program) -> asm_ast::Program {
    match program {
        ast::Program::Function(function) => asm_ast::Program::Function(gen_function(function)),
    }
}

fn gen_function(function: ast::Function) -> asm_ast::Function {
    asm_ast::Function {
        name: function.name,
        instructons: gen_stmt(function.body),
    }
}

fn gen_expr(expr: ast::Expr) -> asm_ast::Operand {
    match expr {
        ast::Expr::Constant(value) => asm_ast::Operand::Imm(value),
        ast::Expr::UnaryOp { operator, right } => todo!(),
    }
}

fn gen_stmt(stmt: ast::Stmt) -> Vec<asm_ast::Instruction> {
    match stmt {
        ast::Stmt::Return(expr) => vec![
            asm_ast::Instruction::Mov {
                src: gen_expr(expr),
                dst: asm_ast::Operand::Register,
            },
            asm_ast::Instruction::Ret,
        ],
    }
}
