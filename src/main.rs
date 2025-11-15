use clap::{Parser, Subcommand};
use git_stack::{commands, RealGitRunner};
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
    /// Create a new branch in the current stack
    New {
        /// The name of the feature for the new branch
        feature_name: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let git_runner = RealGitRunner;

    let result = match &cli.command {
        Commands::New { feature_name } => {
            match commands::current_branch(&git_runner) {
                Ok(current) => println!("Current branch: {}", current),
                Err(e) => {
                    eprintln!("Warning: Could not determine current branch: {}", e);
                }
            }

            match commands::new_branch(&git_runner, feature_name) {
                Ok(branch_name) => {
                    println!("Created new branch: {}", branch_name);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
    };

    if let Err(error) = result {
        eprintln!("Error: {}", error);
        process::exit(1);
    }
}
