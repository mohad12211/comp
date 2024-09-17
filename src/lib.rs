pub mod lexer;
pub mod token;

#[derive(Debug)]
pub enum Error {
    IO(String),
    Preprocess(String),
    Assemble(String),
    Lexer(String),
    InvalidToken(String),
}

pub type Result<T> = std::result::Result<T, Error>;
