//! Tests for the list subcommand module.
//! 
//! These tests verify file finding, pattern matching, recursion, and exclusion functionality.
//! All tests are async to match the async API of the list module.

use frencli::list::{find_files, display_files};
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs;
mod test_utils;
use test_utils::setup_test_data_async;

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
    
    let temp_path = temp_dir.path().canonicalize().unwrap();
    let _keep_alive = &temp_dir;
    
    let pattern = temp_path.join("*.txt").to_string_lossy().to_string();
    let patterns = vec![pattern];
    let result = find_files(&patterns, false, &[]).await.unwrap();
    
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
    let _keep_alive = &temp_dir;
    
    let pattern1 = temp_path.join("*.txt").to_string_lossy().to_string();
    let pattern2 = temp_path.join("*.jpg").to_string_lossy().to_string();
    let patterns = vec![pattern1, pattern2];
    let result = find_files(&patterns, false, &[]).await.unwrap();
    
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
    let _keep_alive = &temp_dir;
    
    let pattern = temp_path.join("*.txt").to_string_lossy().to_string();
    let patterns = vec![pattern];
    let exclude = vec!["*backup*".to_string()];
    let result = find_files(&patterns, false, &exclude).await.unwrap();
    
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
    let _keep_alive = &temp_dir;
    
    let pattern = temp_path.join("*.txt").to_string_lossy().to_string();
    let patterns = vec![pattern.clone()];
    let result_non_recursive = find_files(&patterns, false, &[]).await.unwrap();
    let result_recursive = find_files(&patterns, true, &[]).await.unwrap();
    
    assert_eq!(result_non_recursive.len(), 2);
    assert!(result_non_recursive.iter().any(|f| f.file_name().unwrap() == "file1.txt"));
    assert!(result_non_recursive.iter().any(|f| f.file_name().unwrap() == "file2.txt"));
    
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
    let _keep_alive = &temp_dir;
    
    let pattern = temp_path.join("*.txt").to_string_lossy().to_string();
    let patterns = vec![pattern];
    let exclude = vec!["*backup*".to_string(), "*temp*".to_string()];
    let result = find_files(&patterns, false, &exclude).await.unwrap();
    
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
    
    display_files(&files, false);
}

#[tokio::test]
async fn test_display_files_fullpath() {
    let files = vec![
        PathBuf::from("/path/to/file1.txt"),
        PathBuf::from("/path/to/file2.txt"),
    ];
    
    display_files(&files, true);
}

#[tokio::test]
async fn test_display_files_empty() {
    let files = Vec::<PathBuf>::new();
    display_files(&files, false);
}

// ============================================================================
// Tests using test_data structure
// ============================================================================

#[tokio::test]
async fn test_find_files_in_test_data_nested_structure() {
    let (temp_dir, test_data_path) = match setup_test_data_async().await {
        Some(x) => x,
        None => return,
    };
    
    let _keep_alive = &temp_dir;
    let test_data_path = test_data_path.canonicalize().unwrap();
    
    let pattern = test_data_path.join("*.txt").to_string_lossy().to_string();
    let patterns = vec![pattern];
    let result_non_recursive = find_files(&patterns, false, &[]).await.unwrap();
    let result_recursive = find_files(&patterns, true, &[]).await.unwrap();
    
    assert!(result_recursive.len() > result_non_recursive.len());
    assert!(result_recursive.len() >= 5);
}

#[tokio::test]
async fn test_find_files_in_nested_directories() {
    let (temp_dir, test_data_path) = match setup_test_data_async().await {
        Some(x) => x,
        None => return,
    };
    
    let _keep_alive = &temp_dir;
    let test_data_path = test_data_path.canonicalize().unwrap();
    
    let pattern = test_data_path.join("Documents").join("**").join("*.txt").to_string_lossy().to_string();
    let patterns = vec![pattern];
    let result = find_files(&patterns, true, &[]).await.unwrap();
    
    assert!(result.len() >= 3);
    for file in &result {
        assert!(file.to_string_lossy().contains("Documents"));
    }
}

#[tokio::test]
async fn test_find_files_with_exclude_in_nested_structure() {
    let (temp_dir, test_data_path) = match setup_test_data_async().await {
        Some(x) => x,
        None => return,
    };
    
    let _keep_alive = &temp_dir;
    let test_data_path = test_data_path.canonicalize().unwrap();
    
    let pattern = test_data_path.join("**").join("*.txt").to_string_lossy().to_string();
    let patterns = vec![pattern];
    let exclude = vec!["*backup*".to_string(), "*Archive*".to_string()];
    let result = find_files(&patterns, true, &exclude).await.unwrap();
    
    for file in &result {
        let file_str = file.to_string_lossy();
        assert!(!file_str.contains("backup"));
        assert!(!file_str.contains("Archive"));
    }
}

#[tokio::test]
async fn test_find_files_in_photos_nested_structure() {
    let (temp_dir, test_data_path) = match setup_test_data_async().await {
        Some(x) => x,
        None => return,
    };
    
    let _keep_alive = &temp_dir;
    let test_data_path = test_data_path.canonicalize().unwrap();
    
    let pattern = test_data_path.join("Photos").join("**").join("*.jpg").to_string_lossy().to_string();
    let patterns = vec![pattern];
    let result = find_files(&patterns, true, &[]).await.unwrap();
    
    assert!(result.len() >= 5);
    for file in &result {
        assert!(file.to_string_lossy().contains("Photos"));
    }
    
    let has_nested = result.iter().any(|f| f.to_string_lossy().contains("Vacation"));
    assert!(has_nested);
}

#[tokio::test]
async fn test_find_files_in_logs_structure() {
    let (temp_dir, test_data_path) = match setup_test_data_async().await {
        Some(x) => x,
        None => return,
    };
    
    let _keep_alive = &temp_dir;
    let test_data_path = test_data_path.canonicalize().unwrap();
    
    let pattern = test_data_path.join("Logs").join("**").join("*.log").to_string_lossy().to_string();
    let patterns = vec![pattern];
    let result = find_files(&patterns, true, &[]).await.unwrap();
    
    assert!(result.len() >= 3);
    for file in &result {
        assert!(file.to_string_lossy().contains("Logs"));
    }
}
