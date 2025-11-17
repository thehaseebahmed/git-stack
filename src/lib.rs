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
//! - [`github`] - GitHub CLI integration

use std::fmt;
use std::process::Command;

pub mod github;

/// Custom error types for git-stack operations
#[derive(Debug, PartialEq)]
pub enum GitStackError {
    InvalidFeatureName(String),
    NotInGitRepository,
    GitCommandFailed(String),
    IoError(String),
    FeatureNameRequiredOnBaseBranch(String),
    CannotStartNewStackFromDiff {
        current_branch: String,
        attempted_feature: String,
    },
    InvalidBranchForSync(String),
    GitHubCliNotFound(String),
    GitHubAuthenticationFailed(String),
    GitHubOperationFailed(String),
    InvalidContextForReview(String),
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
            GitStackError::CannotStartNewStackFromDiff {
                current_branch,
                attempted_feature,
            } => {
                write!(f, "Cannot start new stack '{}' from existing stack branch '{}'. To start a new stack, first return to a base branch (like 'main') with: git checkout main", attempted_feature, current_branch)
            }
            GitStackError::InvalidBranchForSync(branch) => {
                write!(f, "Cannot sync from branch '{}'. Please switch to a default branch (like 'main') or a stack branch to run sync.", branch)
            }
            GitStackError::GitHubCliNotFound(message) => {
                write!(f, "{}", message)
            }
            GitStackError::GitHubAuthenticationFailed(message) => {
                write!(f, "{}", message)
            }
            GitStackError::GitHubOperationFailed(message) => {
                write!(f, "GitHub operation failed: {}", message)
            }
            GitStackError::InvalidContextForReview(message) => {
                write!(f, "Cannot run review command: {}", message)
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
#[derive(Default)]
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
            ["checkout", branch_name] => Ok(format!("Switched to branch '{}'", branch_name)),
            ["symbolic-ref", "refs/remotes/origin/HEAD"] => {
                Ok(format!("refs/remotes/origin/{}", self.default_branch))
            }
            ["config", "--get", "init.defaultBranch"] => Ok(self.default_branch.clone()),
            ["fetch", "origin"] => Ok("Mock fetch completed".to_string()),
            ["remote", "get-url", "origin"] => Ok("https://github.com/test/repo.git".to_string()),
            ["pull"] => Ok("Already up to date.".to_string()),
            ["push", "-u", "origin", branch_name] => Ok(format!(
                "Branch '{}' set up to track remote branch",
                branch_name
            )),
            ["rebase", "--update-refs", _] => Ok("Successfully rebased".to_string()),
            ["config", "--get", key] if key.starts_with("branch.") => {
                // For remote tracking branches, return "origin" for most branches
                if key.ends_with(".remote") && !key.contains("main") {
                    Ok("origin".to_string())
                } else {
                    Err(GitStackError::GitCommandFailed("Not found".to_string()))
                }
            }
            ["rev-parse", "--abbrev-ref", upstream] if upstream.contains("@{upstream}") => {
                // For upstream branches, return a mock upstream
                let branch_name = upstream.replace("@{upstream}", "");
                Ok(format!("origin/{}", branch_name))
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
        if let Ok(remote_head) =
            git_runner.run_command(&["symbolic-ref", "refs/remotes/origin/HEAD"])
        {
            if let Some(branch) = remote_head.strip_prefix("refs/remotes/origin/") {
                return Ok(branch.to_string());
            }
        }

        // Fallback: try to get from init.defaultBranch config
        if let Ok(config_default) =
            git_runner.run_command(&["config", "--get", "init.defaultBranch"])
        {
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

    /// Fetch latest changes from the remote repository
    pub fn fetch_remote(git_runner: &dyn GitRunner) -> Result<()> {
        // First check if a remote exists
        if has_remote(git_runner)? {
            git_runner.run_command(&["fetch", "origin"])?;
            Ok(())
        } else {
            // Repository has no remote - this is not an error, just log and continue
            println!("Warning: No remote repository configured. Skipping fetch operation.");
            Ok(())
        }
    }

    /// Check if the repository has a remote configured
    pub fn has_remote(git_runner: &dyn GitRunner) -> Result<bool> {
        match git_runner.run_command(&["remote", "get-url", "origin"]) {
            Ok(_) => Ok(true),
            Err(GitStackError::GitCommandFailed(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Check out a specific branch
    pub fn checkout_branch(git_runner: &dyn GitRunner, branch_name: &str) -> Result<()> {
        git_runner.run_command(&["checkout", branch_name])?;
        Ok(())
    }

    /// Pull changes for the current branch
    pub fn pull_current_branch(git_runner: &dyn GitRunner) -> Result<()> {
        git_runner.run_command(&["pull"])?;
        Ok(())
    }

    /// Check if a specific branch has a remote tracking branch
    pub fn has_remote_tracking(git_runner: &dyn GitRunner, branch_name: &str) -> Result<bool> {
        // Use git config to check if the branch has a remote tracking branch
        let config_key = format!("branch.{}.remote", branch_name);
        match git_runner.run_command(&["config", "--get", &config_key]) {
            Ok(remote) => Ok(!remote.trim().is_empty()),
            Err(GitStackError::GitCommandFailed(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Get the remote tracking branch name for a local branch
    pub fn get_remote_tracking_branch(
        git_runner: &dyn GitRunner,
        branch_name: &str,
    ) -> Result<Option<String>> {
        match git_runner.run_command(&[
            "rev-parse",
            "--abbrev-ref",
            &format!("{}@{{upstream}}", branch_name),
        ]) {
            Ok(upstream) => Ok(Some(upstream.trim().to_string())),
            Err(GitStackError::GitCommandFailed(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Perform rebase with --update-refs to update all branches in a stack
    pub fn rebase_with_update_refs(git_runner: &dyn GitRunner, _base_branch: &str) -> Result<()> {
        let default_branch = get_default_branch(git_runner)?;
        let rebase_onto = if has_remote(git_runner)? {
            format!("origin/{}", default_branch)
        } else {
            default_branch
        };

        git_runner.run_command(&["rebase", "--update-refs", &rebase_onto])?;
        Ok(())
    }

    /// Push a specific branch to remote origin
    pub fn push_branch(git_runner: &dyn GitRunner, branch_name: &str) -> Result<()> {
        // Push the branch to origin, creating it if it doesn't exist
        git_runner.run_command(&["push", "-u", "origin", branch_name])?;
        Ok(())
    }

    /// Pull changes for a specific branch (checkout, pull, return to original branch)
    pub fn pull_branch(
        git_runner: &dyn GitRunner,
        branch_name: &str,
        original_branch: &str,
    ) -> Result<()> {
        // Check if branch has remote tracking
        if !has_remote_tracking(git_runner, branch_name)? {
            println!("  - Skipping {}: no remote tracking branch", branch_name);
            return Ok(());
        }

        println!("  - Pulling changes for {}", branch_name);

        // Checkout the branch
        checkout_branch(git_runner, branch_name)?;

        // Pull changes
        match pull_current_branch(git_runner) {
            Ok(()) => {
                println!("    ✓ Successfully pulled changes");
            }
            Err(e) => {
                // Return to original branch before propagating error
                let _ = checkout_branch(git_runner, original_branch);
                return Err(e);
            }
        }

        Ok(())
    }

    /// Pull changes for multiple stack branches
    pub fn pull_stack_branches(
        git_runner: &dyn GitRunner,
        branch_names: &[String],
        original_branch: &str,
    ) -> Result<()> {
        for branch_name in branch_names {
            pull_branch(git_runner, branch_name, original_branch)?;
        }
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

    /// Find the first existing branch in a stack
    pub fn find_first_branch_in_stack(
        git_runner: &dyn GitRunner,
        feature_name: &str,
    ) -> Result<Option<String>> {
        let branches = crate::git::list_branches(git_runner)?;
        let mut stack_branches: Vec<(u32, String)> = Vec::new();

        // Collect all branches for this stack
        for branch in branches {
            if let Some(stack_info) = parse_stack_branch(&branch) {
                if stack_info.feature_name == feature_name {
                    stack_branches.push((stack_info.index, branch));
                }
            }
        }

        if stack_branches.is_empty() {
            return Ok(None);
        }

        // Sort by index and return the first (lowest index)
        stack_branches.sort_by_key(|&(index, _)| index);
        Ok(Some(stack_branches[0].1.clone()))
    }

    /// Get all branches for a specific stack feature
    pub fn get_stack_branches(
        git_runner: &dyn GitRunner,
        feature_name: &str,
    ) -> Result<Vec<String>> {
        let branches = crate::git::list_branches(git_runner)?;
        let mut stack_branches: Vec<(u32, String)> = Vec::new();

        for branch in branches {
            if let Some(stack_info) = parse_stack_branch(&branch) {
                if stack_info.feature_name == feature_name {
                    stack_branches.push((stack_info.index, branch));
                }
            }
        }

        // Sort by index to maintain proper ordering
        stack_branches.sort_by_key(|&(index, _)| index);
        Ok(stack_branches
            .into_iter()
            .map(|(_, branch)| branch)
            .collect())
    }
}

/// High-level operations for git-stack commands
pub mod commands {
    use super::*;
    use crate::github::GitHubRunner;

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
    fn continue_current_stack(
        git_runner: &dyn GitRunner,
        stack_info: &branch::StackInfo,
    ) -> Result<String> {
        let next_index = branch::get_next_index(git_runner, &stack_info.feature_name)?;
        let new_branch_name = format!("{}/{}", stack_info.feature_name, next_index);
        git::create_branch(git_runner, &new_branch_name)?;
        Ok(new_branch_name)
    }

    /// Get the current branch name
    pub fn current_branch(git_runner: &dyn GitRunner) -> Result<String> {
        git::get_current_branch(git_runner)
    }

    /// List all git stacks in the repository
    pub fn list_stacks(git_runner: &dyn GitRunner) -> Result<()> {
        list_stacks_with_github(git_runner, None)
    }

    /// List all git stacks in the repository with optional GitHub integration
    pub fn list_stacks_with_github(git_runner: &dyn GitRunner, github_runner: Option<&dyn crate::github::GitHubRunner>) -> Result<()> {
        git::check_repository(git_runner)?;

        // Get all branches
        let branches = git::list_branches(git_runner)?;

        // Parse stack branches and group them
        let stacks = analyze_stacks(&branches);

        // Try to get PR information if GitHub runner is provided
        let pr_info = if let Some(github) = github_runner {
            match get_pr_info_for_stacks(github, &stacks) {
                Ok(info) => Some(info),
                Err(e) => {
                    // Log warning but continue with normal display
                    println!("Warning: Could not fetch PR information: {}", e);
                    None
                }
            }
        } else {
            None
        };

        // Display the stacks in tree format with PR info if available
        display_stacks_with_pr_info(&stacks, pr_info.as_ref());

        Ok(())
    }

    /// Get PR information for all branches in the stacks
    fn get_pr_info_for_stacks(
        github_runner: &dyn crate::github::GitHubRunner,
        stacks: &std::collections::BTreeMap<String, Vec<u32>>
    ) -> Result<std::collections::HashMap<String, crate::github::PullRequestInfo>> {
        // Collect all branch names
        let mut all_branches = Vec::new();
        for (feature_name, indices) in stacks {
            for index in indices {
                all_branches.push(format!("{}/{}", feature_name, index));
            }
        }

        // Fetch PR information for all branches
        github_runner.batch_get_pull_request_info(&all_branches)
    }

    /// Analyze branches to extract stack information
    pub fn analyze_stacks(branches: &[String]) -> std::collections::BTreeMap<String, Vec<u32>> {
        let mut stacks: std::collections::BTreeMap<String, Vec<u32>> =
            std::collections::BTreeMap::new();

        for branch in branches {
            if let Some(stack_info) = branch::parse_stack_branch(branch) {
                stacks
                    .entry(stack_info.feature_name)
                    .or_default()
                    .push(stack_info.index);
            }
        }

        // Sort indices within each stack
        for indices in stacks.values_mut() {
            indices.sort_unstable();
        }

        stacks
    }

    /// Synchronize git stacks with the remote repository
    pub fn sync_stacks(git_runner: &dyn GitRunner) -> Result<()> {
        git::check_repository(git_runner)?;

        // Get the current branch to determine sync context
        let current_branch = git::get_current_branch(git_runner)?;
        let current_context = branch::parse_stack_branch(&current_branch);
        let is_on_base_branch = branch::is_base_branch(git_runner, &current_branch)?;

        // Determine sync scope based on current branch context
        match (is_on_base_branch, current_context) {
            // On default branch - sync all stacks
            (true, _) => {
                println!("Syncing all stacks from default branch...");
                sync_all_stacks(git_runner)
            }
            // On stack branch - sync current stack only
            (false, Some(stack_info)) => {
                println!("Syncing current stack: {}", stack_info.feature_name);
                sync_current_stack(git_runner, &stack_info)
            }
            // On non-stack/non-default branch - error
            (false, None) => Err(GitStackError::InvalidBranchForSync(current_branch)),
        }
    }

    /// Sync all stacks in the repository
    fn sync_all_stacks(git_runner: &dyn GitRunner) -> Result<()> {
        let original_branch = git::get_current_branch(git_runner)?;

        println!("🔄 Starting sync for all stacks...");

        // Step 1: Fetch from remote
        println!("\n1. Fetching from remote...");
        git::fetch_remote(git_runner)?;

        // Step 2: Get all stacks
        let branches = git::list_branches(git_runner)?;
        let stacks = analyze_stacks(&branches);

        if stacks.is_empty() {
            println!("ℹ️  No stacks found in repository");
            return Ok(());
        }

        println!("\n2. Syncing {} stack(s):", stacks.len());

        // Step 3: Sync each stack
        for feature_name in stacks.keys() {
            println!("\n📦 Syncing stack: {}", feature_name);
            sync_stack_by_name(git_runner, feature_name, &original_branch)?;
        }

        // Step 4: Return to original branch
        println!("\n3. Returning to original branch: {}", original_branch);
        git::checkout_branch(git_runner, &original_branch)?;

        println!("✅ All stacks synchronized successfully!");
        Ok(())
    }

    /// Sync the current stack only
    fn sync_current_stack(
        git_runner: &dyn GitRunner,
        stack_info: &branch::StackInfo,
    ) -> Result<()> {
        let original_branch = git::get_current_branch(git_runner)?;

        println!("🔄 Starting sync for stack: {}", stack_info.feature_name);

        // Step 1: Fetch from remote
        println!("\n1. Fetching from remote...");
        git::fetch_remote(git_runner)?;

        // Step 2: Sync this stack only
        println!("\n2. Syncing current stack:");
        sync_stack_by_name(git_runner, &stack_info.feature_name, &original_branch)?;

        // Step 3: Return to original branch
        println!("\n3. Returning to original branch: {}", original_branch);
        git::checkout_branch(git_runner, &original_branch)?;

        println!(
            "✅ Stack '{}' synchronized successfully!",
            stack_info.feature_name
        );
        Ok(())
    }

    /// Create pull requests for a git stack using GitHub CLI
    pub fn review_stack(
        git_runner: &dyn GitRunner,
        github_runner: &dyn GitHubRunner,
    ) -> Result<()> {
        git::check_repository(git_runner)?;

        // Check GitHub CLI availability
        github_runner.check_availability()?;

        // Get current branch and validate context
        let current_branch = git::get_current_branch(git_runner)?;
        let current_context = branch::parse_stack_branch(&current_branch);
        let is_on_base_branch = branch::is_base_branch(git_runner, &current_branch)?;

        // Validate we're on a stack branch
        match (is_on_base_branch, current_context) {
            (true, _) => {
                return Err(GitStackError::InvalidContextForReview(format!(
                    "Cannot run review from default branch '{}'. Please switch to a stack branch.",
                    current_branch
                )));
            }
            (false, None) => {
                return Err(GitStackError::InvalidContextForReview(
                    format!("Cannot run review from non-stack branch '{}'. Please switch to a stack branch.", current_branch)
                ));
            }
            (false, Some(stack_info)) => {
                println!(
                    "🔄 Creating pull requests for stack: {}",
                    stack_info.feature_name
                );
                create_stack_prs(git_runner, github_runner, &stack_info.feature_name)?;
            }
        }

        Ok(())
    }

    /// Create pull requests for all branches in a stack
    fn create_stack_prs(
        git_runner: &dyn GitRunner,
        github_runner: &dyn GitHubRunner,
        feature_name: &str,
    ) -> Result<()> {
        // Get all branches for this stack
        let stack_branches = branch::get_stack_branches(git_runner, feature_name)?;

        if stack_branches.is_empty() {
            println!("ℹ️  No branches found for stack '{}'", feature_name);
            return Ok(());
        }

        println!("📦 Found {} branch(es) in stack:", stack_branches.len());
        for branch in &stack_branches {
            println!("  - {}", branch);
        }

        // Get default branch for PR base
        let default_branch = git::get_default_branch(git_runner)?;

        // Analyze existing PRs for the stack
        let mut pr_numbers: std::collections::HashMap<String, u32> =
            std::collections::HashMap::new();

        println!("\n🔍 Checking for existing pull requests...");
        for branch in &stack_branches {
            if let Some(pr_num) = github_runner.list_pull_requests_for_branch(branch)? {
                println!("  ✓ {} already has PR #{}", branch, pr_num);
                pr_numbers.insert(branch.clone(), pr_num);
            } else {
                println!("  - {} needs PR creation", branch);
            }
        }

        // Check if all PRs already exist
        if pr_numbers.len() == stack_branches.len() {
            println!("\n✅ All PRs already exist for this stack:");
            for branch in &stack_branches {
                if let Some(pr_num) = pr_numbers.get(branch) {
                    println!("  {} -> PR #{}", branch, pr_num);
                }
            }
            return Ok(());
        }

        // Create PRs for branches that don't have them
        println!("\n🚀 Creating missing pull requests...");
        let mut dependency_pr: Option<u32> = None;
        let mut previous_branch: Option<String> = None;

        for branch in &stack_branches {
            // If this branch already has a PR, skip creation but update tracking
            if let Some(existing_pr) = pr_numbers.get(branch) {
                println!("  ✓ {} already has PR #{} (skipping)", branch, existing_pr);
                dependency_pr = Some(*existing_pr);
                previous_branch = Some(branch.clone());
                continue;
            }

            // Push the branch to remote first
            println!("  • Pushing branch {} to remote...", branch);
            match git::push_branch(git_runner, branch) {
                Ok(()) => {
                    println!("    ✓ Branch pushed to remote");
                }
                Err(e) => {
                    println!("    ❌ Failed to push branch: {}", e);
                    return Err(e);
                }
            }

            // Parse branch to get index for title
            if let Some(stack_info) = branch::parse_stack_branch(branch) {
                let title = format!("{} #{}", stack_info.feature_name, stack_info.index);

                // Build PR body with dependency if needed
                let body = if let Some(dep_pr) = dependency_pr {
                    format!("Depends on #{}", dep_pr)
                } else {
                    "".to_string()
                };

                // Determine the correct base branch
                let base_branch = if let Some(ref prev_branch) = previous_branch {
                    prev_branch
                } else {
                    &default_branch
                };

                println!("  • Creating PR for {}: '{}'", branch, title);

                match github_runner.create_pull_request(branch, &title, &body, base_branch) {
                    Ok(pr_num) => {
                        println!("    ✓ Created PR #{}", pr_num);
                        dependency_pr = Some(pr_num);
                        previous_branch = Some(branch.clone());
                        pr_numbers.insert(branch.clone(), pr_num);
                    }
                    Err(e) => {
                        println!("    ❌ Failed to create PR: {}", e);
                        return Err(e);
                    }
                }
            }
        }

        // Print summary
        println!("\n✅ Review summary:");
        for branch in &stack_branches {
            if let Some(pr_num) = pr_numbers.get(branch) {
                println!("  {} -> PR #{}", branch, pr_num);
            }
        }

        Ok(())
    }

    /// Sync a specific stack by feature name
    fn sync_stack_by_name(
        git_runner: &dyn GitRunner,
        feature_name: &str,
        original_branch: &str,
    ) -> Result<()> {
        // Get all branches for this stack
        let stack_branches = branch::get_stack_branches(git_runner, feature_name)?;

        if stack_branches.is_empty() {
            println!("  ⚠️  No branches found for stack '{}'", feature_name);
            return Ok(());
        }

        // Step 1: Pull all branches with remote tracking
        println!("  • Pulling remote changes:");
        git::pull_stack_branches(git_runner, &stack_branches, original_branch)?;

        // Step 2: Find first branch for rebasing
        if let Some(first_branch) = branch::find_first_branch_in_stack(git_runner, feature_name)? {
            println!("  • Rebasing stack from: {}", first_branch);

            // Checkout first branch and rebase
            git::checkout_branch(git_runner, &first_branch)?;

            match git::rebase_with_update_refs(git_runner, &first_branch) {
                Ok(()) => println!("    ✓ Stack rebased successfully"),
                Err(e) => {
                    println!("    ❌ Rebase failed: {}", e);
                    println!(
                        "    Please resolve conflicts manually and run 'git rebase --continue'"
                    );
                    // Return to original branch before propagating error
                    let _ = git::checkout_branch(git_runner, original_branch);
                    return Err(e);
                }
            }
        } else {
            println!("  ⚠️  No first branch found for stack '{}'", feature_name);
        }

        Ok(())
    }

    /// Display stacks in tree format with optional PR information
    fn display_stacks_with_pr_info(
        stacks: &std::collections::BTreeMap<String, Vec<u32>>,
        pr_info: Option<&std::collections::HashMap<String, crate::github::PullRequestInfo>>
    ) {
        if stacks.is_empty() {
            println!("No stacks found in this repository.");
            return;
        }

        let stack_names: Vec<_> = stacks.keys().collect();

        for (stack_idx, (feature_name, indices)) in stacks.iter().enumerate() {
            let is_last_stack = stack_idx == stack_names.len() - 1;

            // Print the feature name as the stack root
            println!("{}", feature_name);

            // Print each branch in the stack
            for (branch_idx, index) in indices.iter().enumerate() {
                let is_last_branch = branch_idx == indices.len() - 1;
                let prefix = if is_last_branch { "└─" } else { "├─" };

                let branch_name = format!("{}/{}", feature_name, index);
                
                // Check if we have PR information for this branch
                let display_line = if let Some(pr_map) = pr_info {
                    if let Some(pr) = pr_map.get(&branch_name) {
                        format!("{} {} #{} ({})", 
                            prefix, 
                            branch_name, 
                            pr.number, 
                            pr.status.display()
                        )
                    } else {
                        format!("{} {}", prefix, branch_name)
                    }
                } else {
                    format!("{} {}", prefix, branch_name)
                };

                // Apply color coding if PR information is available
                if let Some(pr_map) = pr_info {
                    if let Some(pr) = pr_map.get(&branch_name) {
                        print!("{}", pr.status.color_code());
                        println!("{}", display_line);
                        print!("{}", crate::github::PrStatus::reset_color());
                    } else {
                        println!("{}", display_line);
                    }
                } else {
                    println!("{}", display_line);
                }
            }

            // Add spacing between stacks (except after the last one)
            if !is_last_stack {
                println!();
            }
        }
    }
}
