# Add git-stack list command

## Why
Users need a way to visualize the structure of their git stacks in the local worktree. Currently there's no command to see which stacks exist and how they're organized, making it difficult to understand the branch structure when working with multiple features.

## What Changes
- Add new `git-stack list` command that displays all stacks in the repository
- Parse branch names to identify valid stack branches (feature-name/index pattern)
- Display stacks in a tree format showing the feature name and branch hierarchy
- Exclude non-stack branches from the output to focus on stack structure

## Impact
- Affected specs: cli-commands (new subcommand), git-integration (branch analysis)
- Affected code: main.rs (new command enum), lib.rs (new list functionality)
- **Non-breaking change** - adds new functionality without changing existing behavior
