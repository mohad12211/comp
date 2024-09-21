use crate::{asm_ast, irc};

pub fn gen_program(program: irc::Program) -> asm_ast::Program {
    match program {
        irc::Program::Function(function) => asm_ast::Program::Function(gen_function(function)),
    }
}

fn gen_function(function: irc::Function) -> asm_ast::Function {
    asm_ast::Function {
        name: function.name,
        instructons: function
            .instructons
            .into_iter()
            .map(gen_instruction)
            .flatten()
            .collect(),
    }
}

fn gen_instruction(instruction: irc::Instruction) -> Vec<asm_ast::Instruction> {
    match instruction {
        irc::Instruction::Ret(value) => vec![
            asm_ast::Instruction::Mov {
                src: gen_operand(value),
                dst: asm_ast::Operand::Register(asm_ast::Register::AX),
            },
            asm_ast::Instruction::Return,
        ],
        irc::Instruction::Unary { operator, src, dst } => {
            let dst = gen_operand(irc::Value::Var(dst));
            vec![
                asm_ast::Instruction::Mov {
                    src: gen_operand(src),
                    dst,
                },
                asm_ast::Instruction::Unary {
                    operator: gen_operator(operator),
                    operand: dst,
                },
            ]
        }
    }
}

fn gen_operand(value: irc::Value) -> asm_ast::Operand {
    match value {
        irc::Value::Constant(value) => asm_ast::Operand::Imm(value),
        irc::Value::Var(counter) => asm_ast::Operand::Pseudo(counter),
    }
}

fn gen_operator(operator: irc::UnaryOp) -> asm_ast::UnaryOp {
    match operator {
        irc::UnaryOp::Complement => asm_ast::UnaryOp::Not,
        irc::UnaryOp::Negate => asm_ast::UnaryOp::Neg,
    }
}
