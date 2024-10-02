use crate::asm_ast::{BinaryOp, Function, Instruction, Operand, Program, Register, UnaryOp};

pub fn emit_program(program: Program) -> String {
    match program {
        Program::Function(function) => format!(
            r#"
{function}
.section .note.GNU-stack,"",@progbits
"#,
            function = emit_function(function)
        ),
    }
}

fn emit_function(function: Function) -> String {
    format!(
        r"    .global {name}
{name}:
    pushq    %rbp
    movq    %rsp, %rbp
    {instructions}
",
        name = function.name.lexeme,
        instructions = emit_instructions(function.instructons)
    )
}

fn emit_instructions(instructions: Vec<Instruction>) -> String {
    // PERF: doesn't seem efficient
    instructions
        .into_iter()
        .map(|ins| match ins {
            Instruction::Mov { src, dst } => format!(
                "movl    {src}, {dst}",
                src = emit_operand(src),
                dst = emit_operand(dst)
            ),
            Instruction::Return => r"movq    %rbp, %rsp
    popq    %rbp
    ret"
            .to_string(),
            Instruction::Unary { operator, operand } => {
                format!(
                    "{operator}    {operand}",
                    operand = emit_operand(operand),
                    operator = emit_unary(operator)
                )
            }
            Instruction::AllocateStack(bytes) => format!("subq    $-{bytes}, %rsp"),
            Instruction::Binary {
                operator,
                operand1,
                operand2,
            } => format!(
                "{operator}    {operand1}, {operand2}",
                operator = emit_binary(operator),
                operand1 = emit_operand(operand1),
                operand2 = emit_operand(operand2)
            ),
            Instruction::Idiv(operand) => {
                format!("idivl    {operand}", operand = emit_operand(operand))
            }
            Instruction::Cdq => "cdq".to_string(),
        })
        .collect::<Vec<String>>()
        .join("\n    ")
}

fn emit_operand(operand: Operand) -> String {
    match operand {
        Operand::Imm(value) => format!("${value}"),
        Operand::Register(Register::AX) => "%eax".to_string(),
        Operand::Register(Register::DX) => "%edx".to_string(),
        Operand::Register(Register::CX) => "%ecx".to_string(),
        Operand::Register(Register::R10) => "%r10d".to_string(),
        Operand::Register(Register::R11) => "%r11d".to_string(),
        Operand::Stack(offset) => format!("-{offset}(%rbp)"),
        Operand::Pseudo(_) => unreachable!(),
    }
}

fn emit_unary(operator: UnaryOp) -> String {
    match operator {
        UnaryOp::Neg => "negl".to_string(),
        UnaryOp::Not => "notl".to_string(),
    }
}

fn emit_binary(operator: BinaryOp) -> String {
    match operator {
        BinaryOp::Add => "addl".to_string(),
        BinaryOp::Sub => "subl".to_string(),
        BinaryOp::Mult => "imull".to_string(),
        BinaryOp::And => "and".to_string(),
        BinaryOp::Or => "or".to_string(),
        BinaryOp::Xor => "xor".to_string(),
        BinaryOp::Shl => "sal".to_string(),
        BinaryOp::Shr => "sar".to_string(),
    }
}
