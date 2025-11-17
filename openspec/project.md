# Project Context

## Purpose
git-stack is a developer productivity tool that makes stacked pull requests simple, intuitive, and GitHub‑native. It streamlines the workflow of creating, managing, and merging dependent branches.

## Tech Stack
- Rust (stable)
- Git (system dependency)
- GitHub CLI (for PR operations)

## Project Conventions

### Terminology
**Critical: Understanding stack vs diff terminology**

- **Stack**: A collection of related branches organized under a common feature name (e.g., `feature-auth`, `payment-flow`). A stack represents a single logical feature broken into multiple independent changes.
- **Diff**: An individual branch within a stack (e.g., `feature-auth/1`, `feature-auth/2`). Each diff represents one reviewable unit of change.

**Example:**
```
feature-auth         ← This is a stack
├─ feature-auth/1    ← This is a diff
├─ feature-auth/2    ← This is a diff  
└─ feature-auth/3    ← This is a diff

payment-flow         ← This is a stack
└─ payment-flow/1    ← This is a diff
```

When referring to:
- Multiple feature groupings → "stacks"
- Individual branches within a stack → "diffs" or "diff(s)"
- The naming pattern itself → "stack naming pattern" or "feature-name/index pattern"

**Important**: While git internally treats everything as branches, in git-stack:
- We call individual branches like `feature-auth/1` a "diff" (not "branch")
- We call the overall feature grouping like `feature-auth` a "stack" (not "branch")
- CLI output should consistently use "diff(s)" when referring to stack branches

### Code Style
- Rust standard style (enforced via `cargo fmt`)
- snake_case for functions/variables
- PascalCase for types
- Explicit types for public APIs
- Document public APIs with `///` comments
- Use `Result<T, E>` with `?` operator for error handling

### Architecture Patterns
- Trait-based architecture for testability (`GitRunner`, `GitHubRunner`)
- Dependency injection for test mocking
- Context-aware commands that adapt based on current git state
- Fail-fast validation with actionable error messages

### Testing Strategy
- Comprehensive unit tests for library functions
- Integration tests for CLI behavior
- Mock implementations for git and GitHub operations
- Test coverage for edge cases and error conditions

### Git Workflow
- Main branch as default
- Feature branches follow `feature-name/index` pattern
- Stack-based development with sequential indices
- PRs created with dependency chains

## Domain Context
git-stack manages "stacks" of dependent changes, where each "diff" (branch) in a stack builds on the previous one. This enables breaking large features into small, reviewable units while maintaining proper dependency relationships in pull requests.

## Important Constraints
- Must be run from within a git repository
- Feature names: alphanumeric, hyphens, and underscores only
- GitHub CLI required for PR operations
- Stack branches must follow `feature-name/index` naming pattern

## External Dependencies
- Git (required, must be in PATH)
- GitHub CLI (required for `review` and enhanced `list` commands)
- GitHub remote repository (for `review` command)
