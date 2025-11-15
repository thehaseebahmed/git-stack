# git-integration Specification Delta

## MODIFIED Requirements

### Requirement: Branch creation with stack naming
The application MUST create new branches following the stack naming pattern with context awareness.

#### Scenario: Creating first branch in a new stack from base branch
```
GIVEN the user is on branch "main"  
WHEN they execute `git-stack new feature-auth` 
THEN a new branch named "feature-auth/1" is created
AND the branch is created from the current base branch
AND the user is switched to the new branch
```

#### Scenario: Continuing existing stack from stack branch
```
GIVEN the user is on branch "feature-auth/2"
WHEN they execute `git-stack new` (without feature name)
THEN a new branch named "feature-auth/3" is created
AND uses the next sequential index for the current stack
AND the branch is created from the current stack branch
```

#### Scenario: Continuing existing stack with dot notation
```
GIVEN the user is on branch "feature-ui/1"
WHEN they execute `git-stack new .`
THEN a new branch named "feature-ui/2" is created  
AND the dot is interpreted as stack continuation
AND follows the same behavior as `git-stack new` without arguments
```

## ADDED Requirements

### Requirement: Current branch analysis
The application MUST analyze the current branch to determine stack context and appropriate branch creation behavior.

#### Scenario: Parse stack branch information
```
GIVEN the current branch name follows pattern "feature-name/index"
WHEN the application analyzes the branch
THEN it extracts the feature name portion
AND determines the current index number
AND calculates the next available index
```

#### Scenario: Identify base branches for new stacks
```
GIVEN the current branch is the repository's default branch
WHEN the application analyzes the branch
THEN it identifies the branch as suitable for starting new stacks
AND requires explicit feature name for branch creation
```

#### Scenario: Handle edge cases in branch name parsing
```
GIVEN branch names with multiple slashes or complex patterns
WHEN the application attempts to parse stack information
THEN it handles parsing errors gracefully
AND defaults to treating branch as base branch
```

### Requirement: Context-aware branch creation
The application MUST adapt branch creation behavior based on current branch context.

#### Scenario: Prevent branch creation in invalid contexts
```
GIVEN the user is on stack branch "feature-auth/2"
WHEN they attempt to create a new stack with different feature name
THEN the branch creation is blocked before git commands
AND appropriate error message is displayed
AND no git operations are performed
```

#### Scenario: Create continuation branches efficiently
```
GIVEN the user is on stack branch with known context
WHEN creating a continuation branch
THEN the feature name is inferred from current branch
AND the next index is calculated automatically
AND git branch creation proceeds with generated name
```

## MODIFIED Requirements

### Requirement: Git command execution
The application MUST execute git commands reliably with enhanced context awareness and error handling.

#### Scenario: Branch creation with context validation
```
GIVEN git commands will be executed
WHEN the application creates a new branch
THEN it validates the current branch context first
AND only proceeds with git operations if context is valid
AND handles context-related errors before git execution
```

#### Scenario: Enhanced error reporting for context failures
```
GIVEN a branch creation fails due to context issues
WHEN the error is reported to user
THEN the error message includes current branch information
AND explains the specific context constraint violated
AND suggests concrete next steps for resolution
```