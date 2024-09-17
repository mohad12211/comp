use std::{
    fs,
    path::PathBuf,
    process::{Command, ExitCode},
};

use clap::Parser;
use comp::{Error, Result};

/// C Compiler
#[derive(Parser, Debug)]
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

fn compile(file: &str) -> Result<()> {
    // temp "stub out"
    let output = Command::new("gcc")
        .args(["-S", &format!("{file}.i"), "-o", "-"])
        .output()
        .unwrap();

    fs::write(format!("{file}.s"), output.stdout)
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
    preprocess(&file)?;
    compile(&file)?;
    let _ = fs::remove_file(format!("{file}.i"));
    if cli.assembly {
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
                Error::IO(err) => println!("System IO Error:\n - {err}"),
                Error::Preprocess(err) => println!("Preprocessor Error:\n - {err}"),
                Error::Assemble(err) => println!("Assembling Error:\n - {err}"),
            };
            let _ = fs::remove_file(format!("{file}.i"));
            let _ = fs::remove_file(format!("{file}.s"));
            let _ = fs::remove_file(format!("{file}"));
            ExitCode::FAILURE
        }
    }
}
