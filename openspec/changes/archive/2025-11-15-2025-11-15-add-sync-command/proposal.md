# Add Sync Command

## Summary

Add a new `git-stack sync` command that synchronizes git stacks with the remote repository by fetching updates, pulling changes to stack branches, and rebasing stacks to maintain up-to-date branches.

## Problem

Currently, there's no command to efficiently synchronize multiple stack branches with the remote repository. Users must manually:
- Run `git fetch` to get remote updates
- Checkout and pull each stack branch individually  
- Rebase stack branches to incorporate upstream changes
- Handle the complexity of identifying which branches belong to which stacks

This is time-consuming and error-prone, especially when working with multiple stacks or when some stack branches have been merged and deleted.

## Proposed Solution

Implement a `git-stack sync` command with context-aware behavior:

### When on Default Branch (e.g., main/master)
- Sync ALL stacks in the repository
- Fetch from remote
- Pull each stack branch
- Rebase each stack using `git rebase --update-refs` on the first branch

### When on Stack Branch
- Sync ONLY the current stack
- Fetch from remote
- Pull all branches in the current stack
- Rebase the current stack using `git rebase --update-refs` on the first branch

### When on Non-Stack/Non-Default Branch
- Display error indicating unsupported branch context
- Suggest switching to default branch or a stack branch

## Architecture

The sync command will leverage existing git-stack infrastructure:

1. **Context Detection**: Use existing `get_current_context()` and branch parsing functions
2. **Stack Analysis**: Use existing `analyze_stacks()` to identify all stacks or current stack
3. **Git Operations**: Extend git module with new operations (fetch, pull, rebase with --update-refs)
4. **Error Handling**: Use existing error system with context-aware messages

## Key Features

1. **Smart First Branch Detection**: Identify the actual first branch in a stack (not always index 1) by checking which branches exist
2. **Update-refs Rebasing**: Use `git rebase --update-refs` to efficiently update all branches in a stack simultaneously
3. **Context-aware Execution**: Different behavior based on current branch context
4. **Comprehensive Error Handling**: Clear messages for various failure scenarios
5. **Safe Operation**: Validate repository state before making changes

## Impact

- **Improved Workflow**: Single command to sync all or specific stacks
- **Reduced Complexity**: Eliminates manual multi-step sync process
- **Better Stack Management**: Maintains proper branching hierarchy automatically
- **Consistent with Existing Commands**: Follows established patterns and conventions

## Dependencies

- Builds on existing cli-commands and git-integration specifications
- Requires extensions to git module for new operations
- Compatible with current stack detection and branch analysis logic

## Risks and Mitigation

- **Merge Conflicts**: Command will fail safely and provide clear error messages
- **Missing Remote Branches**: Handle cases where remote branches don't exist
- **Network Issues**: Standard git error handling for fetch/pull failures
- **Rebase Complexity**: Validate stack state before rebasing operations