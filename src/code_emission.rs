use crate::asm_ast::{Function, Instruction, Operand, Program};

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
        r"
    .global {name}
{name}:
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
                "movl {src}, {dst}",
                src = emit_operand(src),
                dst = emit_operand(dst)
            ),
            Instruction::Return => "ret".to_string(),
        })
        .collect::<Vec<String>>()
        .join("\n    ")
}

fn emit_operand(operand: &Operand) -> String {
    match operand {
        Operand::Imm(value) => format!("${value}"),
        Operand::Register => "%eax".to_string(),
    }
}
