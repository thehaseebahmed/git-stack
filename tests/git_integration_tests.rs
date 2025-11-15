use git_stack::{git, GitRunner, GitStackError, MockGitRunner};

#[test]
fn test_git_repository_detection_with_mock() {
    // Test when we're in a git repository
    let mock_in_repo = MockGitRunner::new();
    let result = git::check_repository(&mock_in_repo);
    assert!(result.is_ok());

    // Test when we're not in a git repository
    let mock_not_in_repo = MockGitRunner::new().not_in_repo();
    let result = git::check_repository(&mock_not_in_repo);
    assert!(matches!(result, Err(GitStackError::NotInGitRepository)));
}

#[test]
fn test_git_operations_with_mock() {
    let mock = MockGitRunner::new()
        .with_branches(vec!["main".to_string(), "feature/1".to_string()])
        .with_current_branch("main");

    // Test getting current branch
    let current_branch = git::get_current_branch(&mock);
    assert!(current_branch.is_ok());
    assert_eq!(current_branch.unwrap(), "main");

    // Test listing branches
    let branches = git::list_branches(&mock);
    assert!(branches.is_ok());
    let branch_list = branches.unwrap();
    assert_eq!(branch_list, vec!["main", "feature/1"]);
}

#[test]
fn test_git_command_execution_with_mock() {
    let mock = MockGitRunner::new();

    // Test successful command
    let result = mock.run_command(&["branch", "--list", "--format=%(refname:short)"]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "main");

    // Test git command failure
    let failing_mock = MockGitRunner::new().should_fail();
    let result = failing_mock.run_command(&["branch", "--list", "--format=%(refname:short)"]);
    assert!(matches!(result, Err(GitStackError::GitCommandFailed(_))));
}

#[test]
fn test_git_branch_creation_with_mock() {
    let mock = MockGitRunner::new();

    let result = git::create_branch(&mock, "new-branch");
    assert!(result.is_ok());
}

#[test]
fn test_empty_branch_list() {
    let mock = MockGitRunner::new().with_branches(vec![]);

    let branches = git::list_branches(&mock);
    assert!(branches.is_ok());
    assert!(branches.unwrap().is_empty());
}

#[test]
fn test_git_operations_failure_scenarios() {
    let mock = MockGitRunner::new().should_fail();

    // All git operations should fail
    assert!(git::get_current_branch(&mock).is_err());
    assert!(git::list_branches(&mock).is_err());
    assert!(git::create_branch(&mock, "test").is_err());
}

#[test]
fn test_complex_branch_scenarios() {
    let mock = MockGitRunner::new()
        .with_branches(vec![
            "main".to_string(),
            "develop".to_string(),
            "feature/auth/1".to_string(),
            "feature/auth/2".to_string(),
            "feature/ui/1".to_string(),
            "hotfix/critical-fix/1".to_string(),
        ])
        .with_current_branch("feature/auth/2");

    let branches = git::list_branches(&mock).unwrap();
    assert_eq!(branches.len(), 6);
    assert!(branches.contains(&"main".to_string()));
    assert!(branches.contains(&"feature/auth/2".to_string()));

    let current = git::get_current_branch(&mock).unwrap();
    assert_eq!(current, "feature/auth/2");
}
