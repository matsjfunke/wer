# wer

Find who last edited any file or directory in a Git repository.

## Installation

```bash
# Build the project
cargo build --release

# Install globally
cargo install --path .
```

## Usage

```bash
# Check who last edited a file
wer Cargo.toml
# → Mats Julius Funke - 2025-06-07: inital commit

# Check who last edited a directory
wer src/
# → Mats Julius Funke - 2025-06-07: inital commit

# Check current directory
wer
# → Mats Julius Funke - 2025-06-07: inital commit
```

`wer` (German for "who") shows you who last modified a file or directory, replacing the need to remember complex `git log` commands.

```bash
# Instead of:
git log -1 --pretty=format:"%an - %ad: %s" --date=short filename

# Just use:
wer filename
```
