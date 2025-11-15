# Implementation Tasks

This document outlines the ordered implementation tasks for the git-stack review command feature.

## 1. GitHub CLI Trait Foundation
- [x] Create `src/github.rs` module for GitHub CLI integration
- [x] Define `GitHubRunner` trait with core methods:
  - `check_availability() -> Result<()>`
  - `create_pull_request(branch: &str, title: &str, body: &str, base: &str) -> Result<u32>`
  - `list_pull_requests_for_branch(branch: &str) -> Result<Option<u32>>`
  - `get_default_branch() -> Result<String>`
- [x] Implement `RealGitHubRunner` using `gh` CLI commands
- [x] Implement `MockGitHubRunner` for testing with configurable responses
- [x] Add comprehensive unit tests for GitHub trait implementations

## 2. Review Command Integration
- [x] Add `Review` variant to `Commands` enum in `src/main.rs`
- [x] Update CLI argument parsing to handle review command
- [x] Create `review_stack()` function in commands module
- [x] Add validation for:
  - GitHub CLI availability
  - Valid branch context (must be on stack branch)
  - GitHub repository remote configuration
- [x] Implement context-aware error messages for review command

## 3. Stack PR Analysis Logic
- [x] Create stack analysis functions:
  - `analyze_stack_pr_status(git_runner: &dyn GitRunner, github_runner: &dyn GitHubRunner, feature_name: &str) -> Result<StackPRStatus>`
  - `determine_creation_strategy(pr_status: StackPRStatus) -> PRCreationPlan`
- [x] Design data structures for:
  - Mapping branches to existing PR numbers
  - Creation plan with dependencies
  - PR metadata (title, description, base branch)
- [x] Implement logic to find first branch needing PR creation
- [x] Add dependency chain calculation for PR descriptions

## 4. PR Creation and Dependency Management
- [x] Implement PR creation workflow:
  - Generate PR titles using "feature-name #index" format
  - Build PR descriptions with dependency information
  - Target correct base branches (main for stack PRs, not previous branch)
- [x] Add "Depends on #123" text formatting for PR descriptions
- [x] Implement batch PR creation with progress feedback
- [x] Handle PR creation failures gracefully with partial rollback

## 5. Integration and Error Handling
- [x] Integrate review command with existing git operations
- [x] Add comprehensive error handling for:
  - GitHub CLI authentication failures
  - Network connectivity issues
  - GitHub API rate limiting
  - Repository permission problems
- [x] Implement user-friendly error messages with actionable guidance
- [x] Add success feedback with PR URLs and summary

## 6. Testing and Validation
- [x] Write unit tests for GitHub CLI trait implementations
- [x] Create integration tests for review command scenarios:
  - New stack with no PRs
  - Partially created stack
  - Stack with all PRs existing
  - Error conditions (auth failure, no GitHub remote, etc.)
- [x] Add end-to-end tests using mock GitHub CLI responses
- [x] Test error handling and edge cases thoroughly

## 7. Documentation and Polish
- [x] Update README with review command usage examples
- [x] Add help text and usage information for review command
- [x] Document GitHub CLI requirements and setup steps
- [x] Add examples of typical workflow with review command
- [x] Ensure consistent code formatting and documentation

## Validation Criteria

Each task should be validated with:
- [x] Code compiles successfully with `cargo build`
- [x] All tests pass with `cargo test`
- [x] Code formatting is correct with `cargo fmt`
- [x] Linting passes with `cargo clippy`
- [x] Integration tests verify expected behavior
- [x] Error messages are clear and actionable
- [x] Command usage follows existing patterns

## Dependencies and Sequencing

- Tasks 1-2 can be worked in parallel after initial trait design
- Task 3 depends on Task 1 (GitHub trait) completion
- Task 4 depends on Tasks 1 and 3 (requires both GitHub trait and analysis logic)
- Task 5 can be integrated throughout Tasks 2-4
- Tasks 6-7 are final validation and documentation phases

## Risk Mitigation

- **GitHub CLI Dependency**: Ensure graceful handling when `gh` is not available
- **Authentication Complexity**: Provide clear guidance for GitHub authentication setup
- **Rate Limiting**: Implement appropriate error handling for GitHub API limits
- **Partial Failures**: Design rollback strategy for partial PR creation failures