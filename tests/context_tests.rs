use git_stack::branch::{is_base_branch, parse_stack_branch};
use git_stack::{commands, GitStackError, MockGitRunner};

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
fn test_is_base_branch() {
    let mock_with_main = MockGitRunner::new().with_default_branch("main");
    let mock_with_master = MockGitRunner::new().with_default_branch("master");
    
    // Test with main as default branch
    assert!(is_base_branch(&mock_with_main, "main").unwrap());
    assert!(!is_base_branch(&mock_with_main, "master").unwrap());
    assert!(!is_base_branch(&mock_with_main, "feature").unwrap());
    assert!(!is_base_branch(&mock_with_main, "feature/1").unwrap());
    
    // Test with master as default branch  
    assert!(is_base_branch(&mock_with_master, "master").unwrap());
    assert!(!is_base_branch(&mock_with_master, "main").unwrap());
    assert!(!is_base_branch(&mock_with_master, "develop").unwrap());
    assert!(!is_base_branch(&mock_with_master, "ui-redesign/2").unwrap());
    assert!(!is_base_branch(&mock_with_master, "test_feature/3").unwrap());
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

#[test]
fn test_contextual_new_on_base_branch_with_feature() {
    let mock = MockGitRunner::new()
        .with_branches(vec!["main".to_string()])
        .with_current_branch("main");

    let result = commands::new_branch_contextual(&mock, Some("my-feature"));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "my-feature/1");
}

#[test]
fn test_contextual_new_on_base_branch_without_feature() {
    let mock = MockGitRunner::new()
        .with_branches(vec!["main".to_string()])
        .with_current_branch("main");

    let result = commands::new_branch_contextual(&mock, None);
    assert!(matches!(
        result,
        Err(GitStackError::FeatureNameRequiredOnBaseBranch(_))
    ));
}

#[test]
fn test_contextual_new_on_base_branch_with_dot() {
    let mock = MockGitRunner::new()
        .with_branches(vec!["main".to_string()])
        .with_current_branch("main");

    let result = commands::new_branch_contextual(&mock, Some("."));
    assert!(matches!(
        result,
        Err(GitStackError::FeatureNameRequiredOnBaseBranch(_))
    ));
}

#[test]
fn test_contextual_new_on_stack_branch_with_different_feature() {
    let mock = MockGitRunner::new()
        .with_branches(vec!["main".to_string(), "feature-auth/1".to_string()])
        .with_current_branch("feature-auth/1");

    let result = commands::new_branch_contextual(&mock, Some("different-feature"));
    assert!(matches!(
        result,
        Err(GitStackError::CannotStartNewStackFromDiff { .. })
    ));
}

#[test]
fn test_contextual_new_on_stack_branch_without_feature() {
    let mock = MockGitRunner::new()
        .with_branches(vec!["main".to_string(), "feature-auth/1".to_string()])
        .with_current_branch("feature-auth/1");

    let result = commands::new_branch_contextual(&mock, None);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "feature-auth/2");
}

#[test]
fn test_contextual_new_on_stack_branch_with_dot() {
    let mock = MockGitRunner::new()
        .with_branches(vec!["main".to_string(), "feature-auth/1".to_string()])
        .with_current_branch("feature-auth/1");

    let result = commands::new_branch_contextual(&mock, Some("."));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "feature-auth/2");
}

#[test]
fn test_contextual_new_on_stack_branch_with_same_feature() {
    let mock = MockGitRunner::new()
        .with_branches(vec!["main".to_string(), "feature-auth/1".to_string()])
        .with_current_branch("feature-auth/1");

    let result = commands::new_branch_contextual(&mock, Some("feature-auth"));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "feature-auth/2");
}
