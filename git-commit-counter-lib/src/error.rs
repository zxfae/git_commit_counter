//! Error types for the Git Commit Counter library

use std::io;
use thiserror::Error;

/// Custom error type for the Git Commit Counter
#[derive(Error, Debug)]
pub enum CommitCounterError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Git error: {0}")]
    GitError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}
