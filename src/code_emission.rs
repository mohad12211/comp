use crate::asm_ast::{
    BinaryOp, CondCode, Function, Instruction, Operand, Program, Register, RegisterSize, UnaryOp,
};

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
        name = function.name,
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
                src = emit_operand(src, RegisterSize::Four),
                dst = emit_operand(dst, RegisterSize::Four)
            ),
            Instruction::Return => r"movq    %rbp, %rsp
    popq    %rbp
    ret"
            .to_string(),
            Instruction::Unary { operator, operand } => {
                format!(
                    "{operator}    {operand}",
                    operand = emit_operand(operand, RegisterSize::Four),
                    operator = emit_unary(operator)
                )
            }
            Instruction::AllocateStack(bytes) => format!("subq    ${bytes}, %rsp"),
            Instruction::Binary {
                operator,
                operand1,
                operand2,
            } => format!(
                "{operator}    {operand1}, {operand2}",
                operator = emit_binary(operator),
                operand1 = emit_operand(
                    operand1,
                    if matches!(operator, BinaryOp::Shl | BinaryOp::Shr) {
                        RegisterSize::One
                    } else {
                        RegisterSize::Four
                    }
                ),
                operand2 = emit_operand(operand2, RegisterSize::Four)
            ),
            Instruction::Idiv(operand) => {
                format!(
                    "idivl    {operand}",
                    operand = emit_operand(operand, RegisterSize::Four)
                )
            }
            Instruction::Cdq => "cdq".to_string(),
            Instruction::Cmp { operand1, operand2 } => format!(
                "cmpl    {operand1}, {operand2}",
                operand1 = emit_operand(operand1, RegisterSize::Four),
                operand2 = emit_operand(operand2, RegisterSize::Four)
            ),
            Instruction::Jmp(label) => format!("jmp    .L{label}"),
            Instruction::JumpCC { cond_code, target } => format!(
                "j{cond_code}    .L{target}",
                cond_code = emit_cond_code(cond_code)
            ),
            Instruction::SetCC { cond_code, operand } => format!(
                "set{cond_code}    {operand}",
                cond_code = emit_cond_code(cond_code),
                operand = emit_operand(operand, RegisterSize::One)
            ),
            Instruction::Label(label) => format!(".L{label}:"),
        })
        .collect::<Vec<String>>()
        .join("\n    ")
}

fn emit_operand(operand: Operand, size: RegisterSize) -> String {
    match (operand, size) {
        (Operand::Register(Register::AX), RegisterSize::Four) => "%eax".to_string(),
        (Operand::Register(Register::AX), RegisterSize::One) => "%al".to_string(),
        (Operand::Register(Register::DX), RegisterSize::Four) => "%edx".to_string(),
        (Operand::Register(Register::DX), RegisterSize::One) => "%dl".to_string(),
        (Operand::Register(Register::CX), RegisterSize::Four) => "%ecx".to_string(),
        (Operand::Register(Register::CX), RegisterSize::One) => "%cl".to_string(),
        (Operand::Register(Register::R10), RegisterSize::Four) => "%r10d".to_string(),
        (Operand::Register(Register::R10), RegisterSize::One) => "%r10b".to_string(),
        (Operand::Register(Register::R11), RegisterSize::Four) => "%r11d".to_string(),
        (Operand::Register(Register::R11), RegisterSize::One) => "%r11b".to_string(),
        (Operand::Imm(value), _) => format!("${value}"),
        (Operand::Stack(offset), _) => format!("-{offset}(%rbp)"),
        (Operand::Pseudo(_), _) => unreachable!(),
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
        BinaryOp::And => "andl".to_string(),
        BinaryOp::Or => "orl".to_string(),
        BinaryOp::Xor => "xorl".to_string(),
        BinaryOp::Shl => "sall".to_string(),
        BinaryOp::Shr => "sarl".to_string(),
    }
}

fn emit_cond_code(cond_code: CondCode) -> String {
    match cond_code {
        CondCode::E => "e".to_string(),
        CondCode::NE => "ne".to_string(),
        CondCode::G => "g".to_string(),
        CondCode::GE => "ge".to_string(),
        CondCode::L => "l".to_string(),
        CondCode::LE => "le".to_string(),
    }
}
