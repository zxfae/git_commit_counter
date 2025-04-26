Git Commit Counter
A Rust-based CLI tool to format Git commit messages and track commit counts by type for each branch in a Git repository.
Features

Format commit messages with a standardized structure: [branch] [TYPE count : message].
Track and increment commit counts per commit type (e.g., FEAT, FIX, DOCS) on a per-branch basis.
Support for standard commit types (FEAT, FIX, DOCS, REF, TEST) and custom types.
Display commit counts for the current branch with the show command.

Installation
Prerequisites

Rust (install via rustup).
Git installed and configured.
A Git repository to work in.

Steps

Clone the repository or navigate to the project directory:
cd path/to/git-commit-counter


Build the project:
cargo build --release


(Optional) Install the gm command globally:
cargo install --path git-commit-counter-bin

Ensure ~/.cargo/bin is in your $PATH:
export PATH="$HOME/.cargo/bin:$PATH"



Usage
The tool is invoked using the gm command in a Git repository.
Create a Formatted Commit
To create a commit with a formatted message and increment the counter for the specified type:
gm "TYPE : message"


TYPE: One of FEAT, FIX, DOCS, REF, TEST, or a custom type.
Example:gm "FEAT : add user authentication"


Output commit message: [main] [FEAT 1 : add user authentication]

Show Commit Counts
To display the number of commits by type for the current branch:
gm show


Example output:📌 Branch: main

FEAT  : 3
FIX   : 1
DOCS  : 0
REF   : 2
TEST  : 0
CUSTOM: 1



Notes

Ensure you are in a Git repository, or the tool will return an error.
The commit message must follow the TYPE : message format.
Commit counts are stored in ~/.git_commit_counts_<project>_<branch>.

Project Structure

git-commit-counter-lib: Library crate containing the core logic (commit types, counters, Git operations).
git-commit-counter-bin: Binary crate providing the gm CLI tool.
Workspace: Combines both crates for easy development.

Contributing
Contributions are welcome! Please open an issue or submit a pull request with your changes.
License
This project is licensed under the MIT License.
Author
Philippe Lecrosnier (alias ZxFae33) lecro.philippe@icloud.com
