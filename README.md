Git Commit Counter
A Rust-based CLI tool to format, count, and synchronize Git commits based on their type (e.g., FEAT, FIX, DOCS) and branch. It helps developers maintain consistent commit messages and track commit counts per type.
Features

Format commit messages with a standardized structure: [branch] [TYPE count : message].
Count commits by type (e.g., FEAT, FIX, DOCS, REF, TEST).
Support for shorthand aliases (e.g., FE for FEAT, D for DOCS) that are automatically converted to full type names in commit messages and counter displays.
Custom commit types supported (any type not defined as standard or alias).
Synchronize commit counts with Git history, normalizing aliases to full type names.
Branch-specific counters stored in ~/.git_commit_counts_<project>_<branch>.
Display commit counts for the current branch with consistent formatting.
Error handling for invalid inputs and non-Git repositories.

Installation

Prerequisites:

Rust (version 1.86.0 or later).
Git installed and configured.


Clone the Repository:
git clone https://github.com/zxfae/git_commit_counter.git
cd git_commit_counter


Build the Project:
cargo build --release


Install the Binary:
cargo install --path git-commit-counter-bin --force


Verify Installation:
gm --help

The binary (gm) is installed in ~/.cargo/bin/. Ensure this directory is in your PATH.


Usage
Run gm in a Git repository to manage commits. The tool supports three main commands:
1. Create a Commit
Create a formatted commit with a specific type and message. Aliases (e.g., FE, D) are converted to full type names (e.g., FEAT, DOCS) in the commit message.
Syntax:
gm "TYPE : message"

Example:
echo "New feature" > feature.txt
git add feature.txt
gm "FE : Add new feature"

Output:

Creates a commit with message: [main] [FEAT 1 : Add new feature].
Increments the FEAT counter for the current branch.

Supported Types and Aliases:



Full Type
Aliases
Description



FEAT
FE, F
Feature


FIX
FI
Bug fix


DOCS
D
Documentation


REF
R
Refactor


TEST
T
Test


Custom
Any other string
Custom type


2. Show Commit Counts
Display commit counts for the current branch, using full type names.
Syntax:
gm show
# or
gm --show

Example:
gm show

Output:
📌 Branch: main

FEAT  : 4
FIX   : 1
DOCS  : 2
REF   : 0
TEST  : 1

3. Sync Commit Counts
Synchronize commit counts with the Git history by parsing commit messages, normalizing aliases to full type names.
Syntax:
gm sync

Example:
gm sync

Output:
✅ Commit counts synchronized with Git history!

Details:

Scans Git log for messages in the format [branch] [TYPE count : message].
Converts aliases (e.g., FE to FEAT, D to DOCS) when updating counts.
Updates the counter file (~/.git_commit_counts_<project>_<branch>) with the highest count for each type.
Useful after resetting history, fixing commit messages, or manual counter file changes.

Examples

Create Multiple Commits with Aliases:
echo "Fix bug" > fix.txt
git add fix.txt
gm "FI : Fix bug"
echo "Update docs" > docs.txt
git add docs.txt
gm "D : Update documentation"
gm show

Output:
📌 Branch: main

FEAT  : 0
FIX   : 1
DOCS  : 1
REF   : 0
TEST  : 0

Commit Log:
git log --pretty=%s

[main] [DOCS 1 : Update documentation]
[main] [FIX 1 : Fix bug]


Sync After Reset:
git reset --hard HEAD~2
gm sync
gm show

Updates counts to reflect the current Git history, normalizing any aliases found in commit messages.


Error Handling

Invalid Commit Message:
gm "FE Invalid message"

Output: ❌ Commit message must be in the format "TYPE : message"

Non-Git Repository:
cd /tmp
gm show

Output: ❌ Error: Git error: Not inside a Git repository

Corrupted Counter File:If the counter file contains invalid data, gm show reports a parse error, and gm sync can fix it by rebuilding counts from the Git history.


Fixing Historical Commits
If your Git history contains commits with aliases (e.g., [branch] [D 1 : message]), you can rewrite them to use full type names:
git rebase -i <commit-before-problematic-commits>

For each commit, change pick to edit, then amend the message:
git commit --amend

Replace the alias with the full type (e.g., D to DOCS). Continue the rebase:
git rebase --continue

After rewriting, synchronize counts:
gm sync

If the branch was pushed to a remote repository, force push the changes:
git push origin <branch> --force

Project Structure

git-commit-counter-lib: Core library with commit counting logic.
git-commit-counter-bin: CLI binary (gm) that uses the library.
Counter File: Stored at ~/.git_commit_counts_<project>_<branch>.

Contributing
Contributions are welcome! Please:

Fork the repository.
Create a feature branch (git checkout -b feature/YourFeature).
Commit changes using the tool (gm "F : Add your feature" 😉).
Push to the branch (git push origin feature/YourFeature).
Open a pull request.

License
MIT License. See LICENSE for details.
Contact
Created by zxfae. Report issues or suggest features on the GitHub repository.

Last updated: April 26, 2025
