use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrainfuckError {
    #[error("Syntax error: Unmatched bracket at position {0}")]
    UnmatchedBracket(usize),
    #[error("Memory error: Pointer out of bounds")]
    PointerOutOfBounds,
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}
