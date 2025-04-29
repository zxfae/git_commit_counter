//! Configuration for Git Commit Counter

use dirs::config_dir;

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

        let mut config_path = config_dir().ok_or_else(|| {
            CommitCounterError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Home directory not found",
            ))
        })?;

        config_path.push("git_commit_counter");

        std::fs::create_dir_all(&config_path).map_err(|e| {
            CommitCounterError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create configuration repository : {}", e),
            ))
        })?;
        
        // Sanitize branch name for file path
        let safe_branch = Self::sanitize_branch_name(&branch);
        config_path.push(format!("{}_{}.counts", project, safe_branch));

        Ok(Config {
            project_name: project,
            branch_name: branch,
            counter_file_path: config_path,
        })
    }

    /// branch names like "name/test" or "feature-123" or "test" can be safely used in file names
    fn sanitize_branch_name(branch: &str) -> String {
        branch.replace('/', "_")
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
