use anyhow::Result;
use clap::Parser;

mod cli;
mod git;
mod utils;
mod syntax;

use cli::Cli;
use git::{get_last_commit, get_blame};

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

    // Choose between blame and last commit based on the flag
    let output = if cli.blame {
        get_blame(&target_path, cli.no_color)?
    } else {
        get_last_commit(&target_path, cli.no_color)?
    };

    println!("{}", output);

    Ok(())
}
