use clap::{Parser, Subcommand};
use git_stack::{commands, github::RealGitHubRunner, RealGitRunner};
use std::process;

/// Command line interface structure for git-stack
#[derive(Parser)]
#[command(name = "git-stack")]
#[command(about = "A tool for managing stacked git branches")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available subcommands for git-stack
#[derive(Subcommand)]
enum Commands {
    /// Create a new branch in the current stack or continue an existing stack
    New {
        /// The name of the feature for the new branch (optional - if not provided, continues current stack)
        feature_name: Option<String>,
    },
    /// List all stacks in the repository
    List,
    /// Synchronize git stacks with the remote repository
    Sync,
    /// Create pull requests for a git stack using GitHub CLI
    Review,
}

fn main() {
    let cli = Cli::parse();
    let git_runner = RealGitRunner;
    let github_runner = RealGitHubRunner;

    let result = match &cli.command {
        Commands::New { feature_name } => {
            match commands::current_branch(&git_runner) {
                Ok(current) => println!("Current branch: {}", current),
                Err(e) => {
                    eprintln!("Warning: Could not determine current branch: {}", e);
                }
            }

            let feature_name_ref = feature_name.as_deref();
            match commands::new_branch_contextual(&git_runner, feature_name_ref) {
                Ok(branch_name) => {
                    println!("Created new branch: {}", branch_name);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Commands::List => match commands::list_stacks_with_github(&git_runner, Some(&github_runner)) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        },
        Commands::Sync => match commands::sync_stacks(&git_runner) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        },
        Commands::Review => match commands::review_stack(&git_runner, &github_runner) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        },
    };

    if let Err(error) = result {
        eprintln!("Error: {}", error);
        process::exit(1);
    }
}
