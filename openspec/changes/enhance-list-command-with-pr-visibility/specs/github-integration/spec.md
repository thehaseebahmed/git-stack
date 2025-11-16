# github-integration Specification Delta

## ADDED Requirements

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

## MODIFIED Requirements

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

## Extended Error Handling

### Requirement: GitHub CLI error handling for list command integration
The application MUST gracefully handle GitHub CLI failures when used within the list command context.

#### Scenario: Handle GitHub CLI unavailability during list command
```
GIVEN GitHub CLI is not installed or authenticated
WHEN list command attempts to fetch PR information
THEN it logs warning about GitHub integration unavailability
AND continues displaying tree structure without PR data
AND provides user guidance about GitHub CLI setup
```

#### Scenario: Handle rate limiting during PR information retrieval
```
GIVEN GitHub API rate limits are exceeded during PR queries
WHEN batch PR information retrieval encounters rate limiting
THEN it handles rate limit errors gracefully
AND falls back to displaying branches without PR information
AND provides informative message about rate limiting
```