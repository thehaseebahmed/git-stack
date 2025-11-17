# git-stack

> **🧪 Experimental Project**: While there are established diff-stacking solutions available, git-stack represents an experiment in developing a production-ready CLI tool using modern AI-assisted development practices. This project was built collaboratively using OpenCode, GitHub Copilot, and OpenSpec with Specification-Driven Development (SDD) methodologies.

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
  - **NEW**: View PR numbers and status directly in the terminal with color-coded indicators
  - Understand branch relationships and stack structure instantly
  - Track progress across multiple feature development streams

  ```bash
  git-stack list
  ```
  ```
  auth-system
  ├─ auth-system/1 #432 (merged)            [displayed in green]
  ├─ auth-system/2 #445 (open)              [default color]
  ├─ auth-system/3 #456 (changes requested) [displayed in yellow]
  └─ auth-system/4 #459 (draft)             [displayed in gray]

  payment-flow
  ├─ payment-flow/1 #401 (merged)           [displayed in green]
  └─ payment-flow/2                         [no PR yet - default color]
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
  - Modern CLI output with animated progress indicators and clear visual hierarchy

  ```bash
  # From any stack branch
  git-stack review
  ```
  ```
  ┌  Creating PRs for stack: feature-auth
  │
  ◇  Found 3 diff(s) in stack
  │
  ◆  Created missing pull requests
  │  ✓ Created PR #101 for feature-auth/1
  │  ✓ Created PR #102 for feature-auth/2
  │  ✓ Created PR #103 for feature-auth/3
  │
  └  All done!
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

- **Advanced PR Management**
  - Automatic base branch updates when parent PRs are merged
  - Smart handling of stack reordering and dependency chain updates
  - Bulk operations for updating PR descriptions and dependencies

## Installation

### Building from Source

#### Prerequisites
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- Git (must be installed and available in PATH)
- GitHub CLI (install from [cli.github.com](https://cli.github.com/)) - required for `git-stack review`

#### Build Instructions

```bash
# Clone the repository
git clone <repository-url>
cd git-stack

# Build the project
cargo build --release

# The binary will be available at target/release/git-stack
# Add it to your PATH or copy to a directory in your PATH
```

## Getting Started

### Quick Start

```bash
# Start a new feature stack
git-stack new auth

# Continue working on the stack
git-stack new

# View your stacks
git-stack list

# Sync with remote
git-stack sync

# Create PRs for review
git-stack review
```

## Requirements

- Must be run from within a git repository
- Feature names can only contain alphanumeric characters, hyphens, and underscores
- Git must be available in your system PATH
- GitHub CLI required for PR operations (`git-stack review`) and enhanced list view with PR information
  - If GitHub CLI is not available or authentication fails, `git-stack list` gracefully falls back to standard tree display

---

# Development & Contributions

This section is for developers who want to contribute to git-stack or understand its internals.

## Project Philosophy

git-stack was developed as an experiment in modern AI-assisted development, utilizing:
- **OpenCode** for development environment and tooling
- **GitHub Copilot** for code generation and assistance
- **OpenSpec** for specification-driven development
- **SDD (Specification-Driven Development)** methodology

The goal was to demonstrate that production-ready CLI tools can be built effectively using AI-assisted development practices while maintaining high code quality and comprehensive testing.

## Development Setup

```bash
# Clone the repository
git clone <repository-url>
cd git-stack

# Install development dependencies
cargo build

# Run tests to verify setup
cargo test
```

## Architecture

### Project Structure

```
src/
├── lib.rs          # Core library functionality
├── main.rs         # CLI entry point
├── github.rs       # GitHub CLI integration
tests/
├── unit_tests.rs           # Unit tests for library functions
├── integration_tests.rs    # CLI integration tests
├── git_integration_tests.rs # Git-specific integration tests
├── list_command_tests.rs   # List command specific tests
├── context_tests.rs        # Context-aware behavior tests
├── sync_tests.rs           # Sync command tests
├── review_command_tests.rs # Review command tests
└── comprehensive_tests.rs  # Comprehensive edge case tests
openspec/
├── specs/          # Technical specifications
└── changes/        # Change proposals and history
```

The project follows Rust best practices with clear separation between:
- **Library code** (`src/lib.rs`) - Reusable functionality organized in modules
- **Binary code** (`src/main.rs`) - Minimal CLI parsing and orchestration
- **Trait abstractions** - `GitRunner` and `GitHubRunner` for dependency injection and testing
- **Comprehensive testing** - Unit, integration, and comprehensive edge case coverage

### Design Principles

- **Trait-based architecture** for testability and dependency injection
- **Context-aware behavior** that adapts to current git state
- **Fail-fast validation** with clear, actionable error messages
- **Git workflow compatibility** - works alongside regular git commands
- **Specification-driven development** with formal requirements and scenarios

## Development Workflow

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --test unit_tests
cargo test --test integration_tests
cargo test --test git_integration_tests
cargo test --test list_command_tests
cargo test --test sync_tests
cargo test --test context_tests
cargo test --test comprehensive_tests
```

### Code Quality

```bash
# Run linting
cargo clippy

# Format code
cargo fmt

# Run all quality checks
cargo test && cargo clippy && cargo fmt --check
```

### Adding New Features

1. **Write specifications** in `openspec/changes/` following the existing pattern
2. **Implement with tests** using the trait-based architecture
3. **Add integration tests** to verify CLI behavior
4. **Update documentation** including help text and examples

## Library API

The core functionality is available as a library for embedding in other applications:

```rust
use git_stack::{commands, RealGitRunner, github::RealGitHubRunner};

let git_runner = RealGitRunner;
let github_runner = RealGitHubRunner;

// Create a new branch
match commands::new_branch_contextual(&git_runner, Some("my-feature")) {
    Ok(branch_name) => println!("Created: {}", branch_name),
    Err(e) => eprintln!("Error: {}", e),
}

// List stacks (basic)
commands::list_stacks(&git_runner)?;

// List stacks with PR information
commands::list_stacks_with_github(&git_runner, Some(&github_runner))?;

// Sync stacks
commands::sync_stacks(&git_runner)?;

// Create PRs for review
commands::review_stack(&git_runner, &github_runner)?;
```

## Contributing

We welcome contributions! Please:

1. **Read the specifications** in `openspec/specs/` to understand the requirements
2. **Follow the existing patterns** for trait-based architecture and testing
3. **Write comprehensive tests** including edge cases
4. **Update documentation** for any user-facing changes
5. **Run the full test suite** before submitting

### Specification-Driven Development

This project uses OpenSpec for formal specifications. When adding features:

1. Create a change proposal in `openspec/changes/`
2. Define requirements with scenarios in `openspec/specs/`
3. Implement according to the specifications
4. Verify implementation matches all scenarios

This ensures consistency, testability, and maintainability across the codebase.
