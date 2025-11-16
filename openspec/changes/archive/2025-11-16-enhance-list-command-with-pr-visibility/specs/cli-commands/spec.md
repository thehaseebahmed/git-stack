# cli-commands Specification Delta

## MODIFIED Requirements

### Requirement: Tree format display
The application MUST display stacks in a hierarchical tree format showing feature names, their branches, PR numbers, and status with appropriate color coding.

#### Scenario: Display stack branches with PR numbers and status
```
GIVEN stack branches "feature-auth/1", "feature-auth/2" exist with PRs #100, #105
WHEN the list command displays the output with PR information
THEN it shows tree structure with PR numbers: "feature-auth/1 #100 (merged)"
AND displays PR status: "feature-auth/2 #105 (open)"
AND applies appropriate color coding based on PR status
```

#### Scenario: Display branches without PRs
```
GIVEN stack branch "feature-auth/3" exists without a PR
WHEN the list command displays the output
THEN it shows the branch without PR information: "feature-auth/3"
AND uses default color formatting
AND maintains tree formatting consistency
```

#### Scenario: Color-code PR status
```
GIVEN branches with different PR states: merged, draft, open, and changes requested
WHEN the list command displays PR information
THEN merged PRs are displayed in green color
AND draft PRs are displayed in gray color
AND changes requested PRs are displayed in yellow color
AND open PRs use default terminal color
```

#### Scenario: Handle GitHub integration failures gracefully
```
GIVEN GitHub CLI is not available or authentication fails
WHEN the list command attempts to fetch PR information
THEN it falls back to displaying branches without PR data
AND shows a warning about GitHub integration issues
AND continues to display the tree structure normally
```

## ADDED Requirements

### Requirement: PR information integration for list command
The list command MUST integrate with GitHub operations to fetch and display pull request information for stack branches.

#### Scenario: Fetch PR numbers and status for all stack branches
```
GIVEN multiple stack branches exist in the repository
WHEN the list command executes with GitHub integration enabled
THEN it queries PR information for each stack branch
AND retrieves PR number, title, status (open/draft/merged/changes requested), and review decision
AND incorporates this data into the tree display format
```

#### Scenario: Format PR information in list output
```
GIVEN branch "feature-auth/1" has PR #432 in "merged" status
WHEN the list command formats the output
THEN it displays "feature-auth/1 #432 (merged)" format
AND applies green color to the entire line
AND maintains proper tree indentation and formatting
```

#### Scenario: Optimize GitHub API calls for performance
```
GIVEN a repository with many stack branches
WHEN the list command fetches PR information
THEN it batches GitHub CLI calls where possible
AND limits API requests to avoid rate limiting
AND caches results during the command execution
```