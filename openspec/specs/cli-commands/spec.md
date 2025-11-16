# cli-commands Specification

## Purpose
TBD - created by archiving change implement-new-branch-command. Update Purpose after archive.
## Requirements
### Requirement: Command-line argument parsing
The application MUST parse command-line arguments to identify and execute commands, with context-aware behavior for the new command.

#### Scenario: User runs git-stack new with feature name on base branch
```
GIVEN the user is on branch "main" 
WHEN they execute `git-stack new my-feature`
THEN the application parses "new" as the command
AND "my-feature" as the feature name argument
AND executes the new branch creation logic for a new stack
```

#### Scenario: User runs git-stack new without arguments on base branch
```
GIVEN the user is on a base branch like "main" or "master"
WHEN they execute `git-stack new` with no feature name argument
THEN the application displays an error message indicating feature name is required
AND shows usage information for the new command
AND exits with status code 1
```

#### Scenario: User runs git-stack new with feature name on stack branch  
```
GIVEN the user is on a stack branch like "feature-auth/2"
WHEN they execute `git-stack new another-feature`
THEN the application displays an error indicating cannot start new stack from existing diff
AND suggests returning to base branch to start new stack
AND exits with status code 1
```

#### Scenario: User runs git-stack new without arguments on stack branch
```
GIVEN the user is on a stack branch like "feature-auth/2"  
WHEN they execute `git-stack new`
THEN the application parses "new" as the command
AND detects the current stack context
AND executes branch creation logic to continue the current stack
```

#### Scenario: User runs git-stack new with dot on stack branch
```
GIVEN the user is on a stack branch like "feature-auth/2"
WHEN they execute `git-stack new .`
THEN the application treats "." as a continuation signal
AND executes branch creation logic to continue the current stack
```

### Requirement: New command validation
The new command MUST validate its arguments and current context before execution.

#### Scenario: New command validation on base branch requires feature name
```
GIVEN the user is on base branch "main"
WHEN they execute `git-stack new` without arguments
THEN the validation fails with clear error message
AND suggests proper usage with feature name
```

#### Scenario: New command validation prevents new stacks from diffs
```
GIVEN the user is on stack branch "feature-ui/1" 
WHEN they execute `git-stack new different-feature`
THEN the validation fails with context-aware error
AND provides guidance to return to base branch
```

### Requirement: Help and usage information
The application MUST provide clear usage information and help text.

#### Scenario: User requests help
```
GIVEN the user executes `git-stack --help` or `git-stack -h`
WHEN the application processes the request
THEN it displays comprehensive usage information
AND lists all available commands with descriptions
AND exits with status code 0
```

### Requirement: Branch context detection
The application MUST detect and parse the current branch to understand stack context.

#### Scenario: Detect base branch context
```
GIVEN the user is on the repository's default branch 
WHEN the application analyzes the current branch
THEN it identifies the branch as a base branch
AND allows new stack creation
```

#### Scenario: Detect stack branch context
```
GIVEN the user is on branch following pattern "feature-name/index"
WHEN the application analyzes the current branch  
THEN it extracts the feature name and current index
AND identifies it as a stack branch
```

#### Scenario: Handle malformed branch names
```
GIVEN the user is on branch with non-standard naming
WHEN the application analyzes the current branch
THEN it treats it as a base branch for safety
AND applies base branch validation rules
```

### Requirement: Context-aware error handling
The application MUST provide specific error messages based on current branch context.

#### Scenario: Error when starting new stack from diff
```
GIVEN the user attempts to create new stack from existing stack branch
WHEN the validation fails
THEN the error message clearly explains the context issue
AND provides actionable next steps to resolve
```

#### Scenario: Error when missing feature name on base branch
```
GIVEN the user omits feature name while on base branch
WHEN the validation fails  
THEN the error message explains feature name requirement
AND shows example usage patterns
```

### Requirement: List command processing
The application MUST provide a list command to display all git stacks in the repository.

#### Scenario: User runs git-stack list
```
GIVEN the user is in a git repository with stack branches
WHEN they execute `git-stack list`
THEN the application parses "list" as the command
AND executes the stack listing logic
AND displays all identified stacks in tree format
```

#### Scenario: User runs git-stack list in repository with no stacks
```
GIVEN the user is in a git repository with no stack branches
WHEN they execute `git-stack list`
THEN the application processes the command successfully
AND displays no stack information
AND exits with status code 0
```

### Requirement: Stack branch identification
The application MUST analyze branch names to identify valid stack branches using the feature-name/index pattern.

#### Scenario: Identify valid stack branches
```
GIVEN branches exist with names like "feature-auth/1", "feature-auth/2", "ui-update/1"
WHEN the list command analyzes the branches
THEN it identifies "feature-auth" and "ui-update" as valid stacks
AND groups branches by feature name
AND orders them by index number
```

#### Scenario: Filter out non-stack branches
```
GIVEN branches exist with names like "main", "feature-auth/1", "username/fix", "bugfix-temp"
WHEN the list command analyzes the branches
THEN it includes only "feature-auth/1" as a stack branch
AND excludes "main", "username/fix", and "bugfix-temp" from stack output
```

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

### Requirement: Sync command processing
The application MUST provide a sync command to synchronize git stacks with the remote repository.

#### Scenario: User runs git-stack sync on default branch
```
GIVEN the user is on the default branch (main/master)
WHEN they execute `git-stack sync`
THEN the application parses "sync" as the command
AND executes synchronization logic for all stacks in the repository
AND fetches from remote, pulls stack branches, and rebases all stacks
```

#### Scenario: User runs git-stack sync on stack branch
```
GIVEN the user is on a stack branch like "feature-auth/2"
WHEN they execute `git-stack sync`
THEN the application identifies the current stack context
AND executes synchronization logic for only the current stack
AND fetches from remote, pulls current stack branches, and rebases the stack
```

#### Scenario: User runs git-stack sync on non-stack branch
```
GIVEN the user is on a branch that is not default or stack branch
WHEN they execute `git-stack sync`
THEN the application displays an error indicating unsupported context
AND suggests switching to default branch or stack branch
AND exits with status code 1
```

### Requirement: Context-aware sync behavior
The sync command MUST adapt its synchronization scope based on the current branch context.

#### Scenario: Sync all stacks from default branch
```
GIVEN the user is on default branch and multiple stacks exist
WHEN the sync command executes
THEN it identifies all stacks in the repository
AND synchronizes each stack independently
AND provides progress feedback for each stack processed
```

#### Scenario: Sync current stack only from stack branch
```
GIVEN the user is on "feature-auth/2" with stack "feature-auth/1,2,3"
WHEN the sync command executes
THEN it identifies only the "feature-auth" stack
AND synchronizes only branches belonging to this stack
AND ignores other stacks in the repository
```

#### Scenario: Error on invalid branch context
```
GIVEN the user is on branch "random-branch" that doesn't match patterns
WHEN the sync command validates context
THEN it rejects the operation with clear error message
AND explains valid contexts for running sync
```

### Requirement: Sync progress and feedback
The sync command MUST provide clear feedback about synchronization progress and results.

#### Scenario: Display sync progress for multiple stacks
```
GIVEN the sync command processes multiple stacks
WHEN each stack operation begins and completes
THEN progress messages indicate current stack being processed
AND success/failure status for each stack is reported
AND final summary shows overall sync results
```

#### Scenario: Report sync completion for single stack
```
GIVEN the sync command processes one stack
WHEN the synchronization completes successfully
THEN completion message indicates stack name and branches updated
AND any rebase conflicts or issues are clearly reported
```

### Requirement: Review command processing
The application MUST provide a review command to create pull requests for git stacks using GitHub CLI.

#### Scenario: User runs git-stack review on stack branch
```
GIVEN the user is on a stack branch like "feature-auth/2"
WHEN they execute `git-stack review`
THEN the application parses "review" as the command
AND identifies the current stack context
AND executes pull request creation for the entire "feature-auth" stack
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

### Requirement: Review command validation
The review command MUST validate context and GitHub CLI availability before execution.

#### Scenario: Review command validates GitHub CLI availability
```
GIVEN the user executes git-stack review from valid context
WHEN the validation checks system requirements
THEN it verifies GitHub CLI (gh) is installed and accessible
AND exits with helpful error if gh is not available
```

#### Scenario: Review command validates GitHub authentication
```
GIVEN GitHub CLI is available but user is not authenticated
WHEN the review command attempts GitHub operations
THEN it detects authentication failure from gh commands
AND provides guidance on running 'gh auth login'
AND exits with status code 1
```

### Requirement: Stack PR analysis and creation strategy
The review command MUST analyze existing PRs for a stack and determine creation strategy.

#### Scenario: Create PRs for stack with no existing PRs
```
GIVEN stack "feature-auth" has branches /1, /2, /3 with no existing PRs
WHEN the review command analyzes the stack
THEN it identifies all branches need PRs created
AND creates PRs starting from feature-auth/1
AND establishes dependencies: /2 depends on /1, /3 depends on /2
```

#### Scenario: Create PRs for partially-reviewed stack
```
GIVEN stack "feature-auth" has branches /1, /2, /3 where /1 already has PR #100
WHEN the review command analyzes the stack
THEN it identifies /1 has existing PR and skips creation
AND creates PR for /2 with "Depends on #100" in description
AND creates PR for /3 with dependency on /2's new PR
```

#### Scenario: Handle stack with all PRs already created
```
GIVEN stack "feature-auth" has PRs for all branches
WHEN the review command analyzes the stack
THEN it detects all PRs exist and reports status
AND displays summary of existing PRs with links
AND exits successfully without creating duplicates
```

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

