# git-stack
git-stack is a developer productivity tool that makes stacked pull requests simple, intuitive, and GitHub‑native. It streamlines the workflow of creating, managing, and merging dependent branches, so you can focus on writing code instead of wrestling with rebases and PR chains.

## Features

- **Stack Branch Creation**: Create new branches that follow a structured naming pattern (`feature-name/1`, `feature-name/2`, etc.)
- **Git Integration**: Seamlessly integrates with your existing git workflow
- **Error Handling**: Clear error messages and proper exit codes

## Building from Source

### Prerequisites
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- Git (must be installed and available in PATH)

### Build Instructions

Build the project:
```bash
cargo build
```

Run tests:
```bash
cargo test
```

Build optimized release version:
```bash
cargo build --release
```

Run linting:
```bash
cargo clippy
```

Format code:
```bash
cargo fmt
```

## CLI Usage

### Creating New Branches

Create a new stacked branch for a feature:
```bash
git-stack new <feature-name>
```

Examples:
```bash
# Creates branch "auth/1" from current branch
git-stack new auth

# If auth/1 already exists, creates "auth/2"
git-stack new auth

# Creates branch "ui-redesign/1"
git-stack new ui-redesign
```

### Help and Version

Show help information:
```bash
git-stack --help
git-stack new --help
```

Show version:
```bash
git-stack --version
```

## Requirements

- Must be run from within a git repository
- Feature names can only contain alphanumeric characters, hyphens, and underscores
- Git must be available in your system PATH

## Error Handling

The tool provides clear error messages for common issues:
- Running outside a git repository
- Invalid feature names (containing spaces or special characters)
- Git command failures
- Missing arguments

## Development

### Project Structure

```
src/
├── lib.rs          # Core library functionality
├── main.rs         # CLI entry point
tests/
├── unit_tests.rs           # Unit tests for library functions
├── integration_tests.rs    # CLI integration tests
└── git_integration_tests.rs # Git-specific integration tests
```

The project follows Rust best practices with a clear separation between:
- **Library code** (`src/lib.rs`) - Reusable functionality organized in modules
- **Binary code** (`src/main.rs`) - Minimal CLI parsing and orchestration
- **Unit tests** (`tests/unit_tests.rs`) - Test individual functions
- **Integration tests** (`tests/integration_tests.rs`) - Test CLI behavior end-to-end

### Running Tests

Run all tests:
```bash
cargo test
```

Run specific test files:
```bash
# Unit tests
cargo test --test unit_tests

# Integration tests  
cargo test --test integration_tests

# Git integration tests
cargo test --test git_integration_tests
```

### Code Quality
```bash
# Run linting
cargo clippy

# Format code
cargo fmt

# Run all checks
cargo test && cargo clippy && cargo fmt --check
```

### Library Usage

The core functionality is available as a library:

```rust
use git_stack::commands;

// Create a new branch
match commands::new_branch("my-feature") {
    Ok(branch_name) => println!("Created: {}", branch_name),
    Err(e) => eprintln!("Error: {}", e),
}
```
