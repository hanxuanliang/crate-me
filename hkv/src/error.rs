use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum KvError {
    #[error("Not fount for table: {0}, key: {1}")]
    NotFound(String, String),

    #[error("Command is invalid: {0}")]
    InvalidCommand(String),

    #[error("Cannot process Cmd {0} in table {1}, key {2}: Err({})")]
    StorageError(&'static str, String, String, String),

    #[error("Internal error: {0}")]
    Internal(String),
}