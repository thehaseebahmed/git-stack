# cli-commands Spec Deltas

## ADDED Requirements

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