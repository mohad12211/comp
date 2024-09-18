use std::{
    fs,
    path::PathBuf,
    process::{Command, ExitCode},
};

use clap::Parser;
use comp::{lexer::Lexer, parser, Error, Result};

/// C Compiler
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Only run the lexer
    #[arg(short, long)]
    lex: bool,

    /// Only run the lexer and the parser
    #[arg(short, long)]
    parse: bool,

    /// Only run the lexer, the parser, and assembly generation, stop before code emission
    #[arg(short, long = "code-gen")]
    code_gen: bool,

    /// Only emit assembly file
    #[arg(short = 'S', long)]
    assembly: bool,

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
    println!("{:?}", lexer.tokens);
    if cli.lex {
        return Ok(());
    }
    let mut parser = parser::Parser::new(&lexer);
    let program = parser.parse()?;
    println!("{program:#?}");
    if cli.parse {
        return Ok(());
    }
    // temp "stub out"
    // let output = Command::new("gcc")
    //     .args(["-S", &format!("{file}.i"), "-o", "-"])
    //     .output()
    //     .unwrap();
    //
    // fs::write(format!("{file}.s"), output.stdout)
    //     .map_err(|e| Error::IO(format!("Couldn't write file '{file}.s': - {e}")))?;
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
    preprocess(&file)?;
    compile(&file, cli)?;
    let _ = fs::remove_file(format!("{file}.i"));
    if cli.assembly || cli.lex || cli.parse || cli.code_gen {
        return Ok(());
    }
    let _ = fs::remove_file(format!("{file}.s"));
    assemble(&file)?;
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
