use git_stack::{
    commands,
    github::{GitHubRunner, MockGitHubRunner},
    GitStackError, MockGitRunner, Result,
};

#[cfg(test)]
mod review_command_tests {
    use super::*;

    #[test]
    fn test_review_from_stack_branch_success() {
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-auth/2")
            .with_branches(vec![
                "main".to_string(),
                "feature-auth/1".to_string(),
                "feature-auth/2".to_string(),
                "feature-auth/3".to_string(),
            ]);

        let github_runner = MockGitHubRunner::new().with_existing_pr("feature-auth/1", 100); // First branch already has PR

        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_ok());
    }

    #[test]
    fn test_review_from_default_branch_error() {
        let git_runner = MockGitRunner::new().with_current_branch("main");
        let github_runner = MockGitHubRunner::new();

        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_err());

        match result.unwrap_err() {
            GitStackError::InvalidContextForReview(msg) => {
                assert!(msg.contains("Cannot run review from default branch"));
            }
            _ => panic!("Expected InvalidContextForReview error"),
        }
    }

    #[test]
    fn test_review_from_non_stack_branch_error() {
        let git_runner = MockGitRunner::new().with_current_branch("random-branch");
        let github_runner = MockGitHubRunner::new();

        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_err());

        match result.unwrap_err() {
            GitStackError::InvalidContextForReview(msg) => {
                assert!(msg.contains("Cannot run review from non-stack branch"));
            }
            _ => panic!("Expected InvalidContextForReview error"),
        }
    }

    #[test]
    fn test_review_github_cli_not_available() {
        let git_runner = MockGitRunner::new().with_current_branch("feature-auth/1");
        let github_runner = MockGitHubRunner::new().with_cli_not_available();

        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_err());

        match result.unwrap_err() {
            GitStackError::GitHubCliNotFound(_) => {}
            _ => panic!("Expected GitHubCliNotFound error"),
        }
    }

    #[test]
    fn test_review_github_auth_failed() {
        let git_runner = MockGitRunner::new().with_current_branch("feature-auth/1");
        let github_runner = MockGitHubRunner::new().with_auth_failure();

        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_err());

        match result.unwrap_err() {
            GitStackError::GitHubAuthenticationFailed(_) => {}
            _ => panic!("Expected GitHubAuthenticationFailed error"),
        }
    }

    #[test]
    fn test_review_not_in_git_repo() {
        let git_runner = MockGitRunner::new().not_in_repo();
        let github_runner = MockGitHubRunner::new();

        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_err());

        match result.unwrap_err() {
            GitStackError::NotInGitRepository => {}
            _ => panic!("Expected NotInGitRepository error"),
        }
    }

    #[test]
    fn test_review_create_prs_for_new_stack() {
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-new/1")
            .with_branches(vec![
                "main".to_string(),
                "feature-new/1".to_string(),
                "feature-new/2".to_string(),
                "feature-new/3".to_string(),
            ]);

        let github_runner = MockGitHubRunner::new(); // No existing PRs

        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_ok());
    }

    #[test]
    fn test_review_partial_stack_with_existing_prs() {
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-partial/3")
            .with_branches(vec![
                "main".to_string(),
                "feature-partial/1".to_string(),
                "feature-partial/2".to_string(),
                "feature-partial/3".to_string(),
            ]);

        let github_runner = MockGitHubRunner::new()
            .with_existing_pr("feature-partial/1", 200)
            .with_existing_pr("feature-partial/2", 201);

        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_ok());
    }

    #[test]
    fn test_review_all_prs_already_exist() {
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-complete/2")
            .with_branches(vec![
                "main".to_string(),
                "feature-complete/1".to_string(),
                "feature-complete/2".to_string(),
                "feature-complete/3".to_string(),
            ]);

        let github_runner = MockGitHubRunner::new()
            .with_existing_pr("feature-complete/1", 300)
            .with_existing_pr("feature-complete/2", 301)
            .with_existing_pr("feature-complete/3", 302);

        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_ok());
    }

    #[test]
    fn test_review_pr_creation_failure() {
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-fail/1")
            .with_branches(vec!["main".to_string(), "feature-fail/1".to_string()]);

        let github_runner = MockGitHubRunner::new().with_operation_failure();

        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_err());

        match result.unwrap_err() {
            GitStackError::GitHubOperationFailed(_) => {}
            _ => panic!("Expected GitHubOperationFailed error"),
        }
    }

    #[test]
    fn test_review_empty_stack() {
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-empty/1")
            .with_branches(vec!["main".to_string()]); // Only main branch exists

        let github_runner = MockGitHubRunner::new();

        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_ok()); // Should succeed but do nothing
    }
}

#[cfg(test)]
mod github_runner_integration_tests {
    use super::*;

    #[test]
    fn test_github_runner_create_pr_mock() {
        let runner = MockGitHubRunner::new();

        let result = runner.create_pull_request("feature-test/1", "feature-test #1", "", "main");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_github_runner_list_prs_no_existing() {
        let runner = MockGitHubRunner::new();

        let result = runner.list_pull_requests_for_branch("feature-test/1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_github_runner_list_prs_with_existing() {
        let runner = MockGitHubRunner::new().with_existing_pr("feature-test/1", 42);

        let result = runner.list_pull_requests_for_branch("feature-test/1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(42));
    }

    #[test]
    fn test_github_runner_availability_check() {
        let runner = MockGitHubRunner::new();
        assert!(runner.check_availability().is_ok());

        let runner = MockGitHubRunner::new().with_cli_not_available();
        assert!(runner.check_availability().is_err());

        let runner = MockGitHubRunner::new().with_auth_failure();
        assert!(runner.check_availability().is_err());
    }
}

#[cfg(test)]
mod pr_analysis_tests {
    use super::*;

    #[test]
    fn test_pr_dependency_chain_creation() {
        // Test that PRs are created with proper dependency chain
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-chain/2")
            .with_branches(vec![
                "main".to_string(),
                "feature-chain/1".to_string(),
                "feature-chain/2".to_string(),
                "feature-chain/3".to_string(),
            ]);

        // Mock that creates PRs with incremental numbers
        struct TestGitHubRunner {
            next_pr: std::cell::RefCell<u32>,
        }

        impl GitHubRunner for TestGitHubRunner {
            fn check_availability(&self) -> Result<()> {
                Ok(())
            }

            fn create_pull_request(
                &self,
                _branch: &str,
                _title: &str,
                body: &str,
                _base: &str,
            ) -> Result<u32> {
                let pr_num = *self.next_pr.borrow();
                *self.next_pr.borrow_mut() += 1;

                // Verify dependency format in body
                if pr_num > 1 {
                    assert!(body.contains(&format!("Depends on #{}", pr_num - 1)));
                }

                Ok(pr_num)
            }

            fn list_pull_requests_for_branch(&self, _branch: &str) -> Result<Option<u32>> {
                Ok(None) // No existing PRs
            }
        }

        let github_runner = TestGitHubRunner {
            next_pr: std::cell::RefCell::new(1),
        };
        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pr_title_formatting() {
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-title/1")
            .with_branches(vec!["main".to_string(), "feature-title/1".to_string()]);

        struct TitleTestGitHubRunner;

        impl GitHubRunner for TitleTestGitHubRunner {
            fn check_availability(&self) -> Result<()> {
                Ok(())
            }

            fn create_pull_request(
                &self,
                _branch: &str,
                title: &str,
                _body: &str,
                _base: &str,
            ) -> Result<u32> {
                // Verify title format
                assert_eq!(title, "feature-title #1");
                Ok(1)
            }

            fn list_pull_requests_for_branch(&self, _branch: &str) -> Result<Option<u32>> {
                Ok(None)
            }
        }

        let github_runner = TitleTestGitHubRunner;
        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pr_base_branch_targeting() {
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-base/1")
            .with_branches(vec!["main".to_string(), "feature-base/1".to_string()]);

        struct BaseTestGitHubRunner;

        impl GitHubRunner for BaseTestGitHubRunner {
            fn check_availability(&self) -> Result<()> {
                Ok(())
            }

            fn create_pull_request(
                &self,
                _branch: &str,
                _title: &str,
                _body: &str,
                base: &str,
            ) -> Result<u32> {
                // Verify base branch is main (not the previous branch in stack)
                assert_eq!(base, "main");
                Ok(1)
            }

            fn list_pull_requests_for_branch(&self, _branch: &str) -> Result<Option<u32>> {
                Ok(None)
            }
        }

        let github_runner = BaseTestGitHubRunner;
        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_ok());
    }
}
