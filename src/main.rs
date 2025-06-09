use anyhow::Result;
use clap::Parser;

mod cli;
mod git;
mod utils;
mod syntax;

use cli::Cli;
use git::{get_last_commit, get_blame};
use utils::resolve_path_all;

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

    // Resolve the path(s) - either search for it or use current directory
    let target_paths = if let Some(input_path) = cli.path {
        let matches = resolve_path_all(&input_path)?;
        
        // In blame mode, only allow single file
        if cli.blame && matches.len() > 1 {
            let mut error_msg = format!("Multiple files/directories named '{}' found:\n", input_path);
            for (i, path) in matches.iter().enumerate() {
                error_msg.push_str(&format!("  {}. {}\n", i + 1, path));
            }
            error_msg.push_str("\nBlame mode only works with a single file. Please specify the full path to the desired file.");
            return Err(anyhow::anyhow!(error_msg));
        }
        
        matches
    } else {
        vec![".".to_string()]
    };

    // Process each target path
    let mut results = Vec::new();
    
    for (_i, target_path) in target_paths.iter().enumerate() {
        let output = if cli.blame {
            get_blame(target_path, cli.no_color, cli.date_only, cli.commit_message)?
        } else {
            get_last_commit(target_path, cli.no_color, cli.date_only, cli.commit_message, cli.last)?
        };
        
        // If multiple files, show which file this is
        if target_paths.len() > 1 {
            results.push(format!("{}:\n{}", target_path, output));
        } else {
            results.push(output);
        }
    }

    // Print all results
    for (i, result) in results.iter().enumerate() {
        if i > 0 {
            println!(); // Add blank line between multiple results
        }
        println!("{}", result);
    }

    Ok(())
}
