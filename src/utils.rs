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
/// Returns the first match found, or the original path if it's absolute or starts with ~/
pub fn resolve_path(input: &str) -> Result<String> {
    // If it's already an absolute path or starts with ~/, return as-is
    if input.starts_with('/') || input.starts_with("~/") {
        return Ok(input.to_string());
    }
    
    // If it exists as a relative path from current directory, use it
    if Path::new(input).exists() {
        return Ok(input.to_string());
    }
    
    // Otherwise, search for it recursively starting from current directory
    let current_dir = std::env::current_dir()?;
    
    if let Some(found_path) = search_recursive(&current_dir, input)? {
        // Return relative path from current directory
        let relative = found_path
            .strip_prefix(&current_dir)
            .unwrap_or(&found_path)
            .to_string_lossy()
            .to_string();
        Ok(relative)
    } else {
        Err(anyhow!("No file or directory named '{}' found starting from current directory", input))
    }
}

/// Recursively search for a file or directory by name
fn search_recursive(dir: &Path, target_name: &str) -> Result<Option<PathBuf>> {
    let entries = fs::read_dir(dir)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();
        
        // Check if this entry matches our target
        if file_name == target_name {
            return Ok(Some(path));
        }
        
        // If it's a directory, search recursively (but skip hidden directories and common ignore patterns)
        if path.is_dir() && !is_ignored_directory(&file_name) {
            if let Ok(Some(found)) = search_recursive(&path, target_name) {
                return Ok(Some(found));
            }
        }
    }
    
    Ok(None)
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
