//! Git Commit Counter library
//!
//! This library provides functionality for counting and formatting Git commits
//! based on their type (FEAT, FIX, etc.) and branch.

mod commit_types;
mod config;
mod counter;
mod error;
mod git;

// Re-export the public API
pub use commit_types::CommitType;
pub use config::Config;
pub use counter::CommitCounter;
pub use error::CommitCounterError;
pub use git::{GitOperations, RealGitOps};
