use parser::ParseError;

pub mod asm_ast;
pub mod ast;
pub mod code_emission;
pub mod code_gen;
pub mod lexer;
pub mod parser;
pub mod token;

#[derive(Debug)]
pub enum Error {
    IO(String),
    Preprocess(String),
    Assemble(String),
    Lexer(String),
    InvalidToken(String),
    Parser(ParseError),
}

impl From<ParseError> for Error {
    fn from(parser_error: ParseError) -> Self {
        Self::Parser(parser_error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
