use anyhow::Result;
use clap::Parser;

mod cli;
mod git;
mod utils;
mod syntax;

use cli::Cli;
use git::{get_last_commit, get_blame};
use utils::resolve_path;

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
    // Resolve the path - either search for it or use current directory
    let target_path = if let Some(input_path) = cli.path {
        resolve_path(&input_path)?
    } else {
        ".".to_string()
    };

    // Validate that date_only and commit_message are not used together
    if cli.date_only && cli.commit_message {
        return Err(anyhow::anyhow!(
            "Cannot use both --date-only and --commit-message flags together. Choose one."
        ));
    }

    // Validate that --last only works in normal mode (not blame mode)
    if cli.last.is_some() && cli.blame {
        return Err(anyhow::anyhow!(
            "--last flag only works in normal mode, not with --blame"
        ));
    }

    // Choose between blame and last commit based on the flag
    let output = if cli.blame {
        get_blame(&target_path, cli.no_color, cli.date_only, cli.commit_message)?
    } else {
        get_last_commit(&target_path, cli.no_color, cli.date_only, cli.commit_message, cli.last)?
    };

    println!("{}", output);

    Ok(())
}
