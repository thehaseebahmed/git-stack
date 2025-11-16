# github-integration Specification

## Purpose
This specification defines the requirements and scenarios for integrating GitHub CLI operations into the application. It establishes trait-based abstractions for testability, outlines real and mock implementations for interacting with GitHub pull requests, and ensures consistent and reliable PR creation workflows. The goal is to enable robust, testable, and maintainable GitHub integration within the application's review and stack management commands.
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
The application MUST extend GitHub CLI output parsing to include additional PR metadata for enhanced display capabilities.

#### Scenario: Parse comprehensive PR data from GitHub CLI
```
GIVEN GitHub CLI returns PR information including state, draft status, review decision, and title
WHEN the output is parsed for display purposes
THEN it extracts PR number, title, state, isDraft, and reviewDecision fields
AND maps state information to display-friendly status (merged/open/draft/changes requested)
AND provides structured data for color coding decisions
```

#### Scenario: Handle extended PR JSON fields
```
GIVEN GitHub CLI returns JSON with fields: number, title, state, isDraft, reviewDecision, mergedAt
WHEN response parsing processes the extended data
THEN it correctly maps "MERGED" state or non-null mergedAt to merged status
AND identifies draft PRs from isDraft boolean field
AND detects "CHANGES_REQUESTED" reviewDecision for changes requested status
AND provides fallback for missing or malformed field data
```

### Requirement: Pull request status and metadata retrieval
The application MUST extend GitHub CLI operations to retrieve comprehensive PR information including status, draft state, and title for display purposes.

#### Scenario: Retrieve PR status and metadata for branch
```
GIVEN a branch "feature-auth/1" has an associated pull request
WHEN get_pull_request_info is called for the branch
THEN it returns PR number, title, state (open/closed), draft status, and review decision
AND provides data structure suitable for display formatting
AND handles cases where branch has no associated PR
```

#### Scenario: Get PR information with state details
```
GIVEN a pull request exists with various states (open, closed, draft, merged, changes requested)
WHEN PR information is retrieved via GitHub CLI
THEN it correctly identifies merged state from GitHub API
AND distinguishes between draft and non-draft PRs
AND detects changes requested status from review decision
AND provides status information in structured format
```

#### Scenario: Batch retrieve PR information for multiple branches
```
GIVEN multiple stack branches need PR information for list display
WHEN batch_get_pull_request_info is called with branch list
THEN it efficiently queries GitHub CLI for all branches
AND returns mapping of branch names to PR information
AND handles cases where some branches have no PRs
```

