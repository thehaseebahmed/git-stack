use git_stack::{commands, GitStackError, MockGitRunner};
use git_stack::github::{MockGitHubRunner, PullRequestInfo, PrStatus};

#[test]
fn test_analyze_stacks_with_valid_branches() {
    let branches = vec![
        "main".to_string(),
        "feature-auth/1".to_string(),
        "feature-auth/2".to_string(),
        "ui-update/1".to_string(),
        "random-branch".to_string(),
    ];

    let stacks = commands::analyze_stacks(&branches);

    assert_eq!(stacks.len(), 2);
    assert_eq!(stacks.get("feature-auth"), Some(&vec![1, 2]));
    assert_eq!(stacks.get("ui-update"), Some(&vec![1]));
    assert!(stacks.get("main").is_none());
    assert!(stacks.get("random-branch").is_none());
}

#[test]
fn test_analyze_stacks_with_no_stack_branches() {
    let branches = vec![
        "main".to_string(),
        "development".to_string(),
        "username/fix".to_string(),
        "feature-name-1".to_string(),
    ];

    let stacks = commands::analyze_stacks(&branches);

    assert_eq!(stacks.len(), 0);
}

#[test]
fn test_analyze_stacks_sorts_indices() {
    let branches = vec![
        "feature-test/3".to_string(),
        "feature-test/1".to_string(),
        "feature-test/2".to_string(),
    ];

    let stacks = commands::analyze_stacks(&branches);

    assert_eq!(stacks.len(), 1);
    assert_eq!(stacks.get("feature-test"), Some(&vec![1, 2, 3]));
}

#[test]
fn test_analyze_stacks_alphabetical_order() {
    let branches = vec![
        "zebra-feature/1".to_string(),
        "alpha-feature/1".to_string(),
        "beta-feature/1".to_string(),
    ];

    let stacks = commands::analyze_stacks(&branches);

    let stack_names: Vec<_> = stacks.keys().collect();
    assert_eq!(
        stack_names,
        vec!["alpha-feature", "beta-feature", "zebra-feature"]
    );
}

#[test]
fn test_list_stacks_integration() {
    let mock_runner = MockGitRunner::new().with_branches(vec![
        "main".to_string(),
        "feature-auth/1".to_string(),
        "feature-auth/2".to_string(),
        "ui-redesign/1".to_string(),
    ]);

    let result = commands::list_stacks(&mock_runner);
    assert!(result.is_ok());
}

#[test]
fn test_list_stacks_not_in_repo() {
    let mock_runner = MockGitRunner::new().not_in_repo();

    let result = commands::list_stacks(&mock_runner);
    assert!(matches!(result, Err(GitStackError::NotInGitRepository)));
}

#[test]
fn test_list_stacks_with_github_integration() {
    let mock_git_runner = MockGitRunner::new().with_branches(vec![
        "main".to_string(),
        "feature-auth/1".to_string(),
        "feature-auth/2".to_string(),
        "ui-redesign/1".to_string(),
    ]);

    let mock_github_runner = MockGitHubRunner::new()
        .with_pr_info("feature-auth/1", PullRequestInfo {
            number: 101,
            title: "Auth system part 1".to_string(),
            status: PrStatus::Merged,
        })
        .with_pr_info("feature-auth/2", PullRequestInfo {
            number: 102,
            title: "Auth system part 2".to_string(),
            status: PrStatus::Open,
        })
        .with_pr_info("ui-redesign/1", PullRequestInfo {
            number: 103,
            title: "UI redesign".to_string(),
            status: PrStatus::Draft,
        });

    let result = commands::list_stacks_with_github(&mock_git_runner, Some(&mock_github_runner));
    assert!(result.is_ok());
}

#[test]
fn test_list_stacks_github_failure_fallback() {
    let mock_git_runner = MockGitRunner::new().with_branches(vec![
        "main".to_string(),
        "feature-auth/1".to_string(),
    ]);

    let mock_github_runner = MockGitHubRunner::new().with_operation_failure();

    let result = commands::list_stacks_with_github(&mock_git_runner, Some(&mock_github_runner));
    // Should still succeed but without PR information
    assert!(result.is_ok());
}

#[test]
fn test_pr_status_display() {
    assert_eq!(PrStatus::Open.display(), "open");
    assert_eq!(PrStatus::Draft.display(), "draft");
    assert_eq!(PrStatus::Merged.display(), "merged");
    assert_eq!(PrStatus::ChangesRequested.display(), "changes requested");
}

#[test]
fn test_pr_status_colors() {
    assert_eq!(PrStatus::Open.color_code(), "");
    assert_eq!(PrStatus::Draft.color_code(), "\x1b[90m");
    assert_eq!(PrStatus::Merged.color_code(), "\x1b[32m");
    assert_eq!(PrStatus::ChangesRequested.color_code(), "\x1b[33m");
}
