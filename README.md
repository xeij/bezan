# Bezan by Xeij

**Bezan** is a simple Rust command-line tool that automates common Git commands: `git add .`, `git commit`, and `git push`. Instead of typing these commands manually, you can run `bezan` in your terminal to execute them all at once.

The commit message defaults to the current date in `MM/DD/YYYY` format (e.g., `06/26/2025`), or you can provide a custom message.

## Features

- Automates `git add .`, `git commit`, and `git push` with a single command.
- Defaults to the current date as the commit message.
- Supports custom commit messages via command-line arguments.
- Cross-platform: works on Windows, macOS, and Linux.
- Error handling for Git command failures.
- Open pull requests

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

### Default (Date as commit message)
```bash
bezan
```
Output:
`git commit -m "12/23/2025"`

### Custom Commit Message
```bash
bezan "My custom commit message"
```
Output:
`git commit -m "My custom commit message"`

The tool will stage all changes, commit (with date or custom message), and push to the remote repository.
Upon success, you will see a success message in the terminal.

Contributions are welcomed!
-fork the repo
-create feature branch
-commit changes (yes use bezan)


























