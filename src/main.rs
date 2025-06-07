use anyhow::Result;
use clap::Parser;

mod cli;
mod git;
mod utils;

use cli::Cli;
use git::{find_repository, get_last_commit};

fn main() {
    let cli = Cli::parse();

    match run(cli) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run(cli: Cli) -> Result<()> {
    // Get the path, default to current directory if none provided
    let target_path = cli.path.unwrap_or_else(|| ".".to_string());

    // Find the git repository
    let repo = find_repository(&target_path)?;

    // Get the last commit for the path
    let commit_info = get_last_commit(&repo, &target_path)?;

    println!("{}", commit_info);

    Ok(())
}
