use anyhow::{Result, anyhow};
use git2::Repository;
use std::path::{Path, PathBuf};

use crate::utils::{format_timestamp_day_month, format_timestamp_day_month_year};
use crate::syntax::SyntaxHighlighter;

/// Validates path existence, file type, finds git repository, and returns all necessary paths
fn validate_git_path(path: &str, must_be_file: bool) -> Result<(Repository, PathBuf, PathBuf)> {
    // First, resolve the full path
    let full_path = if Path::new(path).is_absolute() {
        PathBuf::from(path)
    } else {
        std::env::current_dir()?.join(path)
    };

    // Check if the path exists
    if !full_path.exists() {
        return Err(anyhow!(
            "Path '{}' doesn't exist. Check if the file or directory name is spelled correctly.",
            path
        ));
    }

    // Check if it's a file (when required, e.g., for blame)
    if must_be_file && full_path.is_dir() {
        return Err(anyhow!(
            "Blame can only be used on files, not directories: {}",
            path
        ));
    }

    // Find the git repository
    let search_path = if full_path.is_file() {
        full_path.parent().unwrap_or(&full_path).to_path_buf()
    } else {
        full_path.clone()
    };

    let repo = Repository::discover(search_path)
        .map_err(|_| anyhow!("Not a git repository (or any of the parent directories)"))?;

    // Convert path to relative path from repo root
    let repo_workdir = repo
        .workdir()
        .ok_or_else(|| anyhow!("Repository has no working directory"))?;

    let relative_path = full_path
        .strip_prefix(repo_workdir)
        .map_err(|_| anyhow!("Path '{}' is not within the repository", path))?
        .to_path_buf();

    Ok((repo, full_path, relative_path))
}

pub fn get_blame(path: &str, no_color: bool, date_only: bool) -> Result<String> {
    // Validate path and get repository, full path, and relative path
    let (repo, full_path, relative_path) = validate_git_path(path, true)?;

    // Get the blame for the file
    let blame = repo
        .blame_file(&relative_path, None)
        .map_err(|e| {
            match e.code() {
                git2::ErrorCode::NotFound => anyhow!(
                    "File '{}' exists but is not tracked by git. Use 'git add {}' to track it first.",
                    path,
                    path
                ),
                _ => anyhow!("Failed to get blame for file '{}': {}", path, e)
            }
        })?;

    let mut result = String::new();

    // Read the file content to display alongside blame
    let file_content =
        std::fs::read_to_string(&full_path).map_err(|e| anyhow!("Failed to read file: {}", e))?;

    let lines: Vec<&str> = file_content.lines().collect();
    let line_count = lines.len();
    let line_width = line_count.to_string().len();

    // Initialize syntax highlighter if colors are enabled
    let highlighter = if !no_color {
        Some(SyntaxHighlighter::new())
    } else {
        None
    };

    for (line_num, line_content) in lines.iter().enumerate() {
        let hunk_result = blame.get_line(line_num + 1);

        // Apply syntax highlighting to the line content if enabled
        let highlighted_line = if let Some(ref highlighter) = highlighter {
            highlighter
                .highlight_line(line_content, &full_path, line_num + 1)
                .unwrap_or_else(|_| line_content.to_string())
        } else {
            line_content.to_string()
        };

        if date_only {
            // Date-only format: show just date, line number, and code
            let date = if let Some(hunk) = &hunk_result {
                let commit = repo
                    .find_commit(hunk.final_commit_id())
                    .map_err(|e| anyhow!("Failed to find commit: {}", e))?;

                let time = commit.time();
                format_timestamp_day_month(time.seconds())
            } else {
                "Unknown".to_string()
            };

            let date_color = if no_color { "" } else { "\x1b[36m" }; // Cyan for date
            let reset_color = if no_color { "" } else { "\x1b[0m" }; // Reset color

            result.push_str(&format!(
                "{}{:>6}{} | {:>width$} | {}\n",
                date_color,
                date,
                reset_color,
                line_num + 1,
                highlighted_line,
                width = line_width
            ));
        } else {
            // Full blame format: show commit hash, author, date, line number, and code
            let (commit_hash, author_name, date) = if let Some(hunk) = &hunk_result {
                let commit = repo
                    .find_commit(hunk.final_commit_id())
                    .map_err(|e| anyhow!("Failed to find commit: {}", e))?;

                let author = commit.author();
                let name = author.name().unwrap_or("Unknown");
                let time = commit.time();
                let formatted_date = format_timestamp_day_month(time.seconds());

                (
                    hunk.final_commit_id()
                        .to_string()
                        .chars()
                        .take(7)
                        .collect::<String>(),
                    name.chars().take(15).collect::<String>(),
                    formatted_date,
                )
            } else {
                ("~~~~~~~".to_string(), "Unknown".to_string(), "Unknown".to_string())
            };

            // ANSI color codes
            let commit_color = if no_color { "" } else { "\x1b[33m" }; // Yellow for commit hash
            let date_color = if no_color { "" } else { "\x1b[36m" }; // Cyan for date
            let reset_color = if no_color { "" } else { "\x1b[0m" }; // Reset color

            result.push_str(&format!(
                "{}{:>7}{} ({:>15} - {}{:>6}{}) | {:>width$} | {}\n",
                commit_color,
                commit_hash,
                reset_color,
                author_name,
                date_color,
                date,
                reset_color,
                line_num + 1,
                highlighted_line,
                width = line_width
            ));
        }
    }

    Ok(result)
}

pub fn get_last_commit(path: &str, no_color: bool, date_only: bool) -> Result<String> {
    // Validate path and get repository and relative path (no file requirement for last commit)
    let (repo, _, relative_path) = validate_git_path(path, false)?;

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    // Walk through commits to find the last one that modified the path
    for commit_id in revwalk {
        let commit_id = commit_id?;
        let commit = repo.find_commit(commit_id)?;

        if commit_touches_path(&repo, &commit, &relative_path)? {
            let time = commit.time();
            let date = format_timestamp_day_month_year(time.seconds());

            // If date_only is requested, return just the date
            if date_only {
                return Ok(if no_color {
                    date
                } else {
                    format!("\x1b[36m{}\x1b[0m", date) // Cyan date
                });
            }

            // Otherwise return the full commit info
            let author = commit.author();
            let name = author.name().unwrap_or("Unknown");
            let commit_hash = commit_id.to_string().chars().take(7).collect::<String>();
            let message = commit.summary().unwrap_or("No message");

            // ANSI color codes
            let commit_color = if no_color { "" } else { "\x1b[33m" }; // Yellow for commit hash
            let date_color = if no_color { "" } else { "\x1b[36m" };   // Cyan for date
            let reset_color = if no_color { "" } else { "\x1b[0m" };   // Reset color

            return Ok(format!(
                "{}{}{} {} - {}{}{}: {}",
                commit_color,
                commit_hash,
                reset_color,
                name,
                date_color,
                date,
                reset_color,
                message
            ));
        }
    }

    Err(anyhow!("No commits found for path: {}", path))
}

fn commit_touches_path(repo: &Repository, commit: &git2::Commit, path: &Path) -> Result<bool> {
    // For the first commit (no parents), check if the path exists in the tree
    if commit.parent_count() == 0 {
        let tree = commit.tree()?;
        return Ok(tree_contains_path(&tree, path));
    }

    // For commits with parents, check if the path was modified
    let tree = commit.tree()?;
    let parent = commit.parent(0)?;
    let parent_tree = parent.tree()?;

    let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;

    let mut path_modified = false;
    diff.foreach(
        &mut |delta, _progress| {
            if let Some(file_path) = delta.new_file().path() {
                if file_path.starts_with(path) || path.starts_with(file_path) {
                    path_modified = true;
                }
            }
            if let Some(file_path) = delta.old_file().path() {
                if file_path.starts_with(path) || path.starts_with(file_path) {
                    path_modified = true;
                }
            }
            true
        },
        None,
        None,
        None,
    )?;

    Ok(path_modified)
}

fn tree_contains_path(tree: &git2::Tree, path: &Path) -> bool {
    if path == Path::new("") || path == Path::new(".") {
        return true;
    }

    tree.get_path(path).is_ok()
}
