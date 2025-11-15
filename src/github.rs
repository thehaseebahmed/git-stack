//! GitHub CLI integration module
//!
//! This module provides a trait-based abstraction for GitHub CLI operations,
//! following the same pattern as the GitRunner trait.

use crate::{GitStackError, Result};
use serde_json;
use std::process::Command;

/// Trait for executing GitHub CLI commands - allows for dependency injection
pub trait GitHubRunner {
    /// Check if GitHub CLI is available and authenticated
    fn check_availability(&self) -> Result<()>;

    /// Create a pull request and return the PR number
    fn create_pull_request(&self, branch: &str, title: &str, body: &str, base: &str)
        -> Result<u32>;

    /// List pull requests for a specific branch and return the PR number if it exists
    fn list_pull_requests_for_branch(&self, branch: &str) -> Result<Option<u32>>;
}

/// Real GitHub CLI command runner for production use
pub struct RealGitHubRunner;

impl GitHubRunner for RealGitHubRunner {
    fn check_availability(&self) -> Result<()> {
        // Check if gh command is available
        let output = Command::new("gh")
            .args(["--version"])
            .output()
            .map_err(|e| {
                GitStackError::GitHubCliNotFound(format!(
                    "GitHub CLI (gh) not found: {}. Please install it from https://cli.github.com/",
                    e
                ))
            })?;

        if !output.status.success() {
            return Err(GitStackError::GitHubCliNotFound(
                "GitHub CLI (gh) not found. Please install it from https://cli.github.com/"
                    .to_string(),
            ));
        }

        // Check authentication
        let auth_output = Command::new("gh")
            .args(["auth", "status"])
            .output()
            .map_err(|e| {
                GitStackError::GitHubAuthenticationFailed(format!(
                    "Failed to check GitHub authentication: {}",
                    e
                ))
            })?;

        if !auth_output.status.success() {
            return Err(GitStackError::GitHubAuthenticationFailed(
                "Not authenticated with GitHub. Please run 'gh auth login' to authenticate."
                    .to_string(),
            ));
        }

        Ok(())
    }

    fn create_pull_request(
        &self,
        branch: &str,
        title: &str,
        body: &str,
        base: &str,
    ) -> Result<u32> {
        let output = Command::new("gh")
            .args([
                "pr", "create", "--head", branch, "--base", base, "--title", title, "--body", body,
            ])
            .output()
            .map_err(|e| {
                GitStackError::GitHubOperationFailed(format!(
                    "Failed to create pull request: {}",
                    e
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GitStackError::GitHubOperationFailed(format!(
                "Failed to create pull request: {}",
                stderr
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Parse PR number from URL in output (e.g., "https://github.com/owner/repo/pull/123")
        for line in stdout.lines() {
            if line.contains("/pull/") {
                if let Some(pr_num_str) = line.split("/pull/").nth(1) {
                    if let Ok(pr_num) = pr_num_str.trim().parse::<u32>() {
                        return Ok(pr_num);
                    }
                }
            }
        }

        Err(GitStackError::GitHubOperationFailed(
            "Could not parse PR number from GitHub CLI output".to_string(),
        ))
    }

    fn list_pull_requests_for_branch(&self, branch: &str) -> Result<Option<u32>> {
        let output = Command::new("gh")
            .args([
                "pr", "list", "--head", branch, "--json", "number", "--limit", "1",
            ])
            .output()
            .map_err(|e| {
                GitStackError::GitHubOperationFailed(format!("Failed to list pull requests: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GitStackError::GitHubOperationFailed(format!(
                "Failed to list pull requests: {}",
                stderr
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Parse JSON output to extract PR number
        if stdout.trim().is_empty() || stdout.trim() == "[]" {
            return Ok(None);
        }

        // Parse JSON using serde_json
        match serde_json::from_str::<Vec<serde_json::Value>>(&stdout) {
            Ok(prs) => {
                if let Some(pr) = prs.first() {
                    if let Some(number) = pr.get("number") {
                        if let Some(pr_num) = number.as_u64() {
                            return Ok(Some(pr_num as u32));
                        }
                    }
                }
                Ok(None)
            }
            Err(e) => {
                Err(GitStackError::GitHubOperationFailed(format!(
                    "Failed to parse GitHub CLI JSON response: {}",
                    e
                )))
            }
        }
    }
}

/// Mock GitHub CLI runner for testing
#[derive(Default)]
pub struct MockGitHubRunner {
    pub should_fail_availability: bool,
    pub should_fail_auth: bool,
    pub should_fail_operations: bool,
    pub existing_prs: std::collections::HashMap<String, u32>,
    pub next_pr_number: u32,
}

impl MockGitHubRunner {
    pub fn new() -> Self {
        Self {
            should_fail_availability: false,
            should_fail_auth: false,
            should_fail_operations: false,
            existing_prs: std::collections::HashMap::new(),
            next_pr_number: 1,
        }
    }

    pub fn with_cli_not_available(mut self) -> Self {
        self.should_fail_availability = true;
        self
    }

    pub fn with_auth_failure(mut self) -> Self {
        self.should_fail_auth = true;
        self
    }

    pub fn with_operation_failure(mut self) -> Self {
        self.should_fail_operations = true;
        self
    }

    pub fn with_existing_pr(mut self, branch: &str, pr_number: u32) -> Self {
        self.existing_prs.insert(branch.to_string(), pr_number);
        if pr_number >= self.next_pr_number {
            self.next_pr_number = pr_number + 1;
        }
        self
    }
}

impl GitHubRunner for MockGitHubRunner {
    fn check_availability(&self) -> Result<()> {
        if self.should_fail_availability {
            return Err(GitStackError::GitHubCliNotFound(
                "GitHub CLI (gh) not found. Please install it from https://cli.github.com/"
                    .to_string(),
            ));
        }

        if self.should_fail_auth {
            return Err(GitStackError::GitHubAuthenticationFailed(
                "Not authenticated with GitHub. Please run 'gh auth login' to authenticate."
                    .to_string(),
            ));
        }

        Ok(())
    }

    fn create_pull_request(
        &self,
        _branch: &str,
        _title: &str,
        _body: &str,
        _base: &str,
    ) -> Result<u32> {
        if self.should_fail_operations {
            return Err(GitStackError::GitHubOperationFailed(
                "Mock PR creation failure".to_string(),
            ));
        }

        let pr_number = self.next_pr_number;
        // In a real mock, we'd update the existing_prs map, but since we can't mutate self,
        // we'll just return the next number. Tests should set up existing PRs beforehand.
        Ok(pr_number)
    }

    fn list_pull_requests_for_branch(&self, branch: &str) -> Result<Option<u32>> {
        if self.should_fail_operations {
            return Err(GitStackError::GitHubOperationFailed(
                "Mock PR listing failure".to_string(),
            ));
        }

        Ok(self.existing_prs.get(branch).copied())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_github_runner_availability() {
        let runner = MockGitHubRunner::new();
        assert!(runner.check_availability().is_ok());

        let runner = MockGitHubRunner::new().with_cli_not_available();
        assert!(runner.check_availability().is_err());

        let runner = MockGitHubRunner::new().with_auth_failure();
        assert!(runner.check_availability().is_err());
    }

    #[test]
    fn test_mock_github_runner_pr_operations() {
        let runner = MockGitHubRunner::new();

        // Test creating PR
        let result = runner.create_pull_request("feature/1", "Feature #1", "Description", "main");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        // Test listing non-existing PR
        let result = runner.list_pull_requests_for_branch("feature/1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);

        // Test listing existing PR
        let runner = runner.with_existing_pr("feature/2", 42);
        let result = runner.list_pull_requests_for_branch("feature/2");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(42));
    }

    #[test]
    fn test_mock_github_runner_operation_failures() {
        let runner = MockGitHubRunner::new().with_operation_failure();

        assert!(runner
            .create_pull_request("feature/1", "Feature #1", "Description", "main")
            .is_err());
        assert!(runner.list_pull_requests_for_branch("feature/1").is_err());
    }
}
