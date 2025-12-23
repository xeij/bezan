# Bezan by Xeij

**Bezan** is a simple Rust command-line tool that automates common Git commands: `git add .`, `git commit`, and `git push`. Instead of typing these commands manually, you can run `bezan` in your terminal to execute them all at once.

The commit message defaults to the current date in `MM/DD/YYYY` format (e.g., `06/26/2025`), or you can provide a custom message.

## Features

- Automates `git add .`, `git commit`, and `git push` with a single command.
- Defaults to the current date as the commit message.
- Supports custom commit messages via command-line arguments.
- Cross-platform: works on Windows, macOS, and Linux.
- Error handling for Git command failures.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed to build the tool.
- [Git](https://git-scm.com/downloads) installed and configured with user details (`user.name` and `user.email`).
- Push access to a remote Git repository.

## Installation

```bash
git clone https://github.com/<your-username>/bezan.git
cd bezan
cargo build --release
```

## To run bezan from any terminal move the binary to a directory in your PATH

On Linux/macOS:
```bash
sudo cp target/release/bezan /usr/local/bin/
```

On Windows:
```bash
mkdir "C:\Program Files\bezan"
copy target\release\bezan.exe "C:\Program Files\bezan"
```

Add the directory to your PATH:
1. Open System Properties → Environment Variables.
2. Edit the `Path` variable and add `C:\Program Files\bezan`.

# Usage

Navigate to your git repository and make changes.

### Default (All files, date as commit message)
```bash
bezan
```
Commits all changes with today's date.

### Custom Commit Message (All files)
```bash
bezan "My custom commit message"
```
Commits all changes with a custom message.

### Single File (Date as commit message)
```bash
bezan testfile.py
```
Commits only `testfile.py` with today's date.

### Single File (Custom message)
```bash
bezan testfile.py "Fixed bug in test"
```
Commits only `testfile.py` with a custom message.

### Multiple Files (Date as commit message)
```bash
bezan file1.py file2.js
```
Commits only the specified files with today's date.

### Multiple Files (Custom message)
```bash
bezan file1.py file2.js "Updated multiple files"
```
Commits only the specified files with a custom message.

The tool will stage the specified changes, commit, and push to the remote repository.
Upon success, you will see a success message in the terminal.

Contributions are welcomed!
-fork the repo
-create feature branch
-commit changes (yes use bezan)
-open pull request


























