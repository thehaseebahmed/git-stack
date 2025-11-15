use git_stack::branch::{
    get_next_index_from_branches, parse_stack_branch, validate_feature_name,
};
use git_stack::{GitRunner, GitStackError, MockGitRunner};

#[test]
fn test_validate_feature_name_valid() {
    assert!(validate_feature_name("valid-feature").is_ok());
    assert!(validate_feature_name("feature_name").is_ok());
    assert!(validate_feature_name("feature123").is_ok());
    assert!(validate_feature_name("123feature").is_ok());
}

#[test]
fn test_validate_feature_name_invalid() {
    assert!(validate_feature_name("").is_err());
    assert!(validate_feature_name("feature name").is_err());
    assert!(validate_feature_name("feature@name").is_err());
    assert!(validate_feature_name("feature.name").is_err());
    assert!(validate_feature_name("feature/name").is_err());
}

#[test]
fn test_get_next_branch_index_from_branches_no_existing() {
    let branches = vec![];
    assert_eq!(get_next_index_from_branches("feature", &branches), 1);
}

#[test]
fn test_get_next_branch_index_from_branches_with_existing() {
    let branches = vec![
        "main".to_string(),
        "feature/1".to_string(),
        "feature/2".to_string(),
        "other/1".to_string(),
    ];
    assert_eq!(get_next_index_from_branches("feature", &branches), 3);
    assert_eq!(get_next_index_from_branches("other", &branches), 2);
    assert_eq!(get_next_index_from_branches("new", &branches), 1);
}

#[test]
fn test_get_next_branch_index_from_branches_non_sequential() {
    let branches = vec![
        "feature/1".to_string(),
        "feature/3".to_string(),
        "feature/5".to_string(),
    ];
    assert_eq!(get_next_index_from_branches("feature", &branches), 6);
}

#[test]
fn test_get_next_branch_index_from_branches_invalid_numbers() {
    let branches = vec![
        "feature/1".to_string(),
        "feature/abc".to_string(),
        "feature/2".to_string(),
        "feature/".to_string(),
    ];
    assert_eq!(get_next_index_from_branches("feature", &branches), 3);
}

#[test]
fn test_error_display() {
    let err = GitStackError::InvalidFeatureName("test".to_string());
    assert_eq!(err.to_string(), "Invalid feature name 'test': Feature names must contain only alphanumeric characters, hyphens, and underscores");

    let err = GitStackError::NotInGitRepository;
    assert_eq!(
        err.to_string(),
        "Not in a git repository. Please run this command from within a git repository."
    );
}

#[test]
fn test_mock_git_runner_basic() {
    let mock = MockGitRunner::new()
        .with_branches(vec!["main".to_string(), "feature/1".to_string()])
        .with_current_branch("main");

    // Test that mock responds correctly
    let result = mock.run_command(&["branch", "--list", "--format=%(refname:short)"]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "main\nfeature/1");

    let current = mock.run_command(&["rev-parse", "--abbrev-ref", "HEAD"]);
    assert!(current.is_ok());
    assert_eq!(current.unwrap(), "main");

    assert!(mock.is_repository().unwrap());
}

#[test]
fn test_mock_git_runner_failure() {
    let mock = MockGitRunner::new().should_fail();

    let result = mock.run_command(&["branch", "--list", "--format=%(refname:short)"]);
    assert!(matches!(result, Err(GitStackError::GitCommandFailed(_))));
}

#[test]
fn test_mock_git_runner_not_in_repo() {
    let mock = MockGitRunner::new().not_in_repo();

    assert!(!mock.is_repository().unwrap());
}

// Tests for new context detection functionality
#[test]
fn test_parse_stack_branch_valid() {
    let stack_info = parse_stack_branch("feature-auth/1").unwrap();
    assert_eq!(stack_info.feature_name, "feature-auth");
    assert_eq!(stack_info.index, 1);

    let stack_info = parse_stack_branch("ui-redesign/42").unwrap();
    assert_eq!(stack_info.feature_name, "ui-redesign");
    assert_eq!(stack_info.index, 42);

    let stack_info = parse_stack_branch("feature_name/3").unwrap();
    assert_eq!(stack_info.feature_name, "feature_name");
    assert_eq!(stack_info.index, 3);
}

#[test]
fn test_parse_stack_branch_invalid() {
    assert!(parse_stack_branch("main").is_none());
    assert!(parse_stack_branch("feature").is_none());
    assert!(parse_stack_branch("feature/").is_none());
    assert!(parse_stack_branch("feature/abc").is_none());
    assert!(parse_stack_branch("feature/0").is_none()); // Index must be positive
    assert!(parse_stack_branch("feature name/1").is_none()); // Invalid feature name
    assert!(parse_stack_branch("").is_none());
    assert!(parse_stack_branch("/1").is_none());
}

#[test]
fn test_context_aware_error_messages() {
    let err = GitStackError::FeatureNameRequiredOnBaseBranch("main".to_string());
    assert!(err
        .to_string()
        .contains("Feature name is required when creating a new stack from base branch 'main'"));
    assert!(err.to_string().contains("git-stack new <feature-name>"));

    let err = GitStackError::CannotStartNewStackFromDiff {
        current_branch: "feature-auth/2".to_string(),
        attempted_feature: "ui-redesign".to_string(),
    };
    assert!(err.to_string().contains(
        "Cannot start new stack 'ui-redesign' from existing stack branch 'feature-auth/2'"
    ));
    assert!(err.to_string().contains("git checkout main"));
}
