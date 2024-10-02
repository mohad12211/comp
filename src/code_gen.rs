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
            .flat_map(gen_instruction)
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
        irc::Instruction::Unary {
            operator: irc::UnaryOp::Not,
            src,
            dst,
        } => {
            let dst = gen_operand(irc::Value::Var(dst));
            vec![
                asm_ast::Instruction::Cmp {
                    operand1: asm_ast::Operand::Imm(0),
                    operand2: gen_operand(src),
                },
                asm_ast::Instruction::Mov {
                    src: asm_ast::Operand::Imm(0),
                    dst,
                },
                asm_ast::Instruction::SetCC {
                    cond_code: asm_ast::CondCode::E,
                    operand: dst,
                },
            ]
        }
        irc::Instruction::Unary { operator, src, dst } => {
            let dst = gen_operand(irc::Value::Var(dst));
            vec![
                asm_ast::Instruction::Mov {
                    src: gen_operand(src),
                    dst,
                },
                asm_ast::Instruction::Unary {
                    operator: gen_unary(&operator),
                    operand: dst,
                },
            ]
        }
        irc::Instruction::Binary {
            operator,
            src1,
            src2,
            dst,
        } => {
            let dst = gen_operand(irc::Value::Var(dst));
            match operator {
                irc::BinaryOp::Add => gen_binary_ins(asm_ast::BinaryOp::Add, src1, src2, dst),
                irc::BinaryOp::Subtract => gen_binary_ins(asm_ast::BinaryOp::Sub, src1, src2, dst),
                irc::BinaryOp::Multiply => gen_binary_ins(asm_ast::BinaryOp::Mult, src1, src2, dst),
                irc::BinaryOp::LeftShift => gen_binary_ins(asm_ast::BinaryOp::Shl, src1, src2, dst),
                irc::BinaryOp::RightShift => {
                    gen_binary_ins(asm_ast::BinaryOp::Shr, src1, src2, dst)
                }
                irc::BinaryOp::And => gen_binary_ins(asm_ast::BinaryOp::And, src1, src2, dst),
                irc::BinaryOp::Xor => gen_binary_ins(asm_ast::BinaryOp::Xor, src1, src2, dst),
                irc::BinaryOp::Or => gen_binary_ins(asm_ast::BinaryOp::Or, src1, src2, dst),
                irc::BinaryOp::Equal => gen_binary_rel(asm_ast::CondCode::E, src1, src2, dst),
                irc::BinaryOp::NotEqual => gen_binary_rel(asm_ast::CondCode::NE, src1, src2, dst),
                irc::BinaryOp::LessThan => gen_binary_rel(asm_ast::CondCode::L, src1, src2, dst),
                irc::BinaryOp::LessOrEqual => {
                    gen_binary_rel(asm_ast::CondCode::LE, src1, src2, dst)
                }
                irc::BinaryOp::GreaterThan => gen_binary_rel(asm_ast::CondCode::G, src1, src2, dst),
                irc::BinaryOp::GreaterOrEqual => {
                    gen_binary_rel(asm_ast::CondCode::GE, src1, src2, dst)
                }
                irc::BinaryOp::Divide => {
                    vec![
                        asm_ast::Instruction::Mov {
                            src: gen_operand(src1),
                            dst: asm_ast::Operand::Register(asm_ast::Register::AX),
                        },
                        asm_ast::Instruction::Cdq,
                        asm_ast::Instruction::Idiv(gen_operand(src2)),
                        asm_ast::Instruction::Mov {
                            src: asm_ast::Operand::Register(asm_ast::Register::AX),
                            dst,
                        },
                    ]
                }
                irc::BinaryOp::Remainder => vec![
                    asm_ast::Instruction::Mov {
                        src: gen_operand(src1),
                        dst: asm_ast::Operand::Register(asm_ast::Register::AX),
                    },
                    asm_ast::Instruction::Cdq,
                    asm_ast::Instruction::Idiv(gen_operand(src2)),
                    asm_ast::Instruction::Mov {
                        src: asm_ast::Operand::Register(asm_ast::Register::DX),
                        dst,
                    },
                ],
            }
        }
        irc::Instruction::Copy { src, dst } => vec![asm_ast::Instruction::Mov {
            src: gen_operand(src),
            dst: gen_operand(irc::Value::Var(dst)),
        }],
        irc::Instruction::Jump { target } => vec![asm_ast::Instruction::Jmp(target)],
        irc::Instruction::JumpIfZero { condition, target } => {
            vec![
                asm_ast::Instruction::Cmp {
                    operand1: asm_ast::Operand::Imm(0),
                    operand2: gen_operand(condition),
                },
                asm_ast::Instruction::JumpCC {
                    cond_code: asm_ast::CondCode::E,
                    target,
                },
            ]
        }
        irc::Instruction::JumpIfNotZero { condition, target } => vec![
            asm_ast::Instruction::Cmp {
                operand1: asm_ast::Operand::Imm(0),
                operand2: gen_operand(condition),
            },
            asm_ast::Instruction::JumpCC {
                cond_code: asm_ast::CondCode::NE,
                target,
            },
        ],
        irc::Instruction::Label(target) => vec![asm_ast::Instruction::Label(target)],
    }
}

fn gen_operand(value: irc::Value) -> asm_ast::Operand {
    match value {
        irc::Value::Constant(value) => asm_ast::Operand::Imm(value),
        irc::Value::Var(counter) => asm_ast::Operand::Pseudo(counter),
    }
}

fn gen_unary(operator: &irc::UnaryOp) -> asm_ast::UnaryOp {
    match operator {
        irc::UnaryOp::Complement => asm_ast::UnaryOp::Not,
        irc::UnaryOp::Negate => asm_ast::UnaryOp::Neg,
        irc::UnaryOp::Not => asm_ast::UnaryOp::Not,
    }
}

fn gen_binary_rel(
    cond_code: asm_ast::CondCode,
    src1: irc::Value,
    src2: irc::Value,
    dst: asm_ast::Operand,
) -> Vec<asm_ast::Instruction> {
    vec![
        asm_ast::Instruction::Cmp {
            operand1: gen_operand(src2),
            operand2: gen_operand(src1),
        },
        asm_ast::Instruction::Mov {
            src: asm_ast::Operand::Imm(0),
            dst,
        },
        asm_ast::Instruction::SetCC {
            cond_code,
            operand: dst,
        },
    ]
}

fn gen_binary_ins(
    operator: asm_ast::BinaryOp,
    src1: irc::Value,
    src2: irc::Value,
    dst: asm_ast::Operand,
) -> Vec<asm_ast::Instruction> {
    vec![
        asm_ast::Instruction::Mov {
            src: gen_operand(src1),
            dst,
        },
        asm_ast::Instruction::Binary {
            operator,
            operand1: gen_operand(src2),
            operand2: dst,
        },
    ]
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
                asm_ast::Instruction::Binary {
                    operator: _,
                    operand1,
                    operand2,
                } => {
                    replace_operand(operand1);
                    replace_operand(operand2);
                }
                asm_ast::Instruction::Idiv(operand) => {
                    replace_operand(operand);
                }
                asm_ast::Instruction::Cmp { operand1, operand2 } => {
                    replace_operand(operand1);
                    replace_operand(operand2);
                }
                asm_ast::Instruction::SetCC {
                    cond_code: _,
                    operand,
                } => {
                    replace_operand(operand);
                }
                asm_ast::Instruction::Return
                | asm_ast::Instruction::Cdq
                | asm_ast::Instruction::AllocateStack(_)
                | asm_ast::Instruction::Label(_)
                | asm_ast::Instruction::Jmp(_)
                | asm_ast::Instruction::JumpCC {
                    cond_code: _,
                    target: _,
                } => {}
            });
        }
    }
}

pub fn fix_instructions(program: &mut asm_ast::Program, stack_allocation: usize) {
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
                        asm_ast::Instruction::Idiv(src @ asm_ast::Operand::Imm(_)) => {
                            vec![
                                asm_ast::Instruction::Mov {
                                    src: *src,
                                    dst: asm_ast::Operand::Register(asm_ast::Register::R10),
                                },
                                asm_ast::Instruction::Idiv(asm_ast::Operand::Register(
                                    asm_ast::Register::R10,
                                )),
                            ]
                        }
                        asm_ast::Instruction::Binary {
                            operator:
                                operator @ (asm_ast::BinaryOp::Add
                                | asm_ast::BinaryOp::Sub
                                | asm_ast::BinaryOp::And
                                | asm_ast::BinaryOp::Xor
                                | asm_ast::BinaryOp::Or),
                            operand1: asm_ast::Operand::Stack(src),
                            operand2: asm_ast::Operand::Stack(dst),
                        } => {
                            vec![
                                asm_ast::Instruction::Mov {
                                    src: asm_ast::Operand::Stack(*src),
                                    dst: asm_ast::Operand::Register(asm_ast::Register::R10),
                                },
                                asm_ast::Instruction::Binary {
                                    operator: *operator,
                                    operand1: asm_ast::Operand::Register(asm_ast::Register::R10),
                                    operand2: asm_ast::Operand::Stack(*dst),
                                },
                            ]
                        }
                        asm_ast::Instruction::Binary {
                            operator: operator @ asm_ast::BinaryOp::Mult,
                            operand1,
                            operand2: src @ asm_ast::Operand::Stack(_),
                        } => {
                            vec![
                                asm_ast::Instruction::Mov {
                                    src: *src,
                                    dst: asm_ast::Operand::Register(asm_ast::Register::R11),
                                },
                                asm_ast::Instruction::Binary {
                                    operator: *operator,
                                    operand1: *operand1,
                                    operand2: asm_ast::Operand::Register(asm_ast::Register::R11),
                                },
                                asm_ast::Instruction::Mov {
                                    src: asm_ast::Operand::Register(asm_ast::Register::R11),
                                    dst: *src,
                                },
                            ]
                        }
                        asm_ast::Instruction::Binary {
                            operator: operator @ (asm_ast::BinaryOp::Shr | asm_ast::BinaryOp::Shl),
                            operand1: src @ asm_ast::Operand::Stack(_),
                            operand2,
                        } => {
                            vec![
                                asm_ast::Instruction::Mov {
                                    src: *src,
                                    dst: asm_ast::Operand::Register(asm_ast::Register::CX),
                                },
                                asm_ast::Instruction::Binary {
                                    operator: *operator,
                                    operand1: asm_ast::Operand::Register(asm_ast::Register::CX),
                                    operand2: *operand2,
                                },
                            ]
                        }
                        asm_ast::Instruction::Cmp {
                            operand1: asm_ast::Operand::Stack(operand1),
                            operand2: asm_ast::Operand::Stack(operand2),
                        } => {
                            vec![
                                asm_ast::Instruction::Mov {
                                    src: asm_ast::Operand::Stack(*operand1),
                                    dst: asm_ast::Operand::Register(asm_ast::Register::R10),
                                },
                                asm_ast::Instruction::Mov {
                                    src: asm_ast::Operand::Register(asm_ast::Register::R10),
                                    dst: asm_ast::Operand::Stack(*operand2),
                                },
                            ]
                        }
                        asm_ast::Instruction::Cmp {
                            operand1,
                            operand2: operand2 @ asm_ast::Operand::Imm(_),
                        } => {
                            vec![
                                asm_ast::Instruction::Mov {
                                    src: *operand2,
                                    dst: asm_ast::Operand::Register(asm_ast::Register::R11),
                                },
                                asm_ast::Instruction::Cmp {
                                    operand1: *operand1,
                                    operand2: asm_ast::Operand::Register(asm_ast::Register::R11),
                                },
                            ]
                        }
                        _ => vec![ins.clone()],
                    }))
                    .collect();
        }
    }
}

fn replace_operand(operand: &mut asm_ast::Operand) {
    if let asm_ast::Operand::Pseudo(counter) = operand {
        *operand = asm_ast::Operand::Stack(*counter * 4);
    }
}
