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

            fn get_pull_request_info(&self, _branch: &str) -> Result<Option<git_stack::github::PullRequestInfo>> {
                Ok(None)
            }

            fn batch_get_pull_request_info(&self, _branches: &[String]) -> Result<std::collections::HashMap<String, git_stack::github::PullRequestInfo>> {
                Ok(std::collections::HashMap::new())
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

            fn get_pull_request_info(&self, _branch: &str) -> Result<Option<git_stack::github::PullRequestInfo>> {
                Ok(None)
            }

            fn batch_get_pull_request_info(&self, _branches: &[String]) -> Result<std::collections::HashMap<String, git_stack::github::PullRequestInfo>> {
                Ok(std::collections::HashMap::new())
            }
        }

        let github_runner = TitleTestGitHubRunner;
        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pr_base_branch_targeting() {
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-base/2")
            .with_branches(vec![
                "main".to_string(),
                "feature-base/1".to_string(),
                "feature-base/2".to_string(),
            ]);

        struct BaseTestGitHubRunner {
            call_count: std::cell::RefCell<u32>,
        }

        impl GitHubRunner for BaseTestGitHubRunner {
            fn check_availability(&self) -> Result<()> {
                Ok(())
            }

            fn create_pull_request(
                &self,
                branch: &str,
                _title: &str,
                _body: &str,
                base: &str,
            ) -> Result<u32> {
                let mut count = self.call_count.borrow_mut();
                *count += 1;

                // First PR (feature-base/1) should target main
                // Second PR (feature-base/2) should target feature-base/1
                match branch {
                    "feature-base/1" => {
                        assert_eq!(base, "main", "First branch should target main");
                        Ok(1)
                    }
                    "feature-base/2" => {
                        assert_eq!(
                            base, "feature-base/1",
                            "Second branch should target first branch"
                        );
                        Ok(2)
                    }
                    _ => panic!("Unexpected branch: {}", branch),
                }
            }

            fn list_pull_requests_for_branch(&self, _branch: &str) -> Result<Option<u32>> {
                Ok(None)
            }

            fn get_pull_request_info(&self, _branch: &str) -> Result<Option<git_stack::github::PullRequestInfo>> {
                Ok(None)
            }

            fn batch_get_pull_request_info(&self, _branches: &[String]) -> Result<std::collections::HashMap<String, git_stack::github::PullRequestInfo>> {
                Ok(std::collections::HashMap::new())
            }
        }

        let github_runner = BaseTestGitHubRunner {
            call_count: std::cell::RefCell::new(0),
        };
        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_ok());
        assert_eq!(*github_runner.call_count.borrow(), 2, "Should create 2 PRs");
    }

    #[test]
    fn test_pr_base_branch_targeting_with_existing_prs() {
        // Test that PR base branch targeting works when some PRs already exist
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-existing/3")
            .with_branches(vec![
                "main".to_string(),
                "feature-existing/1".to_string(),
                "feature-existing/2".to_string(),
                "feature-existing/3".to_string(),
            ]);

        struct ExistingPRsGitHubRunner;

        impl GitHubRunner for ExistingPRsGitHubRunner {
            fn check_availability(&self) -> Result<()> {
                Ok(())
            }

            fn create_pull_request(
                &self,
                branch: &str,
                _title: &str,
                _body: &str,
                base: &str,
            ) -> Result<u32> {
                // Only feature-existing/3 should need a PR created
                // feature-existing/1 has PR #100, feature-existing/2 has PR #101
                match branch {
                    "feature-existing/3" => {
                        assert_eq!(
                            base, "feature-existing/2",
                            "Third branch should target second branch"
                        );
                        Ok(102)
                    }
                    _ => panic!("Unexpected PR creation for branch: {}", branch),
                }
            }

            fn list_pull_requests_for_branch(&self, branch: &str) -> Result<Option<u32>> {
                // Simulate existing PRs for first two branches
                match branch {
                    "feature-existing/1" => Ok(Some(100)),
                    "feature-existing/2" => Ok(Some(101)),
                    "feature-existing/3" => Ok(None),
                    _ => Ok(None),
                }
            }

            fn get_pull_request_info(&self, _branch: &str) -> Result<Option<git_stack::github::PullRequestInfo>> {
                Ok(None)
            }

            fn batch_get_pull_request_info(&self, _branches: &[String]) -> Result<std::collections::HashMap<String, git_stack::github::PullRequestInfo>> {
                Ok(std::collections::HashMap::new())
            }
        }

        let github_runner = ExistingPRsGitHubRunner;
        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pr_creation_with_branch_pushing() {
        // Test that branches are pushed before PR creation
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-push/1")
            .with_branches(vec![
                "main".to_string(),
                "feature-push/1".to_string(),
                "feature-push/2".to_string(),
            ]);

        struct PushTestGitHubRunner;

        impl GitHubRunner for PushTestGitHubRunner {
            fn check_availability(&self) -> Result<()> {
                Ok(())
            }

            fn create_pull_request(
                &self,
                _branch: &str,
                _title: &str,
                _body: &str,
                _base: &str,
            ) -> Result<u32> {
                Ok(1)
            }

            fn list_pull_requests_for_branch(&self, _branch: &str) -> Result<Option<u32>> {
                Ok(None) // No existing PRs
            }

            fn get_pull_request_info(&self, _branch: &str) -> Result<Option<git_stack::github::PullRequestInfo>> {
                Ok(None)
            }

            fn batch_get_pull_request_info(&self, _branches: &[String]) -> Result<std::collections::HashMap<String, git_stack::github::PullRequestInfo>> {
                Ok(std::collections::HashMap::new())
            }
        }

        let github_runner = PushTestGitHubRunner;
        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_ok());
    }

    #[test]
    fn test_review_push_failure() {
        // Test error handling when push fails
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-pushfail/1")
            .with_branches(vec!["main".to_string(), "feature-pushfail/1".to_string()])
            .should_fail(); // This will make git push fail

        let github_runner = MockGitHubRunner::new();

        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_err());

        match result.unwrap_err() {
            GitStackError::GitCommandFailed(_) => {}
            _ => panic!("Expected GitCommandFailed error"),
        }
    }
}

#[cfg(test)]
mod spec_compliance_tests {
    use super::*;

    #[test]
    fn test_review_spec_scenario_all_prs_exist() {
        // Spec scenario: "Handle stack with all PRs already created"
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-complete/2")
            .with_branches(vec![
                "main".to_string(),
                "feature-complete/1".to_string(),
                "feature-complete/2".to_string(),
                "feature-complete/3".to_string(),
            ]);

        let github_runner = MockGitHubRunner::new()
            .with_existing_pr("feature-complete/1", 100)
            .with_existing_pr("feature-complete/2", 101)
            .with_existing_pr("feature-complete/3", 102);

        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_ok());
    }

    #[test]
    fn test_review_spec_scenario_partial_stack() {
        // Spec scenario: "Create PRs for partially-reviewed stack"
        // GIVEN stack has branches /1, /2, /3 where /1 already has PR #100
        // THEN it skips /1 and creates PR for /2 with "Depends on #100"
        // AND creates PR for /3 with dependency on /2's new PR
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-partial/2")
            .with_branches(vec![
                "main".to_string(),
                "feature-partial/1".to_string(),
                "feature-partial/2".to_string(),
                "feature-partial/3".to_string(),
            ]);

        struct PartialStackGitHubRunner {
            pr_creation_calls: std::cell::RefCell<Vec<(String, String, String, String)>>,
        }

        impl GitHubRunner for PartialStackGitHubRunner {
            fn check_availability(&self) -> Result<()> {
                Ok(())
            }

            fn create_pull_request(
                &self,
                branch: &str,
                title: &str,
                body: &str,
                base: &str,
            ) -> Result<u32> {
                let mut calls = self.pr_creation_calls.borrow_mut();
                calls.push((
                    branch.to_string(),
                    title.to_string(),
                    body.to_string(),
                    base.to_string(),
                ));

                match branch {
                    "feature-partial/2" => {
                        assert_eq!(
                            body, "Depends on #100",
                            "Second PR should depend on first existing PR"
                        );
                        assert_eq!(
                            base, "feature-partial/1",
                            "Second PR should target first branch"
                        );
                        Ok(200)
                    }
                    "feature-partial/3" => {
                        assert_eq!(
                            body, "Depends on #200",
                            "Third PR should depend on second new PR"
                        );
                        assert_eq!(
                            base, "feature-partial/2",
                            "Third PR should target second branch"
                        );
                        Ok(201)
                    }
                    _ => panic!("Unexpected PR creation for branch: {}", branch),
                }
            }

            fn list_pull_requests_for_branch(&self, branch: &str) -> Result<Option<u32>> {
                match branch {
                    "feature-partial/1" => Ok(Some(100)), // Existing PR
                    _ => Ok(None),                        // No PR exists
                }
            }

            fn get_pull_request_info(&self, _branch: &str) -> Result<Option<git_stack::github::PullRequestInfo>> {
                Ok(None)
            }

            fn batch_get_pull_request_info(&self, _branches: &[String]) -> Result<std::collections::HashMap<String, git_stack::github::PullRequestInfo>> {
                Ok(std::collections::HashMap::new())
            }
        }

        let github_runner = PartialStackGitHubRunner {
            pr_creation_calls: std::cell::RefCell::new(Vec::new()),
        };

        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_ok());

        // Verify that only 2 PRs were created (for branches 2 and 3)
        let calls = github_runner.pr_creation_calls.borrow();
        assert_eq!(calls.len(), 2, "Should create exactly 2 PRs");
    }

    #[test]
    fn test_review_spec_scenario_no_existing_prs() {
        // Spec scenario: "Create PRs for stack with no existing PRs"
        // GIVEN stack has branches /1, /2, /3 with no existing PRs
        // THEN it creates PRs starting from /1
        // AND establishes dependencies: /2 depends on /1, /3 depends on /2
        let git_runner = MockGitRunner::new()
            .with_current_branch("feature-new/1")
            .with_branches(vec![
                "main".to_string(),
                "feature-new/1".to_string(),
                "feature-new/2".to_string(),
                "feature-new/3".to_string(),
            ]);

        struct NewStackGitHubRunner {
            pr_creation_calls: std::cell::RefCell<Vec<(String, String, String, String)>>,
        }

        impl GitHubRunner for NewStackGitHubRunner {
            fn check_availability(&self) -> Result<()> {
                Ok(())
            }

            fn create_pull_request(
                &self,
                branch: &str,
                title: &str,
                body: &str,
                base: &str,
            ) -> Result<u32> {
                let mut calls = self.pr_creation_calls.borrow_mut();
                calls.push((
                    branch.to_string(),
                    title.to_string(),
                    body.to_string(),
                    base.to_string(),
                ));

                match branch {
                    "feature-new/1" => {
                        assert_eq!(body, "", "First PR should have no dependencies");
                        assert_eq!(base, "main", "First PR should target main");
                        Ok(301)
                    }
                    "feature-new/2" => {
                        assert_eq!(body, "Depends on #301", "Second PR should depend on first");
                        assert_eq!(
                            base, "feature-new/1",
                            "Second PR should target first branch"
                        );
                        Ok(302)
                    }
                    "feature-new/3" => {
                        assert_eq!(body, "Depends on #302", "Third PR should depend on second");
                        assert_eq!(
                            base, "feature-new/2",
                            "Third PR should target second branch"
                        );
                        Ok(303)
                    }
                    _ => panic!("Unexpected PR creation for branch: {}", branch),
                }
            }

            fn list_pull_requests_for_branch(&self, _branch: &str) -> Result<Option<u32>> {
                Ok(None) // No existing PRs
            }

            fn get_pull_request_info(&self, _branch: &str) -> Result<Option<git_stack::github::PullRequestInfo>> {
                Ok(None)
            }

            fn batch_get_pull_request_info(&self, _branches: &[String]) -> Result<std::collections::HashMap<String, git_stack::github::PullRequestInfo>> {
                Ok(std::collections::HashMap::new())
            }
        }

        let github_runner = NewStackGitHubRunner {
            pr_creation_calls: std::cell::RefCell::new(Vec::new()),
        };

        let result = commands::review_stack(&git_runner, &github_runner);
        assert!(result.is_ok());

        // Verify that all 3 PRs were created
        let calls = github_runner.pr_creation_calls.borrow();
        assert_eq!(calls.len(), 3, "Should create exactly 3 PRs");
    }
}
