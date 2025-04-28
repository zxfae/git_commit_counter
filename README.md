# Git Commit Counter
![image](./DOCS/file.png)
A simple Rust CLI tool to format, count, and manage Git commits by type (e.g., FEAT, FIX, DOCS, TEST..). Keep your commit history clean and track your progress!

## Features

- 📝 Formats commits as `[branch] [TYPE count : message]`
- 🔢 Counts commits by type (e.g., FEAT: 3, FIXE: 1)
- 🏷️ Supports aliases (e.g., FE → FEAT, D → DOCS)
- 🔄 Syncs counts with Git history
- 🧹 Resets counts for a fresh start
- 🛠️ Custom types allowed
- 📂 Stores counts per branch in `~/.git_commit_counts_<project>_<branch>`

## 🛠️ Installation 🛠️

### Prerequisites

- 🦀 Rust (>=1.86.0 )
- 📚 Git

### Steps
(Ensure you don't have the alias gm on your .bashrc or .zshrc file
alias gm. If present before theses steps write 'unalias gm')

1. Clone the repo:
```bash
git clone https://github.com/zxfae/git_commit_counter.git
cd git_commit_counter
```

2. Build the project:
```bash
cargo build --release
```

3. Install the gm binary:
```bash
cargo install --path git-commit-counter-bin --force
```

4. Verify:
```bash
gm --help
```

**Note:** Ensure `~/.cargo/bin` is in your PATH.
⚠️ IMPORTANT: If you're starting with a newly created repository,
you MUST make an initial commit before using `gm`:
```bash
git commit -m "Initial commit"
```
This is required because `gm` works with existing Git repositories
that have at least one commit. Without an initial commit, you might encounter
errors when trying to use `gm` with local/remote repositories.
⚠️

## 🏷️ Supported Types & Aliases

| Type | Aliases | Description |
|------|---------|-------------|
| FEAT | FE      | Feature     |
| FIXE  | FI      | Bug fix     |
| DOCS | D       | Documentation |
| REFA  | R       | Refactor    |
| TEST | T       | Test        |
| Custom | Any string | Custom type |

Aliases are converted to full type names (e.g., FE → FEAT) in commits and displays.

## Good practices
| Command | Purpose | Practice |
|------|---------|-------------|
| gm "TYPE : msg" | Create a formatted commit      | Use clear, single purpose message     |
| gm show  | Display commit counts by type      | Review progress and balance work type     |
| gm sync | Sync counts with Git history       | Run after making manual Git changes |
| gm reset  | reset commit counts for current branch       | Use after major refactors or resets    |


### Storage Location for Counter Files

Git Commit Counter stores commit counter files in locations that comply with platform-specific configuration standards:

_   Linux/macOS:

        In $XDG_CONFIG_HOME/git_commit_counter/<project>_<branch>.counts if $XDG_CONFIG_HOME is set.

        Otherwise, in ~/.config/git_commit_counter/<project>_<branch>.counts, following the XDG Base Directory Specification.

_   Windows:

        In %APPDATA%\git_commit_counter\<project>_<branch>.counts (e.g., C:\Users\<Username>\AppData\Roaming\git_commit_counter\).

No files are created in the Git repository or directly in the user's home directory, ensuring a clean and organized environment.

## 📚 Usage

Run `gm` in a Git repository to manage commits. Available commands:

### 1. Create a Commit

Format: `gm "TYPE : message"`

```bash
echo "New feature" > feature.txt
git add feature.txt
gm "FE : Add cool feature"
```

Output: `Commit message [main] [FEAT 1 : Add cool feature]`




### 2. Show Commit Counts

```bash
gm show
```

Output:
```
📌 Branch: main
FEAT  : 2
FIXE   : 1
DOCS  : 0
REFA   : 0
TEST  : 0
```

### 3. Sync Counts

Update counts based on Git history:

```bash
gm sync
```

Output: `✅ Commit counts synchronized with Git history!`

### 4. Reset Counts

Clear counts for the current branch:

```bash
gm reset
```

Output: `✅ Commit counts reset`


## 🌟 Examples

### Create Commits

```bash
echo "Fix bug" > bugfix.txt
git add bugfix.txt
gm "FI : Fix login bug"
```
Commit: `[main] [FIX 1 : Fix login bug]`

```bash
echo "Docs" > README.md
git add README.md
gm "D : Update README"
```
Commit: `[main] [DOCS 1 : Update README]`

### Check Counts

```bash
gm show
```

Output:
```
📌 Branch: main
FEAT  : 0
FIXE   : 1
DOCS  : 1
REFA   : 0
TEST  : 0
```

### Reset and Restart

```bash
gm reset
gm show
```

Output:
```
📌 Branch: main
FEAT  : 0
FIXE   : 0
DOCS  : 0
REFA   : 0
TEST  : 0
```

## 🚨 Common Errors

| Error | Solution |
|-------|----------|
| Invalid message:<br>`gm "FE Invalid"` | Use TYPE : message (e.g., `FE : Add feature`) |
| Not a Git repo:<br>`cd /tmp`<br>`gm show` | Run `gm` in a Git repository |
| Counts not updating | Run `gm sync` to refresh counts |

## 🤝 Contributing

Want to help? Awesome! 😄

1. Fork the repo
2. Create a branch: `git checkout -b feature/cool-idea`
3. Commit with gm: `gm "F : Add cool idea"`
4. Push: `git push origin feature/cool-idea`
5. Open a pull request

## 📄 License

MIT License. See LICENSE for details.

## 📬 Contact

Created by zxfae. Issues or ideas? Visit the GitHub repo.

*Last updated: April 28, 2025*
