use anyhow::{anyhow, Result};
use git2::Repository;
use std::path::{Path, PathBuf};

use crate::utils::format_timestamp;

pub fn find_repository(path: &str) -> Result<Repository> {
    let path = Path::new(path);

    // Try to open repository from the given path or find it in parent directories
    Repository::discover(path)
        .map_err(|_| anyhow!("Not a git repository (or any of the parent directories)"))
}

pub fn get_last_commit(repo: &Repository, path: &str) -> Result<String> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    // Convert path to relative path from repo root
    let repo_workdir = repo
        .workdir()
        .ok_or_else(|| anyhow!("Repository has no working directory"))?;

    let full_path = if Path::new(path).is_absolute() {
        PathBuf::from(path)
    } else {
        std::env::current_dir()?.join(path)
    };

    let relative_path = full_path
        .strip_prefix(repo_workdir)
        .map_err(|_| anyhow!("Path is not within the repository"))?;

    // Walk through commits to find the last one that modified the path
    for commit_id in revwalk {
        let commit_id = commit_id?;
        let commit = repo.find_commit(commit_id)?;

        if commit_touches_path(&repo, &commit, relative_path)? {
            let author = commit.author();
            let name = author.name().unwrap_or("Unknown");
            let time = commit.time();

            // Convert timestamp to date string (YYYY-MM-DD format)
            let date = format_timestamp(time.seconds());

            let message = commit.summary().unwrap_or("No message");

            return Ok(format!("{} - {}: {}", name, date, message));
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