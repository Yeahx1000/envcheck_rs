use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("io error {0")]
    Io(#[from] std::io::Error),

    #[error("parse error on line {line}: {msg}")]
    ParseError { line: usize, msg: String },
}

pub type AppResult<T> = Result<T, AppError>;
