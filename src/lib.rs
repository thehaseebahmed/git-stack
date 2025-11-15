//! Git Stack - A tool for managing stacked git branches
//!
//! This library provides functionality for creating and managing stacked git branches
//! with structured naming patterns.
//!
//! # Examples
//!
//! Create a new stacked branch:
//! ```rust,no_run
//! use git_stack::{commands, RealGitRunner};
//!
//! let git_runner = RealGitRunner;
//! match commands::new_branch(&git_runner, "my-feature") {
//!     Ok(branch_name) => println!("Created: {}", branch_name),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//!
//! # Modules
//!
//! - [`git`] - Low-level git operations
//! - [`branch`] - Branch naming and validation
//! - [`commands`] - High-level command operations

use std::fmt;
use std::process::Command;

/// Custom error types for git-stack operations
#[derive(Debug, PartialEq)]
pub enum GitStackError {
    InvalidFeatureName(String),
    NotInGitRepository,
    GitCommandFailed(String),
    IoError(String),
    FeatureNameRequiredOnBaseBranch(String),
    CannotStartNewStackFromDiff { current_branch: String, attempted_feature: String },
}

impl fmt::Display for GitStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitStackError::InvalidFeatureName(name) => {
                write!(f, "Invalid feature name '{}': Feature names must contain only alphanumeric characters, hyphens, and underscores", name)
            }
            GitStackError::NotInGitRepository => {
                write!(f, "Not in a git repository. Please run this command from within a git repository.")
            }
            GitStackError::GitCommandFailed(message) => {
                write!(f, "Git command failed: {}", message)
            }
            GitStackError::IoError(message) => {
                write!(f, "I/O error: {}", message)
            }
            GitStackError::FeatureNameRequiredOnBaseBranch(branch) => {
                write!(f, "Feature name is required when creating a new stack from base branch '{}'. Usage: git-stack new <feature-name>", branch)
            }
            GitStackError::CannotStartNewStackFromDiff { current_branch, attempted_feature } => {
                write!(f, "Cannot start new stack '{}' from existing stack branch '{}'. To start a new stack, first return to a base branch (like 'main') with: git checkout main", attempted_feature, current_branch)
            }
        }
    }
}

impl std::error::Error for GitStackError {}

pub type Result<T> = std::result::Result<T, GitStackError>;

/// Trait for executing git commands - allows for dependency injection
pub trait GitRunner {
    /// Execute a git command and return the output as a string
    fn run_command(&self, args: &[&str]) -> Result<String>;

    /// Check if the current directory is inside a git repository
    fn is_repository(&self) -> Result<bool>;
}

/// Real git command runner for production use
pub struct RealGitRunner;

impl GitRunner for RealGitRunner {
    fn run_command(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("git")
            .args(args)
            .output()
            .map_err(|e| GitStackError::IoError(format!("Failed to execute git command: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GitStackError::GitCommandFailed(stderr.to_string()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.trim().to_string())
    }

    fn is_repository(&self) -> Result<bool> {
        let output = Command::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .output()
            .map_err(|e| GitStackError::IoError(format!("Failed to execute git command: {}", e)))?;

        Ok(output.status.success())
    }
}

/// Mock git runner for testing
pub struct MockGitRunner {
    pub branches: Vec<String>,
    pub current_branch: String,
    pub default_branch: String,
    pub is_repo: bool,
    pub should_fail: bool,
    pub expected_commands: Vec<Vec<String>>,
    pub command_count: std::cell::RefCell<usize>,
}

impl MockGitRunner {
    pub fn new() -> Self {
        Self {
            branches: vec!["main".to_string()],
            current_branch: "main".to_string(),
            default_branch: "main".to_string(),
            is_repo: true,
            should_fail: false,
            expected_commands: vec![],
            command_count: std::cell::RefCell::new(0),
        }
    }

    pub fn with_branches(mut self, branches: Vec<String>) -> Self {
        self.branches = branches;
        self
    }

    pub fn with_current_branch(mut self, branch: &str) -> Self {
        self.current_branch = branch.to_string();
        self
    }

    pub fn with_default_branch(mut self, branch: &str) -> Self {
        self.default_branch = branch.to_string();
        self
    }

    pub fn not_in_repo(mut self) -> Self {
        self.is_repo = false;
        self
    }

    pub fn should_fail(mut self) -> Self {
        self.should_fail = true;
        self
    }
}

impl GitRunner for MockGitRunner {
    fn run_command(&self, args: &[&str]) -> Result<String> {
        if self.should_fail {
            return Err(GitStackError::GitCommandFailed(
                "Mock git failure".to_string(),
            ));
        }

        let mut count = self.command_count.borrow_mut();
        *count += 1;

        match args {
            ["branch", "--list", "--format=%(refname:short)"] => {
                if self.branches.is_empty() {
                    Ok(String::new())
                } else {
                    Ok(self.branches.join("\n"))
                }
            }
            ["rev-parse", "--abbrev-ref", "HEAD"] => Ok(self.current_branch.clone()),
            ["checkout", "-b", branch_name] => {
                Ok(format!("Switched to a new branch '{}'", branch_name))
            }
            ["symbolic-ref", "refs/remotes/origin/HEAD"] => {
                Ok(format!("refs/remotes/origin/{}", self.default_branch))
            }
            ["config", "--get", "init.defaultBranch"] => {
                Ok(self.default_branch.clone())
            }
            _ => Ok(format!("Mock response for: git {}", args.join(" "))),
        }
    }

    fn is_repository(&self) -> Result<bool> {
        Ok(self.is_repo)
    }
}

/// Git operations module
pub mod git {
    use super::*;

    /// Verify that we're running inside a git repository
    pub fn check_repository(git_runner: &dyn GitRunner) -> Result<()> {
        if !git_runner.is_repository()? {
            return Err(GitStackError::NotInGitRepository);
        }
        Ok(())
    }

    /// Get the name of the current git branch
    pub fn get_current_branch(git_runner: &dyn GitRunner) -> Result<String> {
        git_runner.run_command(&["rev-parse", "--abbrev-ref", "HEAD"])
    }

    /// Get the default branch name of the repository
    pub fn get_default_branch(git_runner: &dyn GitRunner) -> Result<String> {
        // Try to get the default branch from remote HEAD first
        if let Ok(remote_head) = git_runner.run_command(&["symbolic-ref", "refs/remotes/origin/HEAD"]) {
            if let Some(branch) = remote_head.strip_prefix("refs/remotes/origin/") {
                return Ok(branch.to_string());
            }
        }
        
        // Fallback: try to get from init.defaultBranch config
        if let Ok(config_default) = git_runner.run_command(&["config", "--get", "init.defaultBranch"]) {
            if !config_default.is_empty() {
                return Ok(config_default);
            }
        }
        
        // Final fallback: assume "main" (modern git default)
        Ok("main".to_string())
    }

    /// Get a list of all local branches in the repository
    pub fn list_branches(git_runner: &dyn GitRunner) -> Result<Vec<String>> {
        let output = git_runner.run_command(&["branch", "--list", "--format=%(refname:short)"])?;
        if output.is_empty() {
            return Ok(vec![]);
        }

        Ok(output.lines().map(|line| line.trim().to_string()).collect())
    }

    /// Create a new git branch and switch to it
    pub fn create_branch(git_runner: &dyn GitRunner, branch_name: &str) -> Result<()> {
        git_runner.run_command(&["checkout", "-b", branch_name])?;
        Ok(())
    }
}

/// Branch naming and validation module
pub mod branch {
    use super::*;

    /// Validate that a feature name contains only allowed characters
    pub fn validate_feature_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(GitStackError::InvalidFeatureName(
                "Feature name cannot be empty".to_string(),
            ));
        }

        // Check for valid git branch name characters
        for char in name.chars() {
            if !char.is_alphanumeric() && char != '-' && char != '_' {
                return Err(GitStackError::InvalidFeatureName(name.to_string()));
            }
        }

        Ok(())
    }

    /// Calculate the next branch index for a given feature name from a list of branches
    pub fn get_next_index_from_branches(feature_name: &str, branches: &[String]) -> u32 {
        let prefix = format!("{}/", feature_name);
        let mut max_index = 0;

        for branch in branches {
            if branch.starts_with(&prefix) {
                // Extract the index part after the prefix
                if let Some(index_str) = branch.strip_prefix(&prefix) {
                    if let Ok(index) = index_str.parse::<u32>() {
                        if index > max_index {
                            max_index = index;
                        }
                    }
                }
            }
        }

        max_index + 1
    }

    /// Get the next available branch index for a feature by examining existing branches
    pub fn get_next_index(git_runner: &dyn GitRunner, feature_name: &str) -> Result<u32> {
        let branches = crate::git::list_branches(git_runner)?;
        Ok(get_next_index_from_branches(feature_name, &branches))
    }

    /// Generate a new branch name using the pattern "feature-name/index"
    pub fn generate_name(git_runner: &dyn GitRunner, feature_name: &str) -> Result<String> {
        let index = get_next_index(git_runner, feature_name)?;
        Ok(format!("{}/{}", feature_name, index))
    }

    /// Information about a stack branch
    #[derive(Debug, PartialEq, Clone)]
    pub struct StackInfo {
        pub feature_name: String,
        pub index: u32,
    }

    /// Parse a branch name to extract stack information if it follows the pattern
    pub fn parse_stack_branch(branch_name: &str) -> Option<StackInfo> {
        // Look for the pattern "feature-name/index" where index is a number
        if let Some(slash_pos) = branch_name.rfind('/') {
            let feature_name = &branch_name[..slash_pos];
            let index_str = &branch_name[slash_pos + 1..];
            
            // Validate feature name part
            if feature_name.is_empty() || validate_feature_name(feature_name).is_err() {
                return None;
            }
            
            // Try to parse index as a positive number
            if let Ok(index) = index_str.parse::<u32>() {
                if index > 0 {
                    return Some(StackInfo {
                        feature_name: feature_name.to_string(),
                        index,
                    });
                }
            }
        }
        None
    }

    /// Check if a branch name is considered a base branch (the default branch)
    pub fn is_base_branch(git_runner: &dyn GitRunner, branch_name: &str) -> Result<bool> {
        let default_branch = crate::git::get_default_branch(git_runner)?;
        Ok(branch_name == default_branch)
    }

    /// Get the current branch context
    pub fn get_current_context(git_runner: &dyn GitRunner) -> Result<Option<StackInfo>> {
        let current_branch = crate::git::get_current_branch(git_runner)?;
        Ok(parse_stack_branch(&current_branch))
    }
}

/// High-level operations for git-stack commands
pub mod commands {
    use super::*;

    /// Create a new stacked branch for the given feature name
    pub fn new_branch(git_runner: &dyn GitRunner, feature_name: &str) -> Result<String> {
        git::check_repository(git_runner)?;
        branch::validate_feature_name(feature_name)?;

        let new_branch_name = branch::generate_name(git_runner, feature_name)?;
        git::create_branch(git_runner, &new_branch_name)?;

        Ok(new_branch_name)
    }

    /// Context-aware new branch creation
    pub fn new_branch_contextual(
        git_runner: &dyn GitRunner,
        feature_name: Option<&str>,
    ) -> Result<String> {
        git::check_repository(git_runner)?;

        let current_branch = git::get_current_branch(git_runner)?;
        let current_context = branch::parse_stack_branch(&current_branch);
        let is_on_base_branch = branch::is_base_branch(git_runner, &current_branch)?;

        match (is_on_base_branch, current_context, feature_name) {
            // Case 1: On base branch with feature name - create new stack (existing behavior)
            (true, _, Some(name)) => {
                // Treat "." as invalid on base branch
                if name == "." {
                    return Err(GitStackError::FeatureNameRequiredOnBaseBranch(
                        current_branch,
                    ));
                }
                new_branch(git_runner, name)
            }

            // Case 2: On base branch without feature name - require feature name
            (true, _, None) => Err(GitStackError::FeatureNameRequiredOnBaseBranch(
                current_branch,
            )),

            // Case 3: On stack branch with different feature name - prevent new stack from diff
            (false, Some(stack_info), Some(name)) => {
                if name != "." && name != stack_info.feature_name {
                    Err(GitStackError::CannotStartNewStackFromDiff {
                        current_branch,
                        attempted_feature: name.to_string(),
                    })
                } else {
                    // Continue current stack (feature name matches or is ".")
                    continue_current_stack(git_runner, &stack_info)
                }
            }

            // Case 4: On stack branch without feature name - continue current stack
            (false, Some(stack_info), None) => continue_current_stack(git_runner, &stack_info),

            // Case 5: On non-base, non-stack branch - treat as base branch for safety
            (false, None, Some(name)) => {
                if name == "." {
                    return Err(GitStackError::FeatureNameRequiredOnBaseBranch(
                        current_branch,
                    ));
                }
                new_branch(git_runner, name)
            }

            // Case 6: On non-base, non-stack branch without feature name
            (false, None, None) => Err(GitStackError::FeatureNameRequiredOnBaseBranch(
                current_branch,
            )),
        }
    }

    /// Continue the current stack by creating the next branch
    fn continue_current_stack(git_runner: &dyn GitRunner, stack_info: &branch::StackInfo) -> Result<String> {
        let next_index = branch::get_next_index(git_runner, &stack_info.feature_name)?;
        let new_branch_name = format!("{}/{}", stack_info.feature_name, next_index);
        git::create_branch(git_runner, &new_branch_name)?;
        Ok(new_branch_name)
    }

    /// Get the current branch name
    pub fn current_branch(git_runner: &dyn GitRunner) -> Result<String> {
        git::get_current_branch(git_runner)
    }
}
