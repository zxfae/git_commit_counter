🚀 Git Commit Counter

Git Commit Counter is a Rust-based CLI tool for formatting, counting, and synchronizing Git commits based on their type (e.g., FEAT, FIX, DOCS) and branch.

It helps developers:

    Maintain consistent commit messages 🛠️

    Track commit counts per type 📊

    Sync counts with Git history 🔄

✨ Features

✅ Format commit messages with a standardized structure:

    [branch] [TYPE count : message]

✅ Count commits by type (e.g., FEAT, FIX, DOCS, REF, TEST, and custom types)

✅ Synchronize commit counts with Git history (git log parsing)

✅ Branch-specific counters stored locally:

    ~/.git_commit_counts_<project>_<branch>

✅ Display commit counts for the current branch

✅ Robust error handling for:

    Invalid commit messages

    Non-Git repositories

    Corrupted counter files

⚙️ Installation
Prerequisites

    🦀 Rust (version 1.86.0 or later)

    🔧 Git installed and configured

Setup Steps

    Clone the repository:

git clone https://github.com/zxfae/git_commit_counter.git
cd git_commit_counter

Build the project:

cargo build --release

Install the binary:

cargo install --path git-commit-counter-bin --force

Verify installation:

    gm --help

👉 The binary (gm) is installed to ~/.cargo/bin/.
Make sure this directory is in your PATH.
🧑‍💻 Usage

Run gm inside any Git repository to manage commits easily.
1. Create a Commit

Create a formatted commit with a specific type and message.

Syntax:

gm "TYPE : message"

Example:

echo "New feature" > feature.txt
git add feature.txt
gm "FEAT : Add new feature"

Result:

    Creates commit:
    [main] [FEAT 1 : Add new feature]

    Increments the FEAT counter

Supported Types:

    Standard: FEAT, FIX, DOCS, REF, TEST

    Custom: Any string you choose

2. Show Commit Counts

Display commit statistics for the current branch.

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

Synchronize counters with Git commit history.

Syntax:

gm sync

Example:

gm sync

Result:

    Parses git log for commit patterns

    Updates local counter file with correct counts

    Useful after history resets or manual edits

🔥 Examples
Create Multiple Commits

echo "Fix bug" > fix.txt
git add fix.txt
gm "FIX : Fix bug"

echo "Update docs" > docs.txt
git add docs.txt
gm "DOCS : Update documentation"

gm show

Output:

📌 Branch: main

FEAT  : 0
FIX   : 1
DOCS  : 1
REF   : 0
TEST  : 0

Sync After History Reset

git reset --hard HEAD~2
gm sync
gm show

    Counters reflect the corrected history after reset!

🚨 Error Handling
Situation	Result
❌ Invalid message format	Error: Must be "TYPE : message"
❌ Outside Git repo	Error: "Not inside a Git repository"
⚠️ Corrupted counter file	Error when showing, auto-fixed by gm sync
🏗️ Project Structure

git-commit-counter-lib/  ➔ Core library (logic)
git-commit-counter-bin/  ➔ CLI binary (gm)
~/.git_commit_counts_<project>_<branch> ➔ Local counter file

🤝 Contributing

Contributions are welcome! 🎉

    Fork the repository

    Create a new feature branch:

git checkout -b feature/YourFeature

Commit your changes using:

    gm "FEAT : Add your feature" 😉

    Push to GitHub and open a Pull Request.

📜 License

This project is licensed under the MIT License.
See LICENSE file for details.
📬 Contact

Created by zxfae.
Report issues or suggest features via GitHub Issues.

Last updated: April 26, 2025
