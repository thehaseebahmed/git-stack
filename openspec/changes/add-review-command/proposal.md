# Add Review Command

## Summary

Add a new `git-stack review` command that creates pull requests for a complete stack using the GitHub CLI. The command intelligently handles both scenarios where no PRs exist for a stack and where some PRs have already been created, creating dependencies between PRs to maintain stack hierarchy.

## Why

This enhancement addresses a critical gap in the current git-stack workflow. While users can create and manage stacked branches locally, the transition to GitHub pull requests remains entirely manual. This creates friction that undermines the productivity benefits of stacked development:

1. **Manual PR Creation Overhead**: Users must create each PR individually, carefully setting titles, descriptions, and base branches
2. **Dependency Management Complexity**: Establishing "Depends on #123" relationships requires manual tracking of PR numbers and updates as the stack evolves
3. **Error-Prone Process**: Manual management leads to inconsistent naming, incorrect dependencies, or missed stack relationships
4. **Workflow Interruption**: The manual process breaks the flow of iterative development that stacked branches are designed to support

The review command eliminates these pain points by automating the entire PR creation workflow while maintaining the flexibility and power of stacked development.

## Motivation

Currently, git-stack helps users manage stacked branches locally but lacks automation for creating pull requests with proper dependencies on GitHub. Users must manually create PRs and establish dependencies, which is error-prone and time-consuming. The review command will bridge this gap by automatically:

1. Creating PRs for all stack branches that don't have them
2. Setting up proper PR dependencies to maintain stack order
3. Using consistent naming conventions aligned with branch patterns
4. Integrating seamlessly with existing GitHub workflows

## Key Requirements

- **Smart Stack Analysis**: Detect which branches in a stack already have PRs and start from the correct point
- **Dependency Management**: Automatically set up "Depends on #123" relationships between PRs
- **GitHub CLI Integration**: Use `gh` commands through a trait-based abstraction similar to GitRunner
- **Consistent Naming**: Generate PR titles that match branch naming (e.g., "feature-one #1" for "feature-one/1")
- **Proper Base Branches**: Ensure PRs target correct branches (stack PRs target main, not each other)

## Scope

This change affects two main areas:
1. **CLI Commands**: Add new `review` subcommand with argument parsing
2. **GitHub Integration**: New GitHub CLI trait and implementation for PR operations

## Architecture Changes

- Introduce `GitHubRunner` trait similar to existing `GitRunner` pattern
- Implement real and mock versions for testing
- Place GitHub CLI integration in separate module for clear separation
- Extend command parsing to handle new `review` subcommand

## Dependencies

- Existing git integration for stack analysis
- GitHub CLI (`gh`) availability on user's system
- Current branch and stack detection logic

## Success Criteria

- `git-stack review` creates PRs for entire stacks with correct dependencies
- Handles both new stacks and partially-created PR stacks correctly  
- Integrates cleanly with existing codebase architecture
- Comprehensive test coverage for both success and error scenarios