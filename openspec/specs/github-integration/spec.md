# github-integration Specification

## Purpose
TBD - created by archiving change add-review-command. Update Purpose after archive.
## Requirements
### Requirement: GitHub CLI trait abstraction
The application MUST provide a trait-based abstraction for GitHub CLI operations similar to GitRunner.

#### Scenario: Define GitHubRunner trait for testability
```
GIVEN the application needs to interact with GitHub CLI
WHEN GitHubRunner trait is implemented
THEN it provides methods for PR creation, listing, and status checking
AND supports dependency injection for testing with mock implementations
AND follows the same pattern as the existing GitRunner trait
```

#### Scenario: Real GitHub CLI implementation
```
GIVEN the GitHubRunner trait is defined
WHEN RealGitHubRunner is implemented
THEN it executes actual 'gh' commands for GitHub operations
AND handles command execution failures appropriately
AND returns structured data from GitHub CLI output
```

#### Scenario: Mock GitHub CLI implementation for testing
```
GIVEN tests need to verify GitHub integration without actual API calls
WHEN MockGitHubRunner is implemented
THEN it simulates GitHub CLI responses for testing
AND allows configuration of different PR states and scenarios
AND enables comprehensive testing of review command logic
```

### Requirement: Pull request creation operations
The application MUST create pull requests using GitHub CLI with proper titles and descriptions.

#### Scenario: Create PR with stack-aware title
```
GIVEN a stack branch "feature-auth/2" needs a PR created
WHEN create_pull_request is called
THEN the PR title is formatted as "feature-auth #2"
AND the title follows the pattern "<feature-name> #<index>"
AND branch-specific formatting is consistent across the stack
```

#### Scenario: Create PR with dependency description
```
GIVEN branch "feature-auth/2" depends on PR #100 from "feature-auth/1"
WHEN create_pull_request is called for feature-auth/2
THEN the PR description includes "Depends on #100"
AND the dependency text is properly formatted for GitHub linking
AND any additional description content is preserved
```

#### Scenario: Create root PR without dependencies
```
GIVEN branch "feature-auth/1" is the first in the stack
WHEN create_pull_request is called
THEN the PR description does not include dependency text
AND the PR targets the default branch (main/master)
AND the title follows standard format "feature-auth #1"
```

### Requirement: Existing PR detection and analysis
The application MUST detect existing pull requests for stack branches to determine creation strategy.

#### Scenario: List PRs for specific branches
```
GIVEN a stack has multiple branches that may or may not have PRs
WHEN list_pull_requests_for_branches is called with branch list
THEN it returns mapping of branch names to PR numbers (if they exist)
AND excludes branches without associated PRs from results
AND provides structured data for creation strategy decisions
```

#### Scenario: Identify PR numbers for dependency chain
```
GIVEN stack branches have existing PRs: feature-auth/1 -> PR #100, feature-auth/3 -> PR #102
WHEN the review command analyzes existing PRs
THEN it identifies PR #100 for feature-auth/1
AND recognizes feature-auth/2 needs PR creation with dependency on #100
AND determines feature-auth/3's existing PR #102 may need dependency updates
```

#### Scenario: Handle GitHub CLI authentication failures
```
GIVEN GitHub CLI is not authenticated or lacks permissions
WHEN any PR operation is attempted
THEN the GitHub CLI returns authentication error
AND the application captures the error appropriately
AND provides user-friendly guidance on authentication steps
```

### Requirement: GitHub CLI availability and error handling
The application MUST verify GitHub CLI availability and provide meaningful error messages.

#### Scenario: Check GitHub CLI installation
```
GIVEN the review command needs to use GitHub CLI
WHEN check_github_cli_available is called
THEN it verifies 'gh' command is installed and accessible
AND returns clear error if GitHub CLI is not found
AND suggests installation steps for missing dependency
```

#### Scenario: Validate repository has GitHub remote
```
GIVEN the review command is executed in a git repository
WHEN GitHub operations are attempted
THEN it verifies the repository has a GitHub remote configured
AND returns error if no GitHub remote is detected
AND provides guidance on proper repository setup
```

#### Scenario: Handle GitHub API rate limits
```
GIVEN GitHub CLI operations encounter rate limiting
WHEN PR creation or listing commands fail with rate limit errors
THEN the application detects rate limit responses
AND provides informative error message about limits
AND suggests retry timing or authentication improvements
```

### Requirement: Structured PR data handling
The application MUST handle GitHub CLI output parsing for PR operations.

#### Scenario: Parse PR creation response
```
GIVEN a PR is successfully created via GitHub CLI
WHEN the command output is processed
THEN it extracts the PR number from the response
AND captures the PR URL for user feedback
AND returns structured data for subsequent operations
```

#### Scenario: Parse PR listing for stack analysis
```
GIVEN GitHub CLI returns list of PRs for repository
WHEN the output is parsed for stack branch analysis
THEN it maps branch names to PR numbers correctly
AND filters results to only stack-relevant PRs
AND provides data structure suitable for dependency logic
```

#### Scenario: Handle malformed GitHub CLI responses
```
GIVEN GitHub CLI returns unexpected output format
WHEN response parsing is attempted
THEN it handles parsing errors gracefully
AND provides fallback behavior or clear error messages
AND does not crash the application with unexpected data
```

