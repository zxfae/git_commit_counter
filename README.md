Git Commit Counter

A Rust-based CLI tool to format, count, and synchronize Git commits based on their type (e.g., FEAT, FIX, DOCS) and branch. It helps developers maintain consistent commit messages and track commit counts per type.
Features

    Format commit messages with a standardized structure: [branch] [TYPE count : message].

    Count commits by type (e.g., FEAT, FIX, DOCS, REF, TEST,- Custom types supported.

    Synchronize commit counts with Git history.

    Branch-specific counters stored in ~/.git_commit_counts_<project>_<branch>.

    Display commit counts for the current branch.

    Error handling for invalid inputs and non-Git repositories.

Installation

    Prerequisites:

        Rust (version 1.86.0 or later).

        Git installed and configured.

    Clone the Repository:
    bash

    git clone https://github.com/zxfae/git_commit_counter.git
    cd git_commit_counter

    Build the Project:
    bash

    cargo build --release

    Install the Binary:
    bash

    cargo install --path git-commit-counter-bin --force

    Verify Installation:
    bash

    gm --help

    The binary (gm) is installed in ~/.cargo/bin/. Ensure this directory is in your PATH.

Usage

Run gm in a Git repository to manage commits. The tool supports three main commands:
1. Create a Commit

Create a formatted commit with a specific type and message.

Syntax:
bash

gm "TYPE : message"

Example:
bash

echo "New feature" > feature.txt
git add feature.txt
gm "FEAT : Add new feature"

Output:

    Creates a commit with message: [main] [FEAT 1 : Add new feature].

    Increments the FEAT counter for the current branch.

Supported Types:

    Standard: FEAT, FIX, DOCS, REF, TEST.

    Custom: Any other string (e.g., CUSTOM).

2. Show Commit Counts

Display commit counts for the current branch.

Syntax:
bash

gm show
# or
gm --show

Example:
bash

gm show

Output:
plain

📌 Branch: main

FEAT  : 4
FIX   : 1
DOCS  : 2
REF   : 0
TEST  : 1

3. Sync Commit Counts

Synchronize commit counts with the Git history by parsing commit messages.

Syntax:
bash

gm sync

Example:
bash

gm sync

Output:
plain

✅ Commit counts synchronized with Git history!

Details:

    Scans Git log for messages in the format [branch] [TYPE count : message].

    Updates the counter file (~/.git_commit_counts_<project>_<branch>) with the highest count for each type.

    Useful after resetting history or manual counter file changes.

Examples

    Create Multiple Commits:
    bash

    echo "Fix bug" > fix.txt
    git add fix.txt
    gm "FIX : Fix bug"
    echo "Update docs" > docs.txt
    git add docs.txt
    gm "DOCS : Update documentation"
    gm show

    Output:
    plain

    📌 Branch: main

    FEAT  : 0
    FIX   : 1
    DOCS  : 1
    REF   : 0
    TEST  : 0

    Sync After Reset:
    bash

    git reset --hard HEAD~2
    gm sync
    gm show

    Updates counts to reflect the current Git history.

Error Handling

    Invalid Commit Message:
    bash

    gm "FEAT Invalid message"

    Output: ❌ Commit message must be in the format "TYPE : message"

    Non-Git Repository:
    bash

    cd /tmp
    gm show

    Output: ❌ Error: Git error: Not inside a Git repository

    Corrupted Counter File: If the counter file contains invalid data, gm show reports a parse error, and gm sync can fix it.

Project Structure

    git-commit-counter-lib: Core library with commit counting logic.

    git-commit-counter-bin: CLI binary (gm) that uses the library.

    Counter File: Stored at ~/.git_commit_counts_<project>_<branch>.

Contributing

Contributions are welcome! Please:

    Fork the repository.

    Create a feature branch (git checkout -b feature/YourFeature).

    Commit changes (gm "FEAT : Add your feature" 😉).

    Push to the branch (git push origin feature/YourFeature).

    Open a pull request.

License

MIT License. See LICENSE for details.
Contact

Created by zxfae. Report issues or suggest features on the GitHub repository.

Last updated: April 26, 2025
