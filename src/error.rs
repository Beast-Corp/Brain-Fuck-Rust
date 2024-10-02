use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrainfuckError {
    #[error("Syntax error: Unmatched bracket at line {line}, column {column}")]
    UnmatchedBracket { line: usize, column: usize },
    #[error("Memory error: Pointer out of bounds at line {line}, column {column}")]
    PointerOutOfBounds { line: usize, column: usize },
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),
}
