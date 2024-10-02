use std::{
    fs,
    path::PathBuf,
    process::{Command, ExitCode},
};

use clap::Parser;
use comp::{code_emission, code_gen, irc_gen::IrcGenerator, lexer::Lexer, parser, Error, Result};

/// C Compiler
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Stop after lexing
    #[arg(short, long)]
    lex: bool,

    /// Stop after parsing
    #[arg(short, long)]
    parse: bool,

    /// Stop after code gen
    #[arg(short, long = "codegen")]
    code_gen: bool,

    /// Stop after generating assembly
    #[arg(short = 'S', long)]
    assembly: bool,

    /// Stop after generating irc
    #[arg(short, long, alias = "tacky")]
    irc: bool,

    /// C source code file
    #[arg(required = true)]
    file: String,
}

fn preprocess(file: &str) -> Result<()> {
    let output = Command::new("gcc")
        .args(["-E", "-P", &format!("{file}.c"), "-o", &format!("{file}.i")])
        .output()
        .map_err(|e| Error::IO(format!("Couldn't run gcc to preprocess:\n - {e}")))?;
    if output.status.code() != Some(0) {
        return Err(Error::Preprocess(format!(
            "Error preprocessing file '{file}.c':\n - stderr: '{}'",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(())
}

fn compile(file: &str, cli: &Cli) -> Result<()> {
    let source = fs::read_to_string(format!("{file}.i"))
        .map_err(|e| Error::IO(format!("Couldn't read file `{file}`:\n - {e}")))?;
    let mut lexer = Lexer::new(&source);
    lexer.tokenize()?;
    if cli.lex {
        return Ok(());
    }
    let mut parser = parser::Parser::new(&lexer);
    let ast = parser.parse()?;
    if cli.parse {
        return Ok(());
    }
    let (irc, stack_allocation) = IrcGenerator::gen_program(ast);
    if cli.irc {
        return Ok(());
    }
    let mut asm_program = code_gen::gen_program(irc);
    code_gen::replace_pseudo(&mut asm_program);
    code_gen::fix_instructions(&mut asm_program, stack_allocation);
    if cli.code_gen {
        return Ok(());
    }
    let assembly = code_emission::emit_program(asm_program);
    fs::write(format!("{file}.s"), assembly)
        .map_err(|e| Error::IO(format!("Couldn't write file '{file}.s': - {e}")))?;
    Ok(())
}

fn assemble(file: &str) -> Result<()> {
    let output = Command::new("gcc")
        .args([&format!("{file}.s"), "-o", file])
        .output()
        .map_err(|e| Error::IO(format!("Couldn't run gcc to assemble:\n - {e}")))?;

    if output.status.code() != Some(0) {
        return Err(Error::Assemble(format!(
            "Error Assembling file '{file}.s':\n - stderr: '{}'",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(())
}

fn run(file: &str, cli: &Cli) -> Result<()> {
    preprocess(file)?;
    compile(file, cli)?;
    let _ = fs::remove_file(format!("{file}.i"));
    if cli.assembly || cli.lex || cli.parse || cli.code_gen || cli.irc {
        return Ok(());
    }
    assemble(file)?;
    let _ = fs::remove_file(format!("{file}.s"));
    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let mut file = PathBuf::from(&cli.file);
    file.set_extension("");
    let file = file.to_string_lossy();
    match run(&file, &cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            match err {
                Error::IO(err) => eprintln!("System IO Error:\n - {err}"),
                Error::Preprocess(err) => eprintln!("Preprocessor Error:\n - {err}"),
                Error::Assemble(err) => eprintln!("Assembling Error:\n - {err}"),
                Error::Lexer(err) => eprintln!("Lexer Error:\n - {err}"),
                Error::InvalidToken(err) => eprintln!("Invalid Token:\n - {err}"),
                Error::Parser(parse_error) => eprintln!("Parser Error:\n - {parse_error}"),
            };
            let _ = fs::remove_file(format!("{file}.i"));
            let _ = fs::remove_file(format!("{file}.s"));
            let _ = fs::remove_file(format!("{file}"));
            ExitCode::FAILURE
        }
    }
}
