//! Commit type definitions for Git Commit Counter

use std::fmt;

/// Types of commits
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CommitType {
    Feature,
    Fix,
    Documentation,
    Refactor,
    Test,
    Performance,
    Custom(String),
}

impl CommitType {
    /// Create a commit type from a string
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_uppercase().as_str() {
            "FEAT" | "FE" => CommitType::Feature,
            "FIXE" | "FI" => CommitType::Fix,
            "DOCS" | "D" => CommitType::Documentation,
            "REFA" | "R" => CommitType::Refactor,
            "TEST" | "T" => CommitType::Test,
            "PERF" | "P" => CommitType::Performance,
            custom => CommitType::Custom(custom.to_string()),
        }
    }

    /// Get all standard commit types
    pub fn standard_types() -> Vec<CommitType> {
        vec![
            CommitType::Feature,
            CommitType::Fix,
            CommitType::Documentation,
            CommitType::Refactor,
            CommitType::Test,
            CommitType::Performance,
        ]
    }
}

impl fmt::Display for CommitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CommitType::Feature => "FEAT",
            CommitType::Fix => "FIXE",
            CommitType::Documentation => "DOCS",
            CommitType::Refactor => "REFA",
            CommitType::Test => "TEST",
            CommitType::Performance => "PERF",
            CommitType::Custom(s) => s,
        };
        write!(f, "{}", s.to_uppercase())
    }
}
