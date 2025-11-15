# Improve git-stack new command behavior

## Problem

The current `git-stack new` command has a simple behavior: it always requires a feature name and creates a new branch with the pattern `<feature-name>/N` where N is the next available index. However, this doesn't handle the case where a user is already on a branch within an existing stack and wants to either:

1. Add a new diff to the current stack (continuation of the current feature)
2. Start a completely new stack from the current branch

The lack of context-aware behavior forces users to manually navigate to base branches and remember stack naming patterns, making the workflow less intuitive.

## Solution

Enhance the `git-stack new` command to be context-aware based on the current branch:

1. **When on a base branch (main/master)**: Maintain current behavior - require feature name and create `<feature-name>/1`

2. **When on a stack branch with explicit feature name**: Show error indicating they cannot start a new stack from an existing diff, suggesting they return to the base branch

3. **When on a stack branch without feature name or with "."**: Create the next branch in the current stack (e.g., from `feature-branch/3` create `feature-branch/4`)

This provides an intuitive workflow where `git-stack new` without arguments naturally continues the current stack, while explicit feature names are used for starting new stacks.

## Impact

- **Improved UX**: Users can naturally continue working on stacks without manually calculating next index numbers
- **Better error handling**: Clear guidance when users try to start new stacks from wrong contexts  
- **Backward compatibility**: Existing behavior on base branches remains unchanged
- **Reduced friction**: Less context switching required for common stack operations

## Capabilities Affected

- `cli-commands`: Command parsing and validation logic needs to handle optional feature name
- `git-integration`: Branch creation logic needs current branch context and stack detection

## Success Criteria

- `git-stack new` (no args) on stack branch creates next branch in sequence
- `git-stack new .` on stack branch creates next branch in sequence  
- `git-stack new <feature>` on stack branch shows helpful error message
- Existing behavior on base branches unchanged
- All error messages provide clear next steps