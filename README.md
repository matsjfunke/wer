# wer

Find who last edited any file or directory in a Git repository.
`wer` (German for "who") shows you who last modified a file or directory, replacing the need to remember complex `git log` commands.

## Installation

```bash
# Build the project
cargo build --release

# Install globally
cargo install --path .
```

## Usage

### Smart Path Resolution ✨

`wer` automatically finds files and directories by name - no need to remember exact paths!

```bash
# Just type the filename - wer finds it automatically
wer main.rs                 # Finds src/main.rs
wer git.rs                  # Finds src/git.rs
wer Cargo.toml             # Finds ./Cargo.toml

# Works with directories too
wer src/                   # Works from anywhere in the repository

# For absolute paths, use full paths to skip search
wer ~/Documents/file.txt   # Uses absolute path directly
wer /full/path/to/file     # No search, direct access
```

### Basic Usage

```bash
# Check who last edited a file
wer Cargo.toml
# → 61fcdda Mats Julius Funke - 07 Jun 2025: Initial commit

# Check who last edited a directory
wer src/
# → 61fcdda Mats Julius Funke - 07 Jun 2025: Added new module

# Check current directory
wer
# → 61fcdda Mats Julius Funke - 07 Jun 2025: Latest changes
```

### Blame Mode

Show git blame with syntax highlighting for any file:

```bash
# Show blame with full commit info and syntax highlighting
wer -b main.rs              # Automatically finds src/main.rs
# → 61fcdda (Mats Julius Fun - 07 Jun) |  1 | use anyhow::Result;
# → 6b70ffb (Mats Julius Fun - 07 Jun) |  2 | use clap::Parser;
```

### Display Options

```bash
# Show only dates
wer -d main.rs
# → 07 Jun 2025

wer -b -d main.rs          # Blame with dates only
# → 07 Jun |  1 | use anyhow::Result;
# → 07 Jun |  2 | use clap::Parser;

# Show commit messages on separate lines
wer -m main.rs
# → 61fcdda Mats Julius Funke - 07 Jun 2025
#     Initial commit

wer -b -m main.rs          # Blame with commit messages
# → 61fcdda (Mats Julius Fun - 07 Jun) |  1 | use anyhow::Result;
#     Initial commit
```

### Last Contributors

Find the last N unique people who touched a file or directory:

```bash
# Show last 5 contributors
wer -l 5 src/
# → a1b2c3d George Boole - 1854: feat: introduce Boolean algebra and logical foundations
# → e4f5g6h Alan Turing - 30 Nov 1936: feat: develop theoretical computing foundations
# → i7j8k9l Claude Shannon - Jul 1948: feat: establish information theory and digital communication
# → m0n1o2p Steve Wozniak - Jul 1976: feat: launch personal computing revolution
# Searched for 5 but only 4 contributed  # (if fewer found)
```

### Color Control

```bash
# Disable colors and syntax highlighting
wer --no-color -b main.rs
```

## All Flags

| Flag                   | Description                                       |
| ---------------------- | ------------------------------------------------- |
| `-b, --blame`          | Show git blame for files with syntax highlighting |
| `-d, --date-only`      | Show dates only (mutually exclusive with -m)      |
| `-m, --commit-message` | Show commit messages on next line                 |
| `-l, --last N`         | Show last N contributors (normal mode only)       |
| `--no-color`           | Disable colors and syntax highlighting            |
| `-v, --version`        | Print version information                         |
| `-h, --help`           | Show help information                             |

## Features

- **Smart Path Resolution**: Automatically finds files and directories by name
- **Syntax Highlighting**: Automatic language detection for 100+ file types in blame mode
- **Smart Error Messages**: Helpful suggestions for common issues
- **Git Integration**: Works with any git repository
- **Multiple Display Modes**: Choose between full info, dates only, or commit messages
- **Color Support**: Beautiful terminal colors with option to disable
- **Last Contributors**: Find who has been working on specific files/directories
