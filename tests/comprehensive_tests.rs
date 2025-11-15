use git_stack::{branch, commands, GitRunner, GitStackError, MockGitRunner};

/// Comprehensive tests for branch naming and creation logic with various scenarios
mod branch_naming_tests {
    use super::*;

    #[test]
    fn test_first_branch_in_stack() {
        let mock = MockGitRunner::new().with_branches(vec!["main".to_string()]);

        let result = commands::new_branch(&mock, "auth");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "auth/1");
    }

    #[test]
    fn test_subsequent_branches_in_stack() {
        let mock = MockGitRunner::new().with_branches(vec![
            "main".to_string(),
            "auth/1".to_string(),
            "auth/2".to_string(),
        ]);

        let result = commands::new_branch(&mock, "auth");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "auth/3");
    }

    #[test]
    fn test_multiple_different_feature_stacks() {
        let mock = MockGitRunner::new().with_branches(vec![
            "main".to_string(),
            "auth/1".to_string(),
            "auth/2".to_string(),
            "ui/1".to_string(),
            "api/1".to_string(),
            "api/2".to_string(),
            "api/3".to_string(),
        ]);

        // Adding to auth stack
        let result = commands::new_branch(&mock, "auth");
        assert_eq!(result.unwrap(), "auth/3");

        // Adding to ui stack
        let result = commands::new_branch(&mock, "ui");
        assert_eq!(result.unwrap(), "ui/2");

        // Adding to api stack
        let result = commands::new_branch(&mock, "api");
        assert_eq!(result.unwrap(), "api/4");

        // Starting new feature stack
        let result = commands::new_branch(&mock, "database");
        assert_eq!(result.unwrap(), "database/1");
    }

    #[test]
    fn test_non_sequential_branch_numbers() {
        let mock = MockGitRunner::new().with_branches(vec![
            "main".to_string(),
            "feature/1".to_string(),
            "feature/3".to_string(),
            "feature/5".to_string(),
            "feature/10".to_string(),
        ]);

        let result = commands::new_branch(&mock, "feature");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "feature/11"); // Should be max + 1
    }

    #[test]
    fn test_invalid_branch_numbers_ignored() {
        let mock = MockGitRunner::new().with_branches(vec![
            "main".to_string(),
            "feature/1".to_string(),
            "feature/abc".to_string(), // Invalid number
            "feature/2".to_string(),
            "feature/".to_string(),   // No number
            "feature/3x".to_string(), // Invalid number
        ]);

        let result = commands::new_branch(&mock, "feature");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "feature/3"); // Only counts 1, 2
    }

    #[test]
    fn test_complex_feature_names() {
        let mock = MockGitRunner::new().with_branches(vec![
            "main".to_string(),
            "user-auth-v2/1".to_string(),
            "api_redesign/1".to_string(),
            "api_redesign/2".to_string(),
        ]);

        // Test underscore names
        let result = commands::new_branch(&mock, "api_redesign");
        assert_eq!(result.unwrap(), "api_redesign/3");

        // Test hyphenated names
        let result = commands::new_branch(&mock, "user-auth-v2");
        assert_eq!(result.unwrap(), "user-auth-v2/2");

        // Test mixed alphanumeric
        let result = commands::new_branch(&mock, "feature123");
        assert_eq!(result.unwrap(), "feature123/1");
    }
}

/// Tests for error scenarios that are hard to reproduce with real git
mod error_scenario_tests {
    use super::*;

    #[test]
    fn test_not_in_git_repository() {
        let mock = MockGitRunner::new().not_in_repo();

        let result = commands::new_branch(&mock, "feature");
        assert!(matches!(result, Err(GitStackError::NotInGitRepository)));
    }

    #[test]
    fn test_git_command_failures() {
        let mock = MockGitRunner::new().should_fail();

        let result = commands::new_branch(&mock, "feature");
        assert!(matches!(result, Err(GitStackError::GitCommandFailed(_))));
    }

    #[test]
    fn test_invalid_feature_names() {
        let mock = MockGitRunner::new();

        // Empty name
        let result = commands::new_branch(&mock, "");
        assert!(matches!(result, Err(GitStackError::InvalidFeatureName(_))));

        // Spaces
        let result = commands::new_branch(&mock, "invalid name");
        assert!(matches!(result, Err(GitStackError::InvalidFeatureName(_))));

        // Special characters
        let result = commands::new_branch(&mock, "feature@name");
        assert!(matches!(result, Err(GitStackError::InvalidFeatureName(_))));

        // Dots
        let result = commands::new_branch(&mock, "feature.name");
        assert!(matches!(result, Err(GitStackError::InvalidFeatureName(_))));
    }

    #[test]
    fn test_partial_git_failures() {
        // Mock that fails only on branch creation, not on listing
        let mut mock = MockGitRunner::new();
        mock.should_fail = true;

        // Should fail when trying to create the branch
        let result = commands::new_branch(&mock, "feature");
        assert!(matches!(result, Err(GitStackError::GitCommandFailed(_))));
    }
}

/// Tests for edge cases and boundary conditions
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_repository() {
        let mock = MockGitRunner::new().with_branches(vec![]);

        let result = commands::new_branch(&mock, "first-feature");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "first-feature/1");
    }

    #[test]
    fn test_repository_with_only_main() {
        let mock = MockGitRunner::new().with_branches(vec!["main".to_string()]);

        let result = commands::new_branch(&mock, "feature");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "feature/1");
    }

    #[test]
    fn test_large_branch_numbers() {
        let mock = MockGitRunner::new().with_branches(vec![
            "main".to_string(),
            "feature/999".to_string(),
            "feature/1000".to_string(),
        ]);

        let result = commands::new_branch(&mock, "feature");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "feature/1001");
    }

    #[test]
    fn test_single_character_feature_names() {
        let mock = MockGitRunner::new().with_branches(vec![
            "main".to_string(),
            "a/1".to_string(),
            "b/1".to_string(),
        ]);

        let result = commands::new_branch(&mock, "a");
        assert_eq!(result.unwrap(), "a/2");

        let result = commands::new_branch(&mock, "c");
        assert_eq!(result.unwrap(), "c/1");
    }

    #[test]
    fn test_numeric_feature_names() {
        let mock = MockGitRunner::new().with_branches(vec![
            "main".to_string(),
            "123/1".to_string(),
            "456/1".to_string(),
            "456/2".to_string(),
        ]);

        let result = commands::new_branch(&mock, "123");
        assert_eq!(result.unwrap(), "123/2");

        let result = commands::new_branch(&mock, "789");
        assert_eq!(result.unwrap(), "789/1");
    }
}

/// Tests for branch generation logic
mod branch_generation_tests {
    use super::*;

    #[test]
    fn test_branch_generate_name_direct() {
        let mock = MockGitRunner::new().with_branches(vec![
            "main".to_string(),
            "feature/1".to_string(),
            "feature/2".to_string(),
        ]);

        let result = branch::generate_name(&mock, "feature");
        assert_eq!(result.unwrap(), "feature/3");

        let result = branch::generate_name(&mock, "newfeature");
        assert_eq!(result.unwrap(), "newfeature/1");
    }

    #[test]
    fn test_branch_get_next_index_direct() {
        let mock = MockGitRunner::new().with_branches(vec![
            "main".to_string(),
            "auth/1".to_string(),
            "auth/2".to_string(),
            "ui/1".to_string(),
        ]);

        let result = branch::get_next_index(&mock, "auth");
        assert_eq!(result.unwrap(), 3);

        let result = branch::get_next_index(&mock, "ui");
        assert_eq!(result.unwrap(), 2);

        let result = branch::get_next_index(&mock, "api");
        assert_eq!(result.unwrap(), 1);
    }
}

/// Performance and stress tests (that would be impossible with real git)
mod performance_tests {
    use super::*;

    #[test]
    fn test_many_branches_performance() {
        // Create a mock with many branches - this would be slow with real git!
        let mut branches = vec!["main".to_string()];
        for i in 1..=1000 {
            branches.push(format!("feature/{}", i));
        }
        for i in 1..=500 {
            branches.push(format!("hotfix/{}", i));
        }

        let mock = MockGitRunner::new().with_branches(branches);

        // This should be instant with mock, but slow with real git
        let result = commands::new_branch(&mock, "feature");
        assert_eq!(result.unwrap(), "feature/1001");

        let result = commands::new_branch(&mock, "newstack");
        assert_eq!(result.unwrap(), "newstack/1");
    }

    #[test]
    fn test_branch_name_collision_avoidance() {
        // Test that we handle feature names that are substrings of each other
        let mock = MockGitRunner::new().with_branches(vec![
            "main".to_string(),
            "auth/1".to_string(),
            "auth/2".to_string(),
            "authentication/1".to_string(),
            "auth-service/1".to_string(),
        ]);

        let result = commands::new_branch(&mock, "auth");
        assert_eq!(result.unwrap(), "auth/3");

        let result = commands::new_branch(&mock, "authentication");
        assert_eq!(result.unwrap(), "authentication/2");

        let result = commands::new_branch(&mock, "auth-service");
        assert_eq!(result.unwrap(), "auth-service/2");
    }
}
