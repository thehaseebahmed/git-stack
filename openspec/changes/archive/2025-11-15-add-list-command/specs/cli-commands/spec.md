## ADDED Requirements

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
The application MUST display stacks in a hierarchical tree format showing feature names and their branches.

#### Scenario: Display single stack with multiple branches
```
GIVEN stack branches "feature-auth/1", "feature-auth/2", "feature-auth/3" exist
WHEN the list command displays the output
THEN it shows a tree structure with "feature-auth" as root
AND displays branches "1", "2", "3" as children
AND uses appropriate tree formatting characters
```

#### Scenario: Display multiple stacks
```
GIVEN multiple stacks exist like "feature-auth/1", "feature-auth/2", "ui-redesign/1"  
WHEN the list command displays the output
THEN it shows separate tree structures for each feature
AND groups branches under their respective feature names
AND maintains consistent formatting across all stacks
```
