//! Tests for the list subcommand module.
//! 
//! These tests verify file finding, pattern matching, recursion, and exclusion functionality.
//! All tests are async to match the async API of the list module.

// Integration tests need to import from the crate root
// Since frencli is a binary crate, we need to make the modules public and import them
use frencli::list::{find_files, display_files};
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs;

// ============================================================================
// list module tests
// ============================================================================

#[tokio::test]
async fn test_find_files_single_pattern() {
    let temp_dir = TempDir::new().unwrap();
    let files = vec![
        temp_dir.path().join("file1.txt"),
        temp_dir.path().join("file2.txt"),
        temp_dir.path().join("file3.jpg"),
    ];
    
    for file in &files {
        fs::write(file, "test").await.unwrap();
    }
    
    // Change to temp directory for relative patterns
    let temp_path = temp_dir.path().canonicalize().unwrap();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_path).unwrap();
    let _keep_alive = &temp_dir;
    
    let patterns = vec!["*.txt".to_string()];
    let result = find_files(&patterns, false, &[]).await.unwrap();
    
    // Restore directory before assertions
    std::env::set_current_dir(&old_dir).unwrap();
    
    println!("=== test_find_files_single_pattern ===");
    println!("  Pattern: *.txt");
    println!("  Files found: {}", result.len());
    for file in &result {
        println!("    {}", file.display());
    }
    
    assert_eq!(result.len(), 2);
    assert!(result.iter().any(|f| f.file_name().unwrap() == "file1.txt"));
    assert!(result.iter().any(|f| f.file_name().unwrap() == "file2.txt"));
    assert!(!result.iter().any(|f| f.file_name().unwrap() == "file3.jpg"));
}

#[tokio::test]
async fn test_find_files_multiple_patterns() {
    let temp_dir = TempDir::new().unwrap();
    let files = vec![
        temp_dir.path().join("file1.txt"),
        temp_dir.path().join("file2.jpg"),
        temp_dir.path().join("file3.png"),
    ];
    
    for file in &files {
        fs::write(file, "test").await.unwrap();
    }
    
    let temp_path = temp_dir.path().canonicalize().unwrap();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_path).unwrap();
    let _keep_alive = &temp_dir;
    
    let patterns = vec!["*.txt".to_string(), "*.jpg".to_string()];
    let result = find_files(&patterns, false, &[]).await.unwrap();
    
    // Restore directory before assertions
    std::env::set_current_dir(&old_dir).unwrap();
    
    println!("=== test_find_files_multiple_patterns ===");
    println!("  Patterns: *.txt, *.jpg");
    println!("  Files found: {}", result.len());
    for file in &result {
        println!("    {}", file.display());
    }
    
    assert_eq!(result.len(), 2);
    assert!(result.iter().any(|f| f.file_name().unwrap() == "file1.txt"));
    assert!(result.iter().any(|f| f.file_name().unwrap() == "file2.jpg"));
    assert!(!result.iter().any(|f| f.file_name().unwrap() == "file3.png"));
}

#[tokio::test]
async fn test_find_files_with_exclude() {
    let temp_dir = TempDir::new().unwrap();
    let files = vec![
        temp_dir.path().join("file1.txt"),
        temp_dir.path().join("file2.txt"),
        temp_dir.path().join("backup.txt"),
    ];
    
    for file in &files {
        fs::write(file, "test").await.unwrap();
    }
    
    let temp_path = temp_dir.path().canonicalize().unwrap();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_path).unwrap();
    let _keep_alive = &temp_dir;
    
    let patterns = vec!["*.txt".to_string()];
    let exclude = vec!["*backup*".to_string()];
    let result = find_files(&patterns, false, &exclude).await.unwrap();
    
    // Restore directory before assertions
    std::env::set_current_dir(&old_dir).unwrap();
    
    println!("=== test_find_files_with_exclude ===");
    println!("  Pattern: *.txt");
    println!("  Exclude: *backup*");
    println!("  Files found: {}", result.len());
    for file in &result {
        println!("    {}", file.display());
    }
    
    assert_eq!(result.len(), 2);
    assert!(result.iter().any(|f| f.file_name().unwrap() == "file1.txt"));
    assert!(result.iter().any(|f| f.file_name().unwrap() == "file2.txt"));
    assert!(!result.iter().any(|f| f.file_name().unwrap() == "backup.txt"));
}

#[tokio::test]
async fn test_find_files_recursive() {
    let temp_dir = TempDir::new().unwrap();
    let subdir = temp_dir.path().join("subdir");
    fs::create_dir_all(&subdir).await.unwrap();
    
    let files = vec![
        temp_dir.path().join("file1.txt"),
        temp_dir.path().join("file2.txt"),
        subdir.join("file3.txt"),
    ];
    
    for file in &files {
        fs::write(file, "test").await.unwrap();
    }
    
    let temp_path = temp_dir.path().canonicalize().unwrap();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_path).unwrap();
    let _keep_alive = &temp_dir;
    
    let patterns = vec!["*.txt".to_string()];
    let result_non_recursive = find_files(&patterns, false, &[]).await.unwrap();
    let result_recursive = find_files(&patterns, true, &[]).await.unwrap();
    
    // Restore directory before assertions
    std::env::set_current_dir(&old_dir).unwrap();
    
    println!("=== test_find_files_recursive ===");
    println!("  Pattern: *.txt");
    println!("  Non-recursive: {} files", result_non_recursive.len());
    println!("  Recursive: {} files", result_recursive.len());
    
    // Non-recursive should only find root files
    assert_eq!(result_non_recursive.len(), 2);
    assert!(result_non_recursive.iter().any(|f| f.file_name().unwrap() == "file1.txt"));
    assert!(result_non_recursive.iter().any(|f| f.file_name().unwrap() == "file2.txt"));
    
    // Recursive should find all files
    assert_eq!(result_recursive.len(), 3);
    assert!(result_recursive.iter().any(|f| f.file_name().unwrap() == "file1.txt"));
    assert!(result_recursive.iter().any(|f| f.file_name().unwrap() == "file2.txt"));
    assert!(result_recursive.iter().any(|f| f.file_name().unwrap() == "file3.txt"));
}

#[tokio::test]
async fn test_find_files_multiple_excludes() {
    let temp_dir = TempDir::new().unwrap();
    let files = vec![
        temp_dir.path().join("file1.txt"),
        temp_dir.path().join("backup.txt"),
        temp_dir.path().join("temp.txt"),
    ];
    
    for file in &files {
        fs::write(file, "test").await.unwrap();
    }
    
    let temp_path = temp_dir.path().canonicalize().unwrap();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_path).unwrap();
    let _keep_alive = &temp_dir;
    
    let patterns = vec!["*.txt".to_string()];
    let exclude = vec!["*backup*".to_string(), "*temp*".to_string()];
    let result = find_files(&patterns, false, &exclude).await.unwrap();
    
    // Restore directory before assertions
    std::env::set_current_dir(&old_dir).unwrap();
    
    println!("=== test_find_files_multiple_excludes ===");
    println!("  Pattern: *.txt");
    println!("  Exclude: *backup*, *temp*");
    println!("  Files found: {}", result.len());
    for file in &result {
        println!("    {}", file.display());
    }
    
    assert_eq!(result.len(), 1);
    assert!(result.iter().any(|f| f.file_name().unwrap() == "file1.txt"));
    assert!(!result.iter().any(|f| f.file_name().unwrap() == "backup.txt"));
    assert!(!result.iter().any(|f| f.file_name().unwrap() == "temp.txt"));
}

#[tokio::test]
async fn test_display_files() {
    let files = vec![
        PathBuf::from("file1.txt"),
        PathBuf::from("file2.txt"),
    ];
    
    println!("=== test_display_files (filename only) ===");
    display_files(&files, false);
    // Just verify it doesn't panic - output is tested via integration tests
}

#[tokio::test]
async fn test_display_files_fullpath() {
    let files = vec![
        PathBuf::from("/path/to/file1.txt"),
        PathBuf::from("/path/to/file2.txt"),
    ];
    
    println!("=== test_display_files (fullpath) ===");
    display_files(&files, true);
    // Just verify it doesn't panic - output is tested via integration tests
}

#[tokio::test]
async fn test_display_files_empty() {
    let files = Vec::<PathBuf>::new();
    
    println!("=== test_display_files_empty ===");
    display_files(&files, false);
    // Just verify it doesn't panic
}

