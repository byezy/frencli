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

// ============================================================================
// Tests using test_data structure
// ============================================================================

#[tokio::test]
async fn test_find_files_in_test_data_nested_structure() {
    // Test finding files in the nested test_data structure
    let test_data_path = PathBuf::from("../test_data");
    if !test_data_path.exists() {
        println!("=== test_find_files_in_test_data_nested_structure ===");
        println!("  Skipping: test_data directory not found. Run generate_test_data.sh first.");
        return;
    }
    
    let test_data_path = test_data_path.canonicalize().unwrap();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&test_data_path).unwrap();
    
    let patterns = vec!["*.txt".to_string()];
    let result_non_recursive = find_files(&patterns, false, &[]).await.unwrap();
    let result_recursive = find_files(&patterns, true, &[]).await.unwrap();
    
    std::env::set_current_dir(&old_dir).unwrap();
    
    println!("=== test_find_files_in_test_data_nested_structure ===");
    println!("  Pattern: *.txt");
    println!("  Non-recursive: {} files", result_non_recursive.len());
    println!("  Recursive: {} files", result_recursive.len());
    
    // Recursive should find more files (including in Documents, Documents/Projects, Documents/Archive, etc.)
    assert!(result_recursive.len() > result_non_recursive.len(), 
        "Recursive should find more files than non-recursive");
    assert!(result_recursive.len() >= 5, "Should find multiple txt files in nested structure");
    
    // Verify files are from test_data
    for file in &result_recursive {
        assert!(file.to_string_lossy().contains("test_data"),
            "File should be in test_data: {}", file.display());
    }
}

#[tokio::test]
async fn test_find_files_in_nested_directories() {
    // Test finding files in nested Documents structure
    let test_data_path = PathBuf::from("../test_data");
    if !test_data_path.exists() {
        println!("=== test_find_files_in_nested_directories ===");
        println!("  Skipping: test_data directory not found. Run generate_test_data.sh first.");
        return;
    }
    
    let test_data_path = test_data_path.canonicalize().unwrap();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&test_data_path).unwrap();
    
    let patterns = vec!["Documents/**/*.txt".to_string()];
    let result = find_files(&patterns, true, &[]).await.unwrap();
    
    // Restore directory before assertions
    std::env::set_current_dir(&old_dir).unwrap();
    
    println!("=== test_find_files_in_nested_directories ===");
    println!("  Pattern: Documents/**/*.txt");
    println!("  Files found: {}", result.len());
    for file in &result {
        println!("    {}", file.display());
    }
    
    // Should find files in Documents, Documents/Projects, Documents/Archive
    assert!(result.len() >= 3, "Should find multiple txt files in Documents structure");
    
    // Verify files are in Documents directory structure
    for file in &result {
        assert!(file.to_string_lossy().contains("Documents"),
            "File should be in Documents: {}", file.display());
    }
}

#[tokio::test]
async fn test_find_files_with_exclude_in_nested_structure() {
    // Test exclude patterns with nested structure
    let test_data_path = PathBuf::from("../test_data");
    if !test_data_path.exists() {
        println!("=== test_find_files_with_exclude_in_nested_structure ===");
        println!("  Skipping: test_data directory not found. Run generate_test_data.sh first.");
        return;
    }
    
    let test_data_path = test_data_path.canonicalize().unwrap();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&test_data_path).unwrap();
    
    let patterns = vec!["**/*.txt".to_string()];
    let exclude = vec!["*backup*".to_string(), "*Archive*".to_string()];
    let result = find_files(&patterns, true, &exclude).await.unwrap();
    
    // Restore directory before assertions
    std::env::set_current_dir(&old_dir).unwrap();
    
    println!("=== test_find_files_with_exclude_in_nested_structure ===");
    println!("  Pattern: **/*.txt");
    println!("  Exclude: *backup*, *Archive*");
    println!("  Files found: {}", result.len());
    
    // Verify excluded patterns are not in results
    for file in &result {
        let file_str = file.to_string_lossy();
        assert!(!file_str.contains("backup"), 
            "Should not find backup files: {}", file.display());
        assert!(!file_str.contains("Archive"), 
            "Should not find Archive files: {}", file.display());
    }
}

#[tokio::test]
async fn test_find_files_in_photos_nested_structure() {
    // Test finding files in Photos nested structure
    let test_data_path = PathBuf::from("../test_data");
    if !test_data_path.exists() {
        println!("=== test_find_files_in_photos_nested_structure ===");
        println!("  Skipping: test_data directory not found. Run generate_test_data.sh first.");
        return;
    }
    
    let test_data_path = test_data_path.canonicalize().unwrap();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&test_data_path).unwrap();
    
    let patterns = vec!["Photos/**/*.jpg".to_string()];
    let result = find_files(&patterns, true, &[]).await.unwrap();
    
    std::env::set_current_dir(&old_dir).unwrap();
    
    println!("=== test_find_files_in_photos_nested_structure ===");
    println!("  Pattern: Photos/**/*.jpg");
    println!("  Files found: {}", result.len());
    
    // Should find files in Photos, Photos/Vacation, Photos/Vacation/2024
    assert!(result.len() >= 5, "Should find multiple jpg files in Photos nested structure");
    
    // Verify files are in Photos directory structure
    for file in &result {
        assert!(file.to_string_lossy().contains("Photos"),
            "File should be in Photos: {}", file.display());
    }
    
    // Should find files in nested directories
    let has_nested = result.iter().any(|f| f.to_string_lossy().contains("Vacation"));
    assert!(has_nested, "Should find files in nested Vacation directory");
}

#[tokio::test]
async fn test_find_files_in_logs_structure() {
    // Test finding files in Logs nested structure
    let test_data_path = PathBuf::from("../test_data");
    if !test_data_path.exists() {
        println!("=== test_find_files_in_logs_structure ===");
        println!("  Skipping: test_data directory not found. Run generate_test_data.sh first.");
        return;
    }
    
    let test_data_path = test_data_path.canonicalize().unwrap();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&test_data_path).unwrap();
    
    let patterns = vec!["Logs/**/*.log".to_string()];
    let result = find_files(&patterns, true, &[]).await.unwrap();
    
    std::env::set_current_dir(&old_dir).unwrap();
    
    println!("=== test_find_files_in_logs_structure ===");
    println!("  Pattern: Logs/**/*.log");
    println!("  Files found: {}", result.len());
    
    // Should find files in Logs, Logs/Application, Logs/System
    assert!(result.len() >= 3, "Should find multiple log files in Logs structure");
    
    // Verify files are in Logs directory structure
    for file in &result {
        assert!(file.to_string_lossy().contains("Logs"),
            "File should be in Logs: {}", file.display());
    }
}
