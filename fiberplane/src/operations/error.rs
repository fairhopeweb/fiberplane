use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum Error {
    #[error("cell not found")]
    CellNotFound(String),

    #[error("content length does not match")]
    ContentLengthMismatch(u32),

    #[error("attempt to insert a duplicate ID")]
    DuplicateId(String),

    #[error("insert index not valid")]
    InvalidInsertIndex(u32),

    #[error("split index not valid on cell")]
    InvalidSplitIndex(u32, String),

    #[error("cell has no content")]
    NoContentCell(String),

    #[error("unauthorized")]
    Unauthorized(String),
}
