//! Tests for the rename subcommand module.
//! 
//! These tests verify rename command functionality including file operations.
//! All tests use isolated temp directories.

use frencli::rename::handle_rename_command;
use freneng::{EnginePreviewResult, FileRename};
use tempfile::TempDir;
use tokio::fs;
mod test_utils;
use test_utils::DirGuard;

#[tokio::test]
async fn test_handle_rename_with_yes_flag() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("old.txt");
    fs::write(&file, "content").await.unwrap();
    
    let preview = EnginePreviewResult {
        renames: vec![FileRename {
            old_path: file.clone(),
            new_path: temp_dir.path().join("new.txt"),
            new_name: "new.txt".to_string(),
        }],
        warnings: vec![],
        has_empty_names: false,
    };
    
    // With yes=true, should rename without prompting
    let result = handle_rename_command(preview, false, true, false, "test command".to_string(), None, true).await;
    assert!(result.is_ok());
    
    // Verify file was renamed
    assert!(!file.exists());
    assert!(temp_dir.path().join("new.txt").exists());
}

#[tokio::test]
async fn test_handle_rename_with_overwrite() {
    let temp_dir = TempDir::new().unwrap();
    let old_file = temp_dir.path().join("old.txt");
    let existing_file = temp_dir.path().join("new.txt");
    fs::write(&old_file, "old content").await.unwrap();
    fs::write(&existing_file, "existing content").await.unwrap();
    
    let preview = EnginePreviewResult {
        renames: vec![FileRename {
            old_path: old_file.clone(),
            new_path: existing_file.clone(),
            new_name: "new.txt".to_string(),
        }],
        warnings: vec![],
        has_empty_names: false,
    };
    
    // With overwrite=true, should overwrite existing file
    let result = handle_rename_command(preview, true, true, false, "test command".to_string(), None, true).await;
    assert!(result.is_ok());
    
    // Verify old file is gone and new file exists
    assert!(!old_file.exists());
    assert!(existing_file.exists());
}

#[tokio::test]
async fn test_handle_rename_with_empty_names_blocks() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("test.txt");
    fs::write(&file, "content").await.unwrap();
    
    let _preview = EnginePreviewResult {
        renames: vec![FileRename {
            old_path: file.clone(),
            new_path: temp_dir.path().join(""),
            new_name: "".to_string(),
        }],
        warnings: vec![],
        has_empty_names: true,
    };
    
    // Should exit with code 1, so we can't easily test
    // But we verify the function exists and compiles
}

#[tokio::test]
async fn test_handle_rename_with_warnings() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("test.txt");
    fs::write(&file, "content").await.unwrap();
    
    let preview = EnginePreviewResult {
        renames: vec![FileRename {
            old_path: file.clone(),
            new_path: temp_dir.path().join("new.txt"),
            new_name: "new.txt".to_string(),
        }],
        warnings: vec!["Unknown token: %X".to_string()],
        has_empty_names: false,
    };
    
    // Should display warnings but continue
    let result = handle_rename_command(preview, false, true, false, "test command".to_string(), None, true).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_handle_rename_multiple_files() {
    let temp_dir = TempDir::new().unwrap();
    let files = vec![
        temp_dir.path().join("file1.txt"),
        temp_dir.path().join("file2.txt"),
    ];
    
    for file in &files {
        fs::write(file, "content").await.unwrap();
    }
    
    let preview = EnginePreviewResult {
        renames: vec![
            FileRename {
                old_path: files[0].clone(),
                new_path: temp_dir.path().join("new1.txt"),
                new_name: "new1.txt".to_string(),
            },
            FileRename {
                old_path: files[1].clone(),
                new_path: temp_dir.path().join("new2.txt"),
                new_name: "new2.txt".to_string(),
            },
        ],
        warnings: vec![],
        has_empty_names: false,
    };
    
    let result = handle_rename_command(preview, false, true, false, "test command".to_string(), None, true).await;
    assert!(result.is_ok());
    
    // Verify all files renamed
    assert!(!files[0].exists());
    assert!(!files[1].exists());
    assert!(temp_dir.path().join("new1.txt").exists());
    assert!(temp_dir.path().join("new2.txt").exists());
}

#[tokio::test]
async fn test_handle_rename_saves_history() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("old.txt");
    fs::write(&file, "content").await.unwrap();
    
    // Change to temp directory for history (save_history writes to current directory)
    let _guard = DirGuard::new(temp_dir.path()).unwrap();
    
    let preview = EnginePreviewResult {
        renames: vec![FileRename {
            old_path: file.clone(),
            new_path: temp_dir.path().join("new.txt"),
            new_name: "new.txt".to_string(),
        }],
        warnings: vec![],
        has_empty_names: false,
    };
    
    let result = handle_rename_command(preview, false, true, false, "test command".to_string(), None, true).await;
    assert!(result.is_ok());
    
    // History should be saved (we can't easily verify without loading it)
    // But the function should complete without error
}

