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
wer -b src/main.rs
# → 61fcdda (Mats Julius Fun - 07 Jun) |  1 | use anyhow::Result;
# → 6b70ffb (Mats Julius Fun - 07 Jun) |  2 | use clap::Parser;
```

### Display Options

```bash
# Show only dates
wer -d src/main.rs
# → 07 Jun 2025

wer -b -d src/main.rs  # Blame with dates only
# → 07 Jun |  1 | use anyhow::Result;
# → 07 Jun |  2 | use clap::Parser;

# Show commit messages on separate lines
wer -m src/main.rs
# → 61fcdda Mats Julius Funke - 07 Jun 2025
#     Initial commit

wer -b -m src/main.rs  # Blame with commit messages
# → 61fcdda (Mats Julius Fun - 07 Jun) |  1 | use anyhow::Result;
#     Initial commit
```

### Top Contributors

Find the last N unique people who touched a file or directory:

```bash
# Show last 3 contributors
wer --top 5 src/
# → a1b2c3d George Boole - 1854: feat: introduce Boolean algebra and logical foundations
# → e4f5g6h Alan Turing - 30 Nov 1936: feat: develop theoretical computing foundations
# → i7j8k9l Claude Shannon - Jul 1948: feat: establish information theory and digital communication
# → m0n1o2p Steve Wozniak - Jul 1976: feat: launch personal computing revolution
# Searched for 5 but only 4 contributed  # (if fewer found)
```

### Color Control

```bash
# Disable colors and syntax highlighting
wer --no-color -b src/main.rs
```

## All Flags

| Flag                   | Description                                       |
| ---------------------- | ------------------------------------------------- |
| `-b, --blame`          | Show git blame for files with syntax highlighting |
| `-d, --date-only`      | Show dates only (mutually exclusive with -m)      |
| `-m, --commit-message` | Show commit messages on next line                 |
| `--top N`              | Show last N contributors (normal mode only)       |
| `--no-color`           | Disable colors and syntax highlighting            |

## Features

- **Syntax Highlighting**: Automatic language detection for 100+ file types in blame mode
- **Smart Error Messages**: Helpful suggestions for common issues
- **Git Integration**: Works with any git repository
- **Multiple Display Modes**: Choose between full info, dates only, or commit messages
- **Color Support**: Beautiful terminal colors with option to disable
- **Top Contributors**: Find who has been working on specific files/directories
