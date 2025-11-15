# Tasks for improve-new-command-behavior

This document outlines the specific tasks needed to implement enhanced context-aware behavior for the git-stack new command.

## Development Tasks

### 1. Detect current branch context
- [x] Add function to determine if current branch follows stack naming pattern (`<feature>/<index>`)
- [x] Extract feature name and index from stack branch names
- [x] Identify base branches (main, master, etc.)

### 2. Update command parsing
- [x] Modify CLI to make feature_name optional in `new` subcommand
- [x] Handle special case of "." as feature name (treat as continuation)
- [x] Update help text to reflect new optional parameter behavior

### 3. Implement context-aware logic
- [x] When on base branch + feature name: use current behavior
- [x] When on base branch + no feature name: show error requiring feature name
- [x] When on stack branch + feature name: show error about starting new stack from diff
- [x] When on stack branch + no feature name: continue current stack
- [x] When on stack branch + "." feature name: continue current stack

### 4. Update branch creation logic
- [x] Add function to extract current stack context (feature name, next index)
- [x] Modify `new_branch` command to use context when feature name not provided
- [x] Ensure proper error handling for all edge cases

### 5. Enhance error messages
- [x] Create specific error types for different invalid contexts
- [x] Provide actionable guidance (e.g., "return to main branch to start new stack")
- [x] Update existing validation to use new error types

### 6. Update tests
- [x] Add tests for context detection functions
- [x] Test all command variations in different branch contexts
- [x] Verify error messages and exit codes
- [x] Test edge cases (malformed branch names, etc.)

### 7. Update documentation
- [x] Update CLI help text for new optional behavior
- [x] Add examples showing context-aware usage
- [x] Document error scenarios and resolutions

## Validation Tasks

### 8. Integration testing
- [x] Test workflow scenarios in real git repositories
- [x] Verify error handling with actual git commands
- [x] Test with various branch naming patterns

### 9. Regression testing
- [x] Ensure existing behavior unchanged on base branches
- [x] Verify all current tests still pass
- [x] Test with various git repository states

### 10. User experience validation
- [x] Test command discoverability and help text clarity
- [x] Validate error messages provide sufficient guidance
- [x] Ensure workflow feels natural and intuitive