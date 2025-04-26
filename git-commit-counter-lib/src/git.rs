//! Git operations for the Git Commit Counter

use crate::error::CommitCounterError;
use std::path::PathBuf;
use std::process::Command;

/// Trait defining Git operations
pub trait GitOperations {
    fn get_branch(&self) -> Result<String, CommitCounterError>;
    fn get_root_dir(&self) -> Result<PathBuf, CommitCounterError>;
    fn commit(&self, message: &str) -> Result<(), CommitCounterError>;
}

/// Real implementation of Git operations using the git command
pub struct RealGitOps;

impl GitOperations for RealGitOps {
    fn get_branch(&self) -> Result<String, CommitCounterError> {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .map_err(|e| CommitCounterError::GitError(format!("Failed to get branch: {}", e)))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(CommitCounterError::GitError(
                "Not inside a Git repository".to_string(),
            ))
        }
    }

    fn get_root_dir(&self) -> Result<PathBuf, CommitCounterError> {
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()
            .map_err(|e| CommitCounterError::GitError(format!("Failed to get root dir: {}", e)))?;

        if output.status.success() {
            Ok(PathBuf::from(
                String::from_utf8_lossy(&output.stdout).trim(),
            ))
        } else {
            Err(CommitCounterError::GitError(
                "Not inside a Git repository".to_string(),
            ))
        }
    }

    fn commit(&self, message: &str) -> Result<(), CommitCounterError> {
        let status = Command::new("git")
            .args(["commit", "-m", message])
            .status()
            .map_err(|e| CommitCounterError::GitError(format!("Failed to execute git: {}", e)))?;

        if status.success() {
            Ok(())
        } else {
            Err(CommitCounterError::GitError("Commit failed".to_string()))
        }
    }
}

/// Get the project name from the Git root directory
pub fn get_project_name() -> Result<String, CommitCounterError> {
    let git_ops = RealGitOps;
    git_ops.get_root_dir().and_then(|p| {
        p.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .ok_or_else(|| {
                CommitCounterError::GitError("Cannot determine project name".to_string())
            })
    })
}
