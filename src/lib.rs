#[derive(Debug)]
pub enum Error {
    IO(String),
    Preprocess(String),
    Assemble(String),
}

pub type Result<T> = std::result::Result<T, Error>;
