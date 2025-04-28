//! Configuration for Git Commit Counter

use crate::error::CommitCounterError;
use crate::git::{GitOperations, RealGitOps, get_project_name};
use std::path::{Path, PathBuf};

/// Configuration for the Git Commit Counter
pub struct Config {
    project_name: String,
    branch_name: String,
    counter_file_path: PathBuf,
}

impl Config {
    /// Create a new configuration
    pub fn new() -> Result<Self, CommitCounterError> {
        let git_ops = RealGitOps;
        let branch = git_ops.get_branch()?;
        let project = get_project_name()?;

        let mut path = dirs::home_dir().ok_or_else(|| {
            CommitCounterError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Home directory not found",
            ))
        })?;

        path.push(format!(".git_commit_counts_{}_{}", project, branch));

        Ok(Config {
            project_name: project,
            branch_name: branch,
            counter_file_path: path,
        })
    }

    /// Get the path to the counter file
    pub fn counter_file_path(&self) -> &Path {
        &self.counter_file_path
    }

    /// Get the branch name
    pub fn branch_name(&self) -> &str {
        &self.branch_name
    }

    /// Get the project name
    pub fn project_name(&self) -> &str {
        &self.project_name
    }
}
