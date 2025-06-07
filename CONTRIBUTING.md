# Contributing to `wer`

Thank you for your interest in contributing to `wer`! This document provides guidelines and information for contributors.

## Submitting Ideas

- Open an issue to discuss new features before implementing
- Explain the use case and expected behavior
- Consider backward compatibility

- Keep changes focused and minimal
- Consider performance impact for large repositories

## Bug Reports

Include:

- Operating system and Rust version
- Operating System
- Expected vs actual behavior
- Repository structure if relevant
- Full error message

## Questions?

- Open an issue for questions about development
- Check existing issues and PRs first
- Be specific about what you're trying to achieve

## Implementation Guidelines

### Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- Git

### Local Development

1. **Fork and clone the repository**

   ```bash
   git clone https://github.com/yourusername/wer.git
   cd wer
   ```

2. **Build the project**

   ```bash
   cargo build
   ```

3. **Test locally**

   ```bash
   cargo run -- -v
   cargo run -- --help
   cargo run -- main.rs
   ```

### Project Structure

```
wer/
├── src/
│   ├── main.rs          # Entry point and CLI coordination
│   ├── cli.rs           # Command-line argument parsing
│   ├── git.rs           # Git operations (blame, commit info)
│   ├── utils.rs         # Utility functions (timestamps, path resolution)
│   └── syntax.rs        # Syntax highlighting
├── Cargo.toml           # Dependencies and metadata
├── README.md            # User documentation
└── CONTRIBUTING.md      # This file
```

## Code Guidelines

### Rust Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use descriptive variable and function names

### Error Handling

- Use `anyhow::Result<T>` for functions that can fail
- Provide helpful error messages with context
- Include actionable suggestions in error messages when possible

### Git Practices

- Write clear, descriptive commit messages
- Use conventional commit format: `feat:`, `fix:`, `docs:`, `refactor:`, etc.
- Keep commits focused and atomic
- Also include types in PR title (`feat:`, `fix:`, `docs:`, `refactor:`)
- **The PR title will become the commit message** (we use squash merge)

### Manual Testing

Test core functionality before submitting:

```bash
# Basic functionality
cargo run -- README.md
cargo run -- src/

# Blame mode
cargo run -- -b main.rs
cargo run -- -b -d git.rs
cargo run -- -b -m utils.rs

# Last contributors
cargo run -- -l 3 .
cargo run -- -l 5 src/

# Path resolution
cargo run -- main.rs        # Should find src/main.rs
cargo run -- nonexistent    # Should show helpful error

# Edge cases
cargo run -- --no-color -b main.rs
cargo run -- -d -m main.rs  # Should error (mutually exclusive)
```

### Documentation

- Update `README.md` for new features or changed behavior
- Update CLI help text in `src/cli.rs`
- Include practical examples in documentation
- Use clear, beginner-friendly language

Thank you for contributing to `wer`! 🎉
