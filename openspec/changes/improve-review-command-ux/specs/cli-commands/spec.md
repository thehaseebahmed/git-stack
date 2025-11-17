# cli-commands Specification Delta

## MODIFIED Requirements

### Requirement: Review command processing
The application MUST provide a review command to create pull requests for git stacks using GitHub CLI with modern, user-friendly output formatting.

#### Scenario: User runs git-stack review on stack branch
```
GIVEN the user is on a stack branch like "feature-auth/2"
WHEN they execute `git-stack review`
THEN the application parses "review" as the command
AND identifies the current stack context
AND displays output with box-drawing characters: "┌  Creating PRs for stack: feature-auth"
AND shows progress with "◇  Found X diff(s) in stack"
AND displays spinner while checking for existing PRs
AND shows "◆  Created missing pull requests" with nested results
AND concludes with "└  All done!"
AND uses GitHub CLI to create PRs with proper dependencies
```

#### Scenario: User runs git-stack review on default branch
```
GIVEN the user is on the default branch and multiple stacks exist
WHEN they execute `git-stack review`
THEN the application displays an error indicating context is required
AND suggests running from a stack branch to review that specific stack
AND exits with status code 1
```

#### Scenario: User runs git-stack review on non-stack branch
```
GIVEN the user is on a branch that doesn't match stack or default patterns
WHEN they execute `git-stack review`
THEN the application displays an error indicating unsupported context
AND suggests switching to a stack branch to review
AND exits with status code 1
```

### Requirement: Review command progress indication
The review command MUST provide clear visual progress feedback using spinners for ongoing operations and static icons for completed steps.

#### Scenario: Display spinner during PR check operation
```
GIVEN the review command is checking for existing PRs
WHEN the GitHub API calls are in progress
THEN an animated spinner is displayed next to the operation description
AND the spinner continues until the check operation completes
AND the spinner is replaced with a static icon when complete
```

#### Scenario: Display spinner during PR creation operation
```
GIVEN the review command is creating new pull requests
WHEN each PR creation is in progress
THEN an animated spinner is displayed for the current operation
AND the spinner stops when the PR is successfully created
AND shows "✓ Created PR #N for branch-name" for each successful creation
```

#### Scenario: Show static icons for completed sections
```
GIVEN each major step of the review process completes
WHEN displaying the step in output
THEN static box-drawing characters are used: ┌ for start, ◇ for info, ◆ for action, └ for completion
AND vertical lines (│) connect sections to show hierarchy
AND proper indentation is maintained for nested items
```

### Requirement: Review command output formatting
The review command MUST use modern CLI formatting with box-drawing characters and clear visual hierarchy.

#### Scenario: Format output for stack with no existing PRs
```
GIVEN a stack "feature-auth" with 3 branches and no existing PRs
WHEN the review command executes successfully
THEN the output displays:
  "┌  Creating PRs for stack: feature-auth"
  "│"
  "◇  Found 3 diff(s) in stack"
  "│"
  "◆  Created missing pull requests"
  "│  ✓ Created PR #101 for feature-auth/1"
  "│  ✓ Created PR #102 for feature-auth/2"
  "│  ✓ Created PR #103 for feature-auth/3"
  "│"
  "└  All done!"
```

#### Scenario: Format output for stack with some existing PRs
```
GIVEN a stack "feature-auth" with 3 branches where branch 1 has existing PR #101
WHEN the review command executes successfully
THEN the output shows existing PRs were skipped
AND only displays newly created PRs in the "Created missing pull requests" section
AND maintains the same box-drawing structure
```

#### Scenario: Format output for stack with all existing PRs
```
GIVEN a stack "feature-auth" where all branches have existing PRs
WHEN the review command detects all PRs exist
THEN it displays the detection process with appropriate formatting
AND shows "└  All done!" without a "Created missing pull requests" section
AND provides summary that all PRs already exist
```

### Requirement: Review command terminology consistency
The review command MUST use "diff(s)" terminology instead of "branch(es)" to align with stack-based workflow concepts.

#### Scenario: Display diff count instead of branch count
```
GIVEN a stack with 3 branches
WHEN the review command displays the stack information
THEN it shows "Found 3 diff(s) in stack"
AND does not use "branch(es)" in user-facing output
```

#### Scenario: Refer to individual diffs in output
```
GIVEN PRs are being created for stack branches
WHEN displaying success messages
THEN individual messages refer to the branch name but contextually treat them as diffs
AND the overall messaging uses "diff" terminology
```
