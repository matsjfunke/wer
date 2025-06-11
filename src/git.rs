use anyhow::{Result, anyhow};
use git2::Repository;
use std::path::{Path, PathBuf};

use crate::syntax::SyntaxHighlighter;
use crate::utils::{format_timestamp_day_month, format_timestamp_day_month_year};

/// Color scheme for output formatting
struct ColorScheme {
    commit: &'static str,
    date: &'static str,
    reset: &'static str,
}

impl ColorScheme {
    fn new(no_color: bool) -> Self {
        if no_color {
            Self {
                commit: "",
                date: "",
                reset: "",
            }
        } else {
            Self {
                commit: "\x1b[33m", // Yellow
                date: "\x1b[36m",   // Cyan
                reset: "\x1b[0m",   // Reset
            }
        }
    }
}

/// Extracted commit information
struct CommitInfo {
    hash: String,
    author: String,
    date: String,
    message: String,
}

impl CommitInfo {
    fn from_commit(commit: &git2::Commit, use_year: bool) -> Self {
        let author = commit.author();
        let name = author.name().unwrap_or("Unknown");
        let time = commit.time();

        Self {
            hash: commit.id().to_string().chars().take(7).collect(),
            author: name.chars().take(15).collect(),
            date: if use_year {
                format_timestamp_day_month_year(time.seconds())
            } else {
                format_timestamp_day_month(time.seconds())
            },
            message: commit.summary().unwrap_or("No message").to_string(),
        }
    }

    fn from_hunk(repo: &Repository, hunk: &git2::BlameHunk, use_year: bool) -> Result<Self> {
        let commit = repo.find_commit(hunk.final_commit_id())?;
        Ok(Self::from_commit(&commit, use_year))
    }

    fn unknown(_use_year: bool) -> Self {
        Self {
            hash: "~~~~~~~".to_string(),
            author: "Unknown".to_string(),
            date: "Unknown".to_string(),
            message: "Unknown".to_string(),
        }
    }

    /// Format for regular mode output
    fn format_regular(&self, colors: &ColorScheme, commit_message_separate: bool) -> String {
        if commit_message_separate {
            format!(
                "{}{}{} {} - {}{}{}\n└─ {}",
                colors.commit,
                self.hash,
                colors.reset,
                self.author,
                colors.date,
                self.date,
                colors.reset,
                self.message
            )
        } else {
            format!(
                "{}{}{} {} - {}{}{}: {}",
                colors.commit,
                self.hash,
                colors.reset,
                self.author,
                colors.date,
                self.date,
                colors.reset,
                self.message
            )
        }
    }

    /// Format for blame mode output
    fn format_blame(
        &self,
        colors: &ColorScheme,
        line_num: usize,
        highlighted_line: &str,
        commit_message_separate: bool,
    ) -> String {
        if commit_message_separate {
            format!(
                "│ {}{:<7}{} │ {:<15} │ {}{:<6}{} │ {:>4} │ {}\n│ {:<7} │ {:<15} │ {:<6} │ {:<4} │ └─ {}\n",
                colors.commit,
                self.hash,
                colors.reset,
                self.author,
                colors.date,
                self.date,
                colors.reset,
                line_num,
                highlighted_line,
                "",
                "",
                "",
                "",
                self.message,
            )
        } else {
            format!(
                "│ {}{:<7}{} │ {:<15} │ {}{:<6}{} │ {:>4} │ {}\n",
                colors.commit,
                self.hash,
                colors.reset,
                self.author,
                colors.date,
                self.date,
                colors.reset,
                line_num,
                highlighted_line,
            )
        }
    }

    /// Format date-only for blame mode
    fn format_date_only(
        &self,
        colors: &ColorScheme,
        line_num: usize,
        highlighted_line: &str,
    ) -> String {
        format!(
            "│ {}{:<6}{} │ {:>4} │ {}\n",
            colors.date, self.date, colors.reset, line_num, highlighted_line,
        )
    }
}

/// Validates path existence, file type, finds git repository, and returns all necessary paths
fn validate_git_path(path: &str, must_be_file: bool) -> Result<(Repository, PathBuf, PathBuf)> {
    // First, resolve the full path
    let full_path = if Path::new(path).is_absolute() {
        PathBuf::from(path)
    } else {
        // For relative paths, resolve them against current working directory
        std::env::current_dir()?.join(path).canonicalize()
            .map_err(|_| anyhow!("Cannot resolve path '{}'. Check if it exists.", path))?
    };

    // Check if the path exists
    if !full_path.exists() {
        return Err(anyhow!("Path '{}' doesn't exist. Check spelling.", path));
    }

    // Check if it's a file (when required, e.g., for blame)
    if must_be_file && full_path.is_dir() {
        return Err(anyhow!(
            "Blame can only be used on files, not directories: {}",
            path
        ));
    }

    // Find the git repository for this specific path
    let search_path = if full_path.is_file() {
        full_path.parent().unwrap_or(&full_path).to_path_buf()
    } else {
        full_path.clone()
    };

    let repo = Repository::discover(search_path)
        .map_err(|_| anyhow!("Path '{}' is not in a git repository", path))?;

    // Convert path to relative path from the discovered repo root
    let repo_workdir = repo
        .workdir()
        .ok_or_else(|| anyhow!("Repository has no working directory"))?;

    let relative_path = full_path
        .strip_prefix(repo_workdir)
        .map_err(|_| anyhow!("Path '{}' is not within the repository at '{}'", path, repo_workdir.display()))?
        .to_path_buf();

    Ok((repo, full_path, relative_path))
}

pub fn get_blame(
    path: &str,
    no_color: bool,
    date_only: bool,
    commit_message: bool,
) -> Result<String> {
    // Validate path and get repository, full path, and relative path
    let (repo, full_path, relative_path) = validate_git_path(path, true)?;

    // Get the blame for the file
    let blame = repo
        .blame_file(&relative_path, None)
        .map_err(|e| match e.code() {
            git2::ErrorCode::NotFound => anyhow!(
                "File '{}' exists but is not tracked by git. Use 'git add {}' to track it first.",
                path,
                path
            ),
            _ => anyhow!("Failed to get blame for file '{}': {}", path, e),
        })?;

    // Read the file content to display alongside blame
    let file_content =
        std::fs::read_to_string(&full_path).map_err(|e| anyhow!("Failed to read file: {}", e))?;

    let lines: Vec<&str> = file_content.lines().collect();

    // Initialize syntax highlighter if colors are enabled
    let highlighter = if !no_color {
        Some(SyntaxHighlighter::new())
    } else {
        None
    };

    let colors = ColorScheme::new(no_color);
    let mut result = String::new();

    if date_only {
        // Add header for date-only mode (Date, Line, Code only)
        let header_line = format!("┌{:─<8}┬{:─<6}┬{:─<100}┐", "", "", "");
        result.push_str(&header_line);
        result.push('\n');

        result.push_str(&format!("│ {:<6} │ {:<4} │ {}\n", "Date", "Line", "Code"));

        let separator_line = format!("├{:─<8}┼{:─<6}┼{:─<100}┤", "", "", "");
        result.push_str(&separator_line);
        result.push('\n');
    } else {
        // Add header for the full blame table
        let header_line = format!(
            "┌{:─<9}┬{:─<17}┬{:─<8}┬{:─<6}┬{:─<100}┐",
            "", "", "", "", ""
        );
        result.push_str(&header_line);
        result.push('\n');

        result.push_str(&format!(
            "│ {:<7} │ {:<15} │ {:<6} │ {:<4} │ {}\n",
            "Commit", "Name", "Date", "Line", "Code"
        ));

        let separator_line = format!(
            "├{:─<9}┼{:─<17}┼{:─<8}┼{:─<6}┼{:─<100}┤",
            "", "", "", "", ""
        );
        result.push_str(&separator_line);
        result.push('\n');
    }

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

        let line_output = if let Some(hunk) = &hunk_result {
            let commit_info = CommitInfo::from_hunk(&repo, hunk, false)?;

            if date_only {
                commit_info.format_date_only(&colors, line_num + 1, &highlighted_line)
            } else {
                commit_info.format_blame(&colors, line_num + 1, &highlighted_line, commit_message)
            }
        } else {
            let commit_info = CommitInfo::unknown(false);

            if date_only {
                commit_info.format_date_only(&colors, line_num + 1, &highlighted_line)
            } else {
                commit_info.format_blame(&colors, line_num + 1, &highlighted_line, commit_message)
            }
        };

        result.push_str(&line_output);
    }

    // Add bottom border to complete the table
    if date_only {
        let bottom_line = format!("└{:─<8}┴{:─<6}┴{:─<100}┘", "", "", "");
        result.push_str(&bottom_line);
    } else {
        let bottom_line = format!(
            "└{:─<9}┴{:─<17}┴{:─<8}┴{:─<6}┴{:─<100}┘",
            "", "", "", "", ""
        );
        result.push_str(&bottom_line);
    }
    result.push('\n');

    Ok(result)
}

pub fn get_last_commit(
    path: &str,
    no_color: bool,
    date_only: bool,
    commit_message: bool,
    last: Option<usize>,
) -> Result<String> {
    // Validate path and get repository and relative path (no file requirement for last commit)
    let (repo, _, relative_path) = validate_git_path(path, false)?;

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let colors = ColorScheme::new(no_color);

    // If last is requested, collect multiple contributors
    if let Some(n) = last {
        let mut contributors = Vec::new();
        let mut seen_authors = std::collections::HashSet::new();

        for commit_id in revwalk {
            let commit_id = commit_id?;
            let commit = repo.find_commit(commit_id)?;

            if commit_touches_path(&repo, &commit, &relative_path)? {
                let commit_info = CommitInfo::from_commit(&commit, true);

                // Only add if we haven't seen this author before
                if !seen_authors.contains(&commit_info.author) {
                    seen_authors.insert(commit_info.author.clone());
                    contributors.push(commit_info.format_regular(&colors, commit_message));

                    // Stop when we have enough contributors
                    if contributors.len() >= n {
                        break;
                    }
                }
            }
        }

        if contributors.is_empty() {
            return Err(anyhow!("No commits found for path: {}", path));
        }

        let mut result = contributors.join("\n");

        // Add indication if fewer contributors found than requested
        if contributors.len() < n {
            result.push_str(&format!(
                "\nSearched for {} but only {} contributed",
                n,
                contributors.len()
            ));
        }

        return Ok(result);
    }

    // Original single commit logic
    for commit_id in revwalk {
        let commit_id = commit_id?;
        let commit = repo.find_commit(commit_id)?;

        if commit_touches_path(&repo, &commit, &relative_path)? {
            let commit_info = CommitInfo::from_commit(&commit, true);

            // If date_only is requested, return just the date
            if date_only {
                return Ok(if no_color {
                    commit_info.date
                } else {
                    format!("{}{}{}", colors.date, commit_info.date, colors.reset)
                });
            }

            return Ok(commit_info.format_regular(&colors, commit_message));
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
