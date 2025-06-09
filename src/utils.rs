use chrono::{DateTime, Utc};
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, anyhow};

pub fn format_timestamp_day_month(timestamp: i64) -> String {
    // Convert Unix timestamp to short format like "22 May" or "07 Jun" (no year)
    let dt = DateTime::from_timestamp(timestamp, 0).unwrap_or_else(|| Utc::now());

    dt.format("%d %b").to_string()
}

pub fn format_timestamp_day_month_year(timestamp: i64) -> String {
    // Convert Unix timestamp to format like "22 May 2025" or "07 Jun 2025" (with year)
    let dt = DateTime::from_timestamp(timestamp, 0).unwrap_or_else(|| Utc::now());

    dt.format("%d %b %Y").to_string()
}

/// Search for a file or directory by name starting from the current directory
/// Returns all matches found, or the original path if it's absolute or starts with ~/
pub fn resolve_path_all(input: &str) -> Result<Vec<String>> {
    // If it's already an absolute path or starts with ~/, return as-is
    if input.starts_with('/') || input.starts_with("~/") {
        return Ok(vec![input.to_string()]);
    }
    
    // If it exists as a relative path from current directory, use it
    if Path::new(input).exists() {
        return Ok(vec![input.to_string()]);
    }
    
    // Otherwise, search for it recursively starting from current directory
    let current_dir = std::env::current_dir()?;
    
    let matches = search_all_matches(&current_dir, input)?;
    
    if matches.is_empty() {
        return Err(anyhow!("No file or directory named '{}' found starting from current directory", input));
    }
    
    // Convert all matches to relative paths
    let relative_matches: Vec<String> = matches
        .iter()
        .map(|path| {
            path.strip_prefix(&current_dir)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string()
        })
        .collect();
    
    Ok(relative_matches)
}

/// Recursively search for all files or directories matching the target name
fn search_all_matches(dir: &Path, target_name: &str) -> Result<Vec<PathBuf>> {
    let mut matches = Vec::new();
    search_recursive_all(dir, target_name, &mut matches)?;
    Ok(matches)
}

/// Recursively search for all files or directories by name and collect them
fn search_recursive_all(dir: &Path, target_name: &str, matches: &mut Vec<PathBuf>) -> Result<()> {
    let entries = fs::read_dir(dir)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();
        
        // Check if this entry matches our target
        if file_name == target_name {
            matches.push(path.clone());
        }
        
        // If it's a directory, search recursively (but skip hidden directories and common ignore patterns)
        if path.is_dir() && !is_ignored_directory(&file_name) {
            search_recursive_all(&path, target_name, matches)?;
        }
    }
    
    Ok(())
}

/// Check if a directory should be ignored during search
fn is_ignored_directory(name: &str) -> bool {
    match name {
        ".git" | ".svn" | ".hg" | 
        "node_modules" | "target" | "build" | "dist" |
        ".vscode" | ".idea" | "__pycache__" => true,
        name if name.starts_with('.') => true,
        _ => false,
    }
} 
