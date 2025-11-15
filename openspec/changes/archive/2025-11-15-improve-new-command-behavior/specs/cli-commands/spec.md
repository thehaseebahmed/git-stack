# cli-commands Specification Delta

## MODIFIED Requirements

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

## ADDED Requirements

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