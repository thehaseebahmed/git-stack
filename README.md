# git-stack
git-stack is a developer productivity tool that makes stacked pull requests simple, intuitive, and GitHub‑native. It streamlines the workflow of creating, managing, and merging dependent branches, so you can focus on writing code instead of wrestling with rebases and PR chains.

## Features

### 🌳 Stack Creation & Management

- **Smart Branch Creation** (`git-stack new`)
  - Create stacked branches with automatic naming (`feature-auth/1`, `feature-auth/2`, etc.)
  - Continue existing stacks or start new ones based on your current context
  - No more manual branch naming or index tracking

  ```bash
  # From main branch - start a new stack
  git-stack new auth
  # Creates "auth/1"

  # From stack branch - continue the stack
  git-stack new
  # Creates "auth/2" (if on auth/1)
  ```

- **Visual Stack Overview** (`git-stack list`)
  - See all your stacks at a glance in a clean tree format
  - Understand branch relationships and stack structure instantly
  - Track progress across multiple feature development streams

  ```bash
  git-stack list
  ```
  ```
  feature-auth
  ├─ feature-auth/1
  ├─ feature-auth/2
  └─ feature-auth/3

  ui-redesign
  └─ ui-redesign/1
  ```

### 🔄 Stack Synchronization

- **One-Command Stack Sync** (`git-stack sync`)
  - Keep entire stacks up-to-date with latest main branch
  - Automatically rebase all dependent branches when you change a parent
  - Context-aware: sync all stacks or just your current one

  ```bash
  # From main - sync all stacks
  git-stack sync
  ```
  ```
  🔄 Starting sync for all stacks...
  1. Fetching from remote...
  2. Syncing 2 stack(s):

  📦 Syncing stack: feature-auth
    ✓ Stack rebased successfully
  ✅ All stacks synchronized successfully!
  ```

- **Remote Integration**
  - Pull latest changes from remote for all stack branches
  - Handle mixed local/remote branch scenarios seamlessly
  - Works with or without configured git remotes

### 🚀 GitHub Workflow

- **Automated Pull Request Creation** (`git-stack review`)
  - Create PRs for entire stacks with proper dependency chains
  - Automatic PR titles and descriptions with stack context
  - Smart detection of existing PRs to avoid duplicates
  - Set up proper PR relationships so reviewers understand dependencies

  ```bash
  # From any stack branch
  git-stack review
  ```
  ```
  🔄 Creating pull requests for stack: feature-auth
  📦 Found 3 branch(es) in stack:
    - feature-auth/1
    - feature-auth/2
    - feature-auth/3
  🚀 Creating missing pull requests...
    ✓ Created PR #101 for feature-auth/1
    ✓ Created PR #102 for feature-auth/2 (Depends on #101)
  ✅ Review summary:
    feature-auth/1 -> PR #101
    feature-auth/2 -> PR #102
  ```

- **GitHub CLI Integration**
  - Leverages GitHub CLI for reliable PR operations
  - Handles authentication and repository validation automatically

### 🎯 Developer Experience

- **Context-Aware Commands**
  - Commands adapt behavior based on whether you're on main, a stack branch, or other branches
  - Helpful error messages that guide you to the right action
  - Prevents common mistakes like creating new stacks from the wrong branch

- **Git Workflow Compatible**
  - Use regular git commands (`git commit --amend`, `git rebase -i`) alongside git-stack
  - `git-stack sync` handles propagating changes up the stack automatically
  - No lock-in - your branches are just regular git branches

### 🛣️ Roadmap

- **Interactive Stack Rebase** (`git-stack rebase`)
  - Simplified interactive rebase that only shows commits from your current stack
  - No more getting overwhelmed by long git history or accidentally modifying main commits
  - Stack-aware commit reordering, squashing, and editing

- **Automated Landing** (`git-stack land`)
  - Merge approved PRs sequentially with automatic stack updates
  - Check approval status and merge bottom-most diff automatically
  - Rebase remaining stack onto updated main after each merge
  - Handle the entire "merge-and-rebase dance" in one command

- **Enhanced Stack Visibility**
  - Show PR review status directly in `git-stack list` (Pending, Approved, Changes Requested)
  - Display which branches have been pushed to remote
  - Indicate merge conflicts or rebase issues in stack overview

- **Advanced PR Management**
  - Automatic base branch updates when parent PRs are merged
  - Smart handling of stack reordering and dependency chain updates
  - Bulk operations for updating PR descriptions and dependencies

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

## Help and Version

Show help information:
```bash
git-stack --help
git-stack new --help
git-stack list --help
git-stack sync --help
git-stack review --help
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
├── git_integration_tests.rs # Git-specific integration tests
├── list_command_tests.rs   # List command specific tests
├── context_tests.rs        # Context-aware behavior tests
├── sync_tests.rs           # Sync command tests
└── comprehensive_tests.rs  # Comprehensive edge case tests
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

# List command tests
cargo test --test list_command_tests

# Sync command tests
cargo test --test sync_tests

# Context-aware behavior tests
cargo test --test context_tests

# Comprehensive edge case tests
cargo test --test comprehensive_tests
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
use git_stack::{commands, RealGitRunner};

let git_runner = RealGitRunner;

// Create a new branch
match commands::new_branch_contextual(&git_runner, Some("my-feature")) {
    Ok(branch_name) => println!("Created: {}", branch_name),
    Err(e) => eprintln!("Error: {}", e),
}

// List stacks
match commands::list_stacks(&git_runner) {
    Ok(()) => println!("Stacks listed successfully"),
    Err(e) => eprintln!("Error: {}", e),
}

// Sync stacks
match commands::sync_stacks(&git_runner) {
    Ok(()) => println!("Sync completed successfully"),
    Err(e) => eprintln!("Error: {}", e),
}
```
