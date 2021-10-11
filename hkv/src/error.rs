use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum KvError {
    #[error("Not fount for table: {0}, key: {1}")]
    NotFound(String, String),

    #[error("Command is invalid: {0}")]
    InvalidCommand(String),

    #[error("Internal error: {0}")]
    Internal(String),
}