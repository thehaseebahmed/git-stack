use std::process::Command;

/// Test the CLI behavior using the actual binary
#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("A tool for managing stacked git branches"));
    assert!(stdout.contains("new"));
    assert!(stdout.contains("list"));
}

#[test]
fn test_cli_version() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.1.0"));
}

#[test]
fn test_cli_new_command_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "new", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Create a new branch in the current stack"));
    assert!(stdout.contains("FEATURE_NAME"));
}

/// Test that the CLI properly handles invalid feature names
#[test]
fn test_cli_new_command_invalid_feature_name() {
    let output = Command::new("cargo")
        .args(&["run", "--", "new", "invalid name with spaces"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid feature name"));
}

#[test]
fn test_cli_list_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "list"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    // The exact output depends on what branches exist in the test environment,
    // but the command should succeed
}

/// Integration test using the git_stack library directly with MockGitRunner
mod library_integration {
    use git_stack::{commands, GitRunner, GitStackError, MockGitRunner};

    #[test]
    fn test_new_branch_outside_git_repo() {
        let mock = MockGitRunner::new().not_in_repo();

        let result = commands::new_branch(&mock, "test-feature");
        assert!(matches!(result, Err(GitStackError::NotInGitRepository)));
    }

    #[test]
    fn test_invalid_feature_name_through_library() {
        let mock = MockGitRunner::new();

        let result = commands::new_branch(&mock, "invalid name");
        assert!(matches!(result, Err(GitStackError::InvalidFeatureName(_))));
    }

    #[test]
    fn test_successful_branch_creation() {
        let mock = MockGitRunner::new()
            .with_branches(vec!["main".to_string()])
            .with_current_branch("main");

        let result = commands::new_branch(&mock, "new-feature");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "new-feature/1");
    }

    #[test]
    fn test_branch_creation_with_existing_stack() {
        let mock = MockGitRunner::new()
            .with_branches(vec![
                "main".to_string(),
                "feature/1".to_string(),
                "feature/2".to_string(),
            ])
            .with_current_branch("feature/2");

        let result = commands::new_branch(&mock, "feature");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "feature/3");
    }

    #[test]
    fn test_git_command_failure() {
        let mock = MockGitRunner::new().should_fail();

        let result = commands::new_branch(&mock, "test-feature");
        assert!(matches!(result, Err(GitStackError::GitCommandFailed(_))));
    }

    #[test]
    fn test_current_branch() {
        let mock = MockGitRunner::new().with_current_branch("my-branch");

        let result = commands::current_branch(&mock);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "my-branch");
    }

    #[test]
    fn test_empty_repository() {
        let mock = MockGitRunner::new().with_branches(vec![]);

        let result = commands::new_branch(&mock, "first-feature");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "first-feature/1");
    }

    #[test]
    fn test_list_stacks_empty_repo() {
        let mock = MockGitRunner::new().with_branches(vec!["main".to_string()]);

        let result = commands::list_stacks(&mock);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_stacks_with_stacks() {
        let mock = MockGitRunner::new()
            .with_branches(vec![
                "main".to_string(),
                "feature-auth/1".to_string(),
                "feature-auth/2".to_string(),
                "ui-redesign/1".to_string(),
                "other-branch".to_string(),
            ]);

        let result = commands::list_stacks(&mock);
        assert!(result.is_ok());
    }
}
