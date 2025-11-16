# Enhance List Command with PR Visibility

## Overview
Improve the "git-stack list" command to display pull request numbers and status information for each stack branch, providing better visibility into the state of stacked PRs.

## Problem Statement
The current `git-stack list` command only shows branch names in a tree format (e.g., "feature-name/1", "feature-name/2"), but doesn't provide any information about associated pull requests or their status. Users have to manually check GitHub to see:

- Which branches have PRs
- What the PR numbers are
- Whether PRs are draft, open, or merged

## Proposed Solution
Enhance the list command to:

1. **Display PR numbers**: Show format "feature-name/1 #432 (<pr_status>)" where 432 is the PR number
2. **Show PR status**: Include status information (draft, open, merged, changes requested)  
3. **Color-code by status**:
   - **Green**: Merged PRs
   - **Gray**: Draft PRs
   - **Yellow**: Changes requested PRs
   - **Default**: Open PRs

## Example Output
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

## Dependencies
- Requires GitHub CLI (gh) integration for fetching PR information
- Builds on existing GitHub integration capabilities

## Impact
- **User Experience**: Provides complete stack overview without leaving terminal
- **Workflow Efficiency**: Eliminates need to check GitHub web interface for PR status
- **Visual Clarity**: Color coding makes status immediately apparent

## Scope
This change affects:
- CLI commands specification (list command behavior)
- GitHub integration specification (PR status retrieval)
- Implementation in list command and GitHub integration modules