//! List subcommand for finding and displaying matching files.
//! 
//! This module handles the `fren list` command which searches for files
//! matching given patterns, optionally recursively, and with exclusion support.
//! All operations are async to match the async API of freneng.

use std::path::PathBuf;
use freneng::{find_matching_files_recursive, FrenError};

/// Finds files matching the given patterns, with optional recursion and exclusions.
/// 
/// # Arguments
/// 
/// * `patterns` - List of glob patterns or file paths to search for
/// * `recursive` - Whether to search recursively in subdirectories
/// * `exclude` - List of patterns to exclude from results
/// 
/// # Returns
/// 
/// * `Ok(Vec<PathBuf>)` - List of matching file paths (deduplicated and filtered)
/// * `Err(FrenError)` - If pattern matching fails
pub async fn find_files(
    patterns: &[String],
    recursive: bool,
    exclude: &[String],
) -> Result<Vec<PathBuf>, FrenError> {
    let mut all_files = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // Process each pattern separately and combine results
    // The engine now handles both glob patterns and literal file paths automatically,
    // but we keep this structure for clarity and potential future CLI-specific handling
    for pat in patterns {
        let files = find_matching_files_recursive(pat, recursive).await?;
        
        // Add files, avoiding duplicates
        for file in files {
            if seen.insert(file.clone()) {
                all_files.push(file);
            }
        }
    }

    // Apply exclusions
    if !exclude.is_empty() {
        all_files.retain(|path| {
            let path_str = path.to_string_lossy();
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            !exclude.iter().any(|excl_pattern| {
                // Try glob pattern matching first
                if let Ok(glob_pattern) = glob::Pattern::new(excl_pattern) {
                    if glob_pattern.matches(file_name) || glob_pattern.matches_path(path) {
                        return true;
                    }
                }
                // Fallback: simple contains check
                path_str.contains(excl_pattern) || file_name.contains(excl_pattern)
            })
        });
    }

    Ok(all_files)
}

/// Displays the list of found files.
/// 
/// # Arguments
/// 
/// * `files` - List of file paths to display
/// * `fullpath` - If true, display full paths; if false, display just filenames
pub fn display_files(files: &[PathBuf], fullpath: bool) {
    if files.is_empty() {
        println!("No matching files found.");
    } else {
        println!("Found {} matching file(s):", files.len());
        for file in files {
            if fullpath {
                println!("  {}", file.display());
            } else {
                let name = file.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("?");
                println!("  {}", name);
            }
        }
    }
}

/// Handles the list subcommand.
/// 
/// # Arguments
/// 
/// * `patterns` - List of search patterns
/// * `recursive` - Whether to search recursively
/// * `exclude` - List of exclusion patterns
/// * `fullpath` - Whether to display full paths or just filenames
/// 
/// # Returns
/// 
/// * `Ok(())` - Command completed successfully
/// * `Err(FrenError)` - If file finding fails
pub async fn handle_list_command(
    patterns: Vec<String>,
    recursive: bool,
    exclude: Vec<String>,
    fullpath: bool,
) -> Result<(), FrenError> {
    if patterns.is_empty() {
        eprintln!("Error: No search pattern provided.");
        std::process::exit(1);
    }

    let files = find_files(&patterns, recursive, &exclude).await?;
    display_files(&files, fullpath);
    
    Ok(())
}

