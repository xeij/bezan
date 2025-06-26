# Bezan by Xeij

**Bezan** is a simple Rust command-line tool that automates common Git commands: `git add .`, `git commit -m "<current date>"`, and `git push`. Instead of typing these commands manually, you can run `bezan` in your terminal to execute them all at once. The commit message uses the current date in `MM/DD/YYYY` format (e.g., `06/26/2025`).

## Features

- Automates `git add .`, `git commit`, and `git push` with a single command.
- Uses the current date as the commit message.
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

## To run bezan from any terminal move the binary to a directory in your PATH

On Linux/macOS:
sudo cp target/release/bezan /usr/local/bin/

On Windows:
mkdir "C:\Program Files\bezan"
copy target\release\bezan.exe "C:\Program Files\bezan"

Add the directory to your PATH:
Open System Properties → Environment Variables.
Edit the Path variable and add C:\Program Files\bezan

# Usage

cd/path/to/your/repo
make changes
run bezan

The tool will stage all chages, comit with current date, push to remote repo
Upon success, you will see message in terminal.

Contributions are welcomed!
-fork the repo
-create feature branch
-commit changes (yes use bezan)
-open pull request


























