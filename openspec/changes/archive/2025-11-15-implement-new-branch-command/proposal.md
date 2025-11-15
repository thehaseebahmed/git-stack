# Implement git-stack new command

## Overview
Add the first core feature to the git-stack CLI: the `new` command that creates new branches in a stacked workflow.

## Problem
Currently, git-stack is a basic "Hello World" application. Users need a way to create new feature branches that follow a structured naming pattern and integrate with git to establish branch stacks.

## Solution
Implement a `git-stack new <feature-name>` command that:
- Accepts a required feature name argument
- Creates a new git branch from the current branch
- Names the branch using the pattern `<feature-name>/<index>` where index starts at 1
- Uses the git CLI to perform the actual branch creation

## Scope
This change introduces two new capabilities:
1. **CLI command handling** - Parse and handle command-line arguments
2. **Git integration** - Execute git commands to create branches

## Out of Scope
- Branch stack management beyond basic creation
- Advanced git operations (merge, rebase, etc.)
- Configuration files or persistent state
- Interactive prompts or confirmation dialogs

## Success Criteria
- `git-stack new my-feature` creates a branch named `my-feature/1`
- Command provides appropriate error messages for invalid usage
- Integration with git CLI works reliably
- Code follows Rust best practices and passes linting

## Dependencies
- Access to git CLI in the system PATH
- Current directory must be a git repository

## Risks
- Git CLI availability and version compatibility
- Error handling for git command failures
- Naming conflicts with existing branches