# git-integration Specification

## Purpose
TBD - created by archiving change implement-new-branch-command. Update Purpose after archive.
## Requirements
### Requirement: Git repository detection
The application MUST verify it's running in a valid git repository before executing git operations.

#### Scenario: Running in a git repository
```
GIVEN the current directory is a git repository
WHEN the application checks for git repository status
THEN it confirms the repository is valid
AND proceeds with git operations
```

#### Scenario: Running outside a git repository
```
GIVEN the current directory is not a git repository
WHEN the application checks for git repository status
THEN it displays an error message indicating no git repository found
AND exits with status code 1
```

### Requirement: Branch creation with stack naming
The application MUST create new branches following the stack naming pattern.

#### Scenario: Creating first branch in a stack
```
GIVEN the user is on branch "main"
WHEN they execute `git-stack new feature-auth`
THEN a new branch named "feature-auth/1" is created
AND the branch is created from the current branch
AND the user remains on the original branch
```

#### Scenario: Creating subsequent branch in existing stack
```
GIVEN branches "feature-auth/1" and "feature-auth/2" already exist
WHEN the user executes `git-stack new feature-auth`
THEN a new branch named "feature-auth/3" is created
AND uses the next available index number
```

#### Scenario: Creating branch for new feature name
```
GIVEN branch "feature-auth/1" exists
WHEN the user executes `git-stack new feature-ui`
THEN a new branch named "feature-ui/1" is created
AND starts the stack at index 1 for the new feature name
```

### Requirement: Git command execution
The application MUST execute git commands reliably and handle errors appropriately.

#### Scenario: Successful branch creation
```
GIVEN git is available in system PATH
WHEN the application executes git commands to create a branch
THEN the git command succeeds
AND the new branch exists in the repository
AND success is reported to the user
```

#### Scenario: Git command failure
```
GIVEN git command fails (e.g., permission error, disk full)
WHEN the application attempts to create a branch
THEN the error is captured and reported to the user
AND the application exits with status code 1
AND no partial state changes remain
```

#### Scenario: Git CLI not available
```
GIVEN git is not installed or not in PATH
WHEN the application attempts to execute git commands
THEN it displays an error message indicating git is not available
AND exits with status code 1
```

### Requirement: Branch name validation
The application MUST ensure generated branch names are valid git branch names.

#### Scenario: Valid feature names
```
GIVEN the user provides feature names like "auth", "feature-ui", "api-v2"
WHEN the application generates branch names
THEN the resulting names "auth/1", "feature-ui/1", "api-v2/1" are valid
AND can be successfully created by git
```

#### Scenario: Feature names requiring sanitization
```
GIVEN the user provides feature names with invalid characters
WHEN the application processes the feature name
THEN it validates the name against git branch naming rules
AND rejects names that would create invalid branches
AND provides helpful error messages about naming requirements
```

