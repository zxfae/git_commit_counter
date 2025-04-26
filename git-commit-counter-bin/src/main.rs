//! Git Commit Counter CLI tool

use clap::{Parser, Subcommand};
use git_commit_counter_lib::{CommitCounter, CommitType, RealGitOps};
use std::process::exit;

/// Git Commit Counter - A tool to format and count git commits
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,

    /// Commit message in the format "TYPE : message"
    #[clap(value_parser)]
    message: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show commit counts for the current branch
    Show,
    /// Sync commit counts with the Git history
    Sync,
}

fn main() {
    let args = Args::parse();

    // Create a counter with real Git operations
    let git_ops = Box::new(RealGitOps);
    let counter = match CommitCounter::new(git_ops) {
        Ok(counter) => counter,
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            exit(1);
        }
    };

    // Handle commands
    match &args.command {
        Some(Commands::Show) => match counter.show_counts() {
            Ok(output) => {
                println!("{}", output);
            }
            Err(e) => {
                eprintln!("❌ Error displaying counts: {}", e);
                exit(1);
            }
        },
        Some(Commands::Sync) => match counter.sync_counts() {
            // Add this arm
            Ok(()) => {
                println!("✅ Commit counts synchronized with Git history!");
            }
            Err(e) => {
                eprintln!("❌ Error syncing counts: {}", e);
                exit(1);
            }
        },
        None => {
            // Handle commit message
            let message = match args.message {
                Some(msg) => msg,
                None => {
                    eprintln!("❌ USAGE:\n  gm \"TYPE : msg\"\n  gm show\n  gm sync");
                    exit(1);
                }
            };

            // Parse commit message
            if !message.contains(':') {
                eprintln!("❌ Commit message must be in the format \"TYPE : message\"");
                exit(1);
            }

            let parts: Vec<&str> = message.splitn(2, ':').collect();
            let commit_type = CommitType::from_str(parts[0]);
            let msg = parts[1].trim();

            if let Err(e) = counter.create_commit(&commit_type, msg) {
                eprintln!("❌ Commit failed: {}", e);
                exit(1);
            }

            println!("✅ Commit successful!");
        }
    }
}
