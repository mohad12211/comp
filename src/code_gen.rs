use std::iter;

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

pub fn replace_pseudo(program: &mut asm_ast::Program) {
    match program {
        asm_ast::Program::Function(function) => {
            function.instructons.iter_mut().for_each(|ins| match ins {
                asm_ast::Instruction::Mov { src, dst } => {
                    replace_operand(src);
                    replace_operand(dst);
                }
                asm_ast::Instruction::Unary {
                    operator: _,
                    operand,
                } => replace_operand(operand),
                _ => {}
            });
        }
    }
}

pub fn fix_instructions(program: &mut asm_ast::Program, stack_allocation: i32) {
    match program {
        asm_ast::Program::Function(function) => {
            function.instructons =
                iter::once(asm_ast::Instruction::AllocateStack(stack_allocation))
                    .chain(function.instructons.iter().flat_map(|ins| match ins {
                        asm_ast::Instruction::Mov {
                            src: asm_ast::Operand::Stack(src),
                            dst: asm_ast::Operand::Stack(dst),
                        } => vec![
                            asm_ast::Instruction::Mov {
                                src: asm_ast::Operand::Stack(*src),
                                dst: asm_ast::Operand::Register(asm_ast::Register::R10),
                            },
                            asm_ast::Instruction::Mov {
                                src: asm_ast::Operand::Register(asm_ast::Register::R10),
                                dst: asm_ast::Operand::Stack(*dst),
                            },
                        ],
                        _ => vec![*ins],
                    }))
                    .collect();
        }
    }
}

fn replace_operand(operand: &mut asm_ast::Operand) {
    match operand {
        asm_ast::Operand::Pseudo(counter) => {
            *operand = asm_ast::Operand::Stack((*counter + 1) as i32 * -4)
        }
        _ => {}
    }
}
