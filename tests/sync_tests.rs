use git_stack::{commands, branch, git, MockGitRunner};

#[cfg(test)]
mod sync_unit_tests {
    use super::*;

    #[test]
    fn test_sync_on_default_branch() {
        let git_runner = MockGitRunner::new()
            .with_branches(vec![
                "main".to_string(),
                "feature-auth/1".to_string(),
                "feature-auth/2".to_string(),
                "feature-ui/1".to_string(),
            ])
            .with_current_branch("main")
            .with_default_branch("main");

        // Should succeed from default branch
        let result = commands::sync_stacks(&git_runner);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sync_on_stack_branch() {
        let git_runner = MockGitRunner::new()
            .with_branches(vec![
                "main".to_string(),
                "feature-auth/1".to_string(),
                "feature-auth/2".to_string(),
                "feature-ui/1".to_string(),
            ])
            .with_current_branch("feature-auth/2")
            .with_default_branch("main");

        // Should succeed from stack branch
        let result = commands::sync_stacks(&git_runner);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sync_on_invalid_branch() {
        let git_runner = MockGitRunner::new()
            .with_branches(vec![
                "main".to_string(),
                "feature-auth/1".to_string(),
                "random-branch".to_string(),
            ])
            .with_current_branch("random-branch")
            .with_default_branch("main");

        // Should return an error for invalid branch context
        let result = commands::sync_stacks(&git_runner);
        assert!(result.is_err());
        
        // Verify error message mentions the invalid branch
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("random-branch"));
        assert!(error_msg.contains("Cannot sync from branch"));
    }

    #[test]
    fn test_find_first_branch_in_stack() {
        let git_runner = MockGitRunner::new()
            .with_branches(vec![
                "main".to_string(),
                "feature-auth/2".to_string(),  // Missing /1 (merged)
                "feature-auth/3".to_string(),
                "feature-auth/4".to_string(),
                "feature-ui/1".to_string(),
            ]);

        // Should find feature-auth/2 as first existing branch
        let result = branch::find_first_branch_in_stack(&git_runner, "feature-auth");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("feature-auth/2".to_string()));
    }

    #[test]
    fn test_find_first_branch_no_stack() {
        let git_runner = MockGitRunner::new()
            .with_branches(vec!["main".to_string()]);

        // Should return None for non-existent stack
        let result = branch::find_first_branch_in_stack(&git_runner, "nonexistent");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_get_stack_branches() {
        let git_runner = MockGitRunner::new()
            .with_branches(vec![
                "main".to_string(),
                "feature-auth/3".to_string(),
                "feature-auth/1".to_string(),
                "feature-auth/2".to_string(),
                "feature-ui/1".to_string(),
            ]);

        // Should return sorted branches for the stack
        let result = branch::get_stack_branches(&git_runner, "feature-auth");
        assert!(result.is_ok());
        let branches = result.unwrap();
        assert_eq!(branches, vec![
            "feature-auth/1".to_string(),
            "feature-auth/2".to_string(),
            "feature-auth/3".to_string(),
        ]);
    }

    #[test]
    fn test_has_remote_tracking() {
        let git_runner = MockGitRunner::new();

        // Test branch with remote tracking (mock returns "origin" for most branches)
        let result = git::has_remote_tracking(&git_runner, "feature-auth/1");
        assert!(result.is_ok());
        assert!(result.unwrap());

        // Test main branch (mock returns error for main)
        let result = git::has_remote_tracking(&git_runner, "main");
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should be false based on mock implementation
    }

    #[test]
    fn test_fetch_remote_with_remote() {
        let git_runner = MockGitRunner::new();

        // Should succeed when remote is available
        let result = git::fetch_remote(&git_runner);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sync_git_operations() {
        let git_runner = MockGitRunner::new();

        // Test individual git operations
        assert!(git::checkout_branch(&git_runner, "test-branch").is_ok());
        assert!(git::pull_current_branch(&git_runner).is_ok());
        assert!(git::rebase_with_update_refs(&git_runner, "main").is_ok());
    }
}

#[cfg(test)]
mod sync_integration_tests {
    use super::*;

    #[test]
    fn test_sync_context_detection() {
        // Test that sync properly detects different contexts

        // Default branch context
        let git_runner = MockGitRunner::new()
            .with_current_branch("main")
            .with_default_branch("main");

        let current_branch = git::get_current_branch(&git_runner).unwrap();
        let is_base = branch::is_base_branch(&git_runner, &current_branch).unwrap();
        let context = branch::parse_stack_branch(&current_branch);

        assert!(is_base);
        assert!(context.is_none());

        // Stack branch context
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-auth/2")
            .with_default_branch("main");

        let current_branch = git::get_current_branch(&git_runner).unwrap();
        let is_base = branch::is_base_branch(&git_runner, &current_branch).unwrap();
        let context = branch::parse_stack_branch(&current_branch);

        assert!(!is_base);
        assert!(context.is_some());
        assert_eq!(context.unwrap().feature_name, "feature-auth");
    }

    #[test]
    fn test_sync_empty_repository() {
        let git_runner = MockGitRunner::new()
            .with_branches(vec!["main".to_string()])
            .with_current_branch("main");

        // Should handle empty repository gracefully
        let result = commands::sync_stacks(&git_runner);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sync_repository_no_stacks() {
        let git_runner = MockGitRunner::new()
            .with_branches(vec![
                "main".to_string(),
                "develop".to_string(),
                "feature-branch".to_string(), // Not a stack branch
            ])
            .with_current_branch("main");

        // Should handle repository with no stack branches
        let result = commands::sync_stacks(&git_runner);
        assert!(result.is_ok());
    }
}