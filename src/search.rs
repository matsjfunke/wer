use anyhow::{Result, anyhow};
use std::fs;
use std::path::{Path, PathBuf};

/// Search for a file or directory by name starting from current directory
/// Handles different path types:
/// - ~/path: home directory paths (returned as-is)
/// - /absolute/path: absolute paths (returned as-is)
/// - ../relative/path or ./path or subdir/path: relative paths (checked directly)
/// - filename: bare filename (searched recursively in current directory)
pub fn find_all_matches(input: &str) -> Result<Vec<String>> {
    find_all_matches_from(input, None)
}

/// Internal function that can optionally specify a base directory (for testing)
fn find_all_matches_from(input: &str, base_dir: Option<&Path>) -> Result<Vec<String>> {
    // If it's already an absolute path or starts with ~/, return as-is
    if input.starts_with('/') || input.starts_with("~/") {
        return Ok(vec![input.to_string()]);
    }

    let current_dir = match base_dir {
        Some(dir) => dir.to_path_buf(),
        None => std::env::current_dir()?,
    };

    // If it contains path separators, treat it as a relative path and check if it exists
    if input.contains('/') {
        let path = current_dir.join(input);
        if path.exists() {
            return Ok(vec![input.to_string()]);
        } else {
            return Err(anyhow!("Path '{}' not found", input));
        }
    }

    // If it exists as a file/directory in current directory, use it
    let direct_path = current_dir.join(input);
    if direct_path.exists() {
        return Ok(vec![input.to_string()]);
    }

    // Otherwise, it's just a filename - search for it recursively
    let mut matches = Vec::new();
    search_recursive(&current_dir, input, &mut matches)?;

    if matches.is_empty() {
        return Err(anyhow!(
            "No file or directory named '{}' found starting from current directory",
            input
        ));
    }

    // Convert all matches to relative paths
    Ok(matches
        .into_iter()
        .map(|path| {
            path.strip_prefix(&current_dir)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string()
        })
        .collect())
}

/// Recursively search for all files or directories by name and collect them
fn search_recursive(dir: &Path, target_name: &str, matches: &mut Vec<PathBuf>) -> Result<()> {
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
            search_recursive(&path, target_name, matches)?;
        }
    }

    Ok(())
}

/// Check if a directory should be ignored during search
fn is_ignored_directory(name: &str) -> bool {
    matches!(
        name,
        ".git"
            | ".svn"
            | ".hg"
            | "node_modules"
            | "target"
            | "build"
            | "dist"
            | ".vscode"
            | ".idea"
            | "__pycache__"
    ) || name.starts_with('.')
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_find_all_matches_absolute_path() {
        // function should recognize an absolute path and return it directly.
        let path = "/some/absolute/path";
        let result = find_all_matches(path).unwrap();
        assert_eq!(result, vec![path.to_string()]);
    }

    #[test]
    fn test_find_all_matches_home_tilde() {
        // function should recognize a home path and return it directly.
        let input = "~/myfile.txt";
        let result = find_all_matches(input).unwrap();
        assert_eq!(result, vec![input.to_string()]);
    }

    #[test]
    fn test_find_all_matches_existing_relative_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("testfile.txt");
        File::create(&file_path).unwrap();

        let rel_path = "testfile.txt";
        let result = find_all_matches_from(rel_path, Some(dir.path())).unwrap();
        assert_eq!(result, vec![rel_path.to_string()]);
    }

    #[test]
    fn test_find_all_matches_recursive_search() {
        let dir = tempdir().unwrap();
        let nested_dir = dir.path().join("nested");
        fs::create_dir(&nested_dir).unwrap();

        let target_file_name = "targetfile.txt";
        let target_path = nested_dir.join(target_file_name);
        let mut file = File::create(&target_path).unwrap();
        file.write_all(b"test content").unwrap();
        file.sync_all().unwrap();
        drop(file); // Explicitly close the file

        let matches = find_all_matches_from(target_file_name, Some(dir.path())).unwrap();
        assert_eq!(
            matches[0], "nested/targetfile.txt",
            "Expected match to be 'nested/targetfile.txt', found: '{}'",
            matches[0]
        );
    }

    #[test]
    fn test_find_all_matches_relative_path() {
        let dir = tempdir().unwrap();
        let nested_dir = dir.path().join("subdir");
        fs::create_dir(&nested_dir).unwrap();
        let file_path = nested_dir.join("testfile.txt");
        File::create(&file_path).unwrap();

        // Test relative path with directory separator
        let result = find_all_matches_from("subdir/testfile.txt", Some(dir.path())).unwrap();
        assert_eq!(result, vec!["subdir/testfile.txt".to_string()]);
    }

    #[test]
    fn test_find_all_matches_not_found() {
        let dir = tempdir().unwrap();

        let result = find_all_matches_from("nonexistentfile.txt", Some(dir.path()));
        assert!(result.is_err());

        // Check that the error matches the expected "file not found" message
        if let Err(e) = result {
            let error_string = e.to_string();
            assert!(
                error_string.contains("No file or directory named 'nonexistentfile.txt'"),
                "Error message was: {}",
                error_string
            );
        }
    }

    #[test]
    fn test_is_ignored_directory() {
        assert!(is_ignored_directory(".git"));
        assert!(is_ignored_directory(".svn"));
        assert!(is_ignored_directory("node_modules"));
        assert!(is_ignored_directory("target"));
        assert!(is_ignored_directory("build"));
        assert!(is_ignored_directory("dist"));
        assert!(is_ignored_directory("__pycache__"));
        assert!(is_ignored_directory(".hidden"));
        assert!(is_ignored_directory(".hg"));
        assert!(is_ignored_directory(".vscode"));
        assert!(is_ignored_directory(".idea"));
        assert!(!is_ignored_directory("some_dir"));
    }
}
