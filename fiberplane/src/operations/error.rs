use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("cell not found")]
    CellNotFound(String),

    #[error("attempt to insert a duplicate ID")]
    DuplicateId(String),

    #[error("internal error")]
    InternalError(String),

    #[error("insert index not valid")]
    InvalidInsertIndex(u32),

    #[error("the given offset is out-of-range for that cell")]
    InvalidTextOffset(String, u32),

    #[error("cell has no text field")]
    NoTextCell(String),

    #[error("unauthorized")]
    Unauthorized(String),
}
