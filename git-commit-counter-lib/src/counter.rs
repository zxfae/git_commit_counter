//! Counter logic for Git Commit Counter
use crate::commit_types::CommitType;
use crate::config::Config;
use crate::error::CommitCounterError;
use crate::git::GitOperations;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

/// Counter for Git commits
pub struct CommitCounter {
    config: Config,
    git_ops: Box<dyn GitOperations>,
}

impl CommitCounter {
    /// Create a new commit counter with the given Git operations
    pub fn new(git_ops: Box<dyn GitOperations>) -> Result<Self, CommitCounterError> {
        let config = Config::new()?;
        Ok(CommitCounter { config, git_ops })
    }

    /// Read all counters from the counter file
    pub fn read_all_counts(&self) -> Result<HashMap<String, u32>, CommitCounterError> {
        let file_path = self.config.counter_file_path();
        let mut map = HashMap::new();

        if file_path.exists() {
            let file = File::open(file_path)?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line?;
                let parts: Vec<&str> = line.trim().split_whitespace().collect();

                if parts.len() == 2 {
                    let count = parts[1].parse::<u32>().map_err(|e| {
                        CommitCounterError::ParseError(format!("Failed to parse count: {}", e))
                    })?;

                    map.insert(parts[0].to_string(), count);
                }
            }
        }

        Ok(map)
    }

    /// Read the counter for a specific commit type
    pub fn read_count(&self, commit_type: &CommitType) -> Result<u32, CommitCounterError> {
        let counts = self.read_all_counts()?;
        let type_str = commit_type.to_string();
        Ok(counts.get(&type_str).copied().unwrap_or(0))
    }

    /// Update the counter for a specific commit type
    pub fn update_count(
        &self,
        commit_type: &CommitType,
        count: u32,
    ) -> Result<(), CommitCounterError> {
        let mut counts = self.read_all_counts()?;
        counts.insert(commit_type.to_string(), count);

        let file_path = self.config.counter_file_path();
        let mut file = File::create(file_path)?;

        for (k, v) in counts {
            writeln!(file, "{} {}", k, v)?;
        }

        Ok(())
    }

    /// Increment the counter for a specific commit type
    pub fn increment_count(&self, commit_type: &CommitType) -> Result<u32, CommitCounterError> {
        let current_count = self.read_count(commit_type)?;
        let new_count = current_count + 1;
        self.update_count(commit_type, new_count)?;
        Ok(new_count)
    }

    /// Format a commit message with the branch, type, count, and message
    pub fn format_commit_message(
        &self,
        commit_type: &CommitType,
        count: u32,
        message: &str,
    ) -> String {
        format!(
            "[{}] [{} {} : {}]",
            self.config.branch_name(),
            commit_type,
            count,
            message
        )
    }

    /// Create a commit with the given type and message
    pub fn create_commit(
        &self,
        commit_type: &CommitType,
        message: &str,
    ) -> Result<(), CommitCounterError> {
        let count = self.increment_count(commit_type)?;
        let formatted_message = self.format_commit_message(commit_type, count, message);
        self.git_ops.commit(&formatted_message)
    }

    /// Show all counters for the current branch
    pub fn show_counts(&self) -> Result<String, CommitCounterError> {
        let counts = self.read_all_counts()?;
        let mut output = format!("📌 Branch: {}\n\n", self.config.branch_name());

        // Show standard types first
        for commit_type in CommitType::standard_types() {
            let type_str = commit_type.to_string();
            let count = counts.get(&type_str).unwrap_or(&0);
            output.push_str(&format!("{:<6}: {}\n", type_str, count));
        }

        // Show custom types
        for (type_str, count) in counts.iter() {
            if !CommitType::standard_types()
                .iter()
                .any(|t| &t.to_string() == type_str)
            {
                output.push_str(&format!("{:<6}: {}\n", type_str, count));
            }
        }

        Ok(output)
    }

    /// Sync commit counts with the Git history
    pub fn sync_counts(&self) -> Result<(), CommitCounterError> {
        // Get the commit history using git log
        let output = Command::new("git")
            .args([
                "log",
                "--pretty=format:%s", // Only get the commit message
                "--first-parent",     // Follow only the first parent (simplify merges)
            ])
            .output()
            .map_err(|e| CommitCounterError::GitError(format!("Failed to read git log: {}", e)))?;

        if !output.status.success() {
            return Err(CommitCounterError::GitError(
                "Failed to read git history".to_string(),
            ));
        }

        // Parse commit messages to extract types and counts
        let mut counts: HashMap<String, u32> = HashMap::new();
        let messages = String::from_utf8_lossy(&output.stdout);
        let branch_name = self.config.branch_name();

        for message in messages.lines() {
            // Check if the message matches the expected format: [branch] [TYPE count : message]
            if let Some(captures) = regex::Regex::new(r"\[(.*?)\]\s*\[(.*?)\s*(\d+)\s*:\s*(.*)\]")
                .unwrap()
                .captures(message)
            {
                let msg_branch = captures.get(1).map(|m| m.as_str()).unwrap_or("");
                let type_str = captures.get(2).map(|m| m.as_str()).unwrap_or("");
                let count: u32 = captures
                    .get(3)
                    .map(|m| m.as_str())
                    .unwrap_or("0")
                    .parse()
                    .map_err(|e| {
                        CommitCounterError::ParseError(format!("Invalid count in message: {}", e))
                    })?;

                // Only process messages for the current branch
                if msg_branch == branch_name {
                    // Update the count if it's the highest for this type
                    let current_count = counts.get(type_str).copied().unwrap_or(0);
                    if count > current_count {
                        counts.insert(type_str.to_string(), count);
                    }
                }
            }
        }

        // Update the counter file with the new counts
        let file_path = self.config.counter_file_path();
        let mut file = File::create(file_path)?;
        for (type_str, count) in counts {
            writeln!(file, "{} {}", type_str, count)?;
        }

        Ok(())
    }
}
