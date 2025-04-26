# VGit Commit Counter 🚀

A simple Rust CLI tool to format, count, and manage Git commits by type (e.g., FEAT, FIX, DOCS). Keep your commit history clean and track your progress!

## ✨ Features

- 📝 Formats commits as `[branch] [TYPE count : message]`
- 🔢 Counts commits by type (e.g., FEAT: 3, FIX: 1)
- 🏷️ Supports aliases (e.g., FE → FEAT, D → DOCS)
- 🔄 Syncs counts with Git history
- 🧹 Resets counts for a fresh start
- 🛠️ Custom types allowed
- 📂 Stores counts per branch in `~/.git_commit_counts_<project>_<branch>`

## 🛠️ Installation

### Prerequisites

- 🦀 Rust (1.86.0 or later)
- 📚 Git

### Steps

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
FIX   : 1
DOCS  : 0
REF   : 0
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

## 🏷️ Supported Types & Aliases

| Type | Aliases | Description |
|------|---------|-------------|
| FEAT | FE      | Feature     |
| FIX  | FI      | Bug fix     |
| DOCS | D       | Documentation |
| REF  | R       | Refactor    |
| TEST | T       | Test        |
| Custom | Any string | Custom type |

Aliases are converted to full type names (e.g., FE → FEAT) in commits and displays.

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
FIX   : 1
DOCS  : 1
REF   : 0
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
FIX   : 0
DOCS  : 0
REF   : 0
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

*Last updated: April 26, 2025*
