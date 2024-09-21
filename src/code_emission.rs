use crate::asm_ast::{Function, Instruction, Operand, Program, Register, UnaryOp};

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
        instructions = emit_instructions(&function.instructons)
    )
}

fn emit_instructions(instructions: &[Instruction]) -> String {
    // PERF: doesn't seem efficient
    instructions
        .iter()
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
                    operator = emit_operator(operator)
                )
            }
            Instruction::AllocateStack(bytes) => format!("subq    ${bytes}, %rsp"),
        })
        .collect::<Vec<String>>()
        .join("\n    ")
}

fn emit_operand(operand: &Operand) -> String {
    match operand {
        Operand::Imm(value) => format!("${value}"),
        Operand::Register(Register::AX) => "%eax".to_string(),
        Operand::Register(Register::R10) => "%r10d".to_string(),
        Operand::Stack(offset) => format!("{offset}(%rbp)"),
        Operand::Pseudo(_) => unreachable!(),
    }
}

fn emit_operator(operator: &UnaryOp) -> String {
    match operator {
        UnaryOp::Neg => "negl".to_string(),
        UnaryOp::Not => "notl".to_string(),
    }
}
