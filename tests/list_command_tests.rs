use git_stack::{commands, GitStackError, MockGitRunner};

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
    assert_eq!(stack_names, vec!["alpha-feature", "beta-feature", "zebra-feature"]);
}

#[test]
fn test_list_stacks_integration() {
    let mock_runner = MockGitRunner::new()
        .with_branches(vec![
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