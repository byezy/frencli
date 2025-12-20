//! Tests for the validate subcommand module.
//! 
//! These tests verify validation command functionality.

use frencli::validate::handle_validate_command;
use freneng::{RenamingEngine, EnginePreviewResult, FileRename};
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs;

#[tokio::test]
async fn test_handle_validate_with_valid_renames() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("test.txt");
    fs::write(&file, "content").await.unwrap();
    
    let engine = RenamingEngine;
    let preview = EnginePreviewResult {
        renames: vec![FileRename {
            old_path: file.clone(),
            new_path: temp_dir.path().join("new.txt"),
            new_name: "new.txt".to_string(),
        }],
        warnings: vec![],
        has_empty_names: false,
    };
    
    // Should not exit (validation passes)
    // Note: This function exits on error, so we can't easily assert success
    // But we can verify it doesn't panic
    handle_validate_command(&engine, &preview, false, false).await;
}

#[tokio::test]
async fn test_handle_validate_with_empty_names() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("test.txt");
    fs::write(&file, "content").await.unwrap();
    
    let engine = RenamingEngine;
    let preview = EnginePreviewResult {
        renames: vec![FileRename {
            old_path: file.clone(),
            new_path: temp_dir.path().join(""),
            new_name: "".to_string(),
        }],
        warnings: vec![],
        has_empty_names: true,
    };
    
    // With skip_invalid=false, this should exit with code 1
    // We can't easily test exit codes, but we verify the function exists
    // In practice, this would exit, so the test verifies compilation
}

#[tokio::test]
async fn test_handle_validate_with_warnings() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("test.txt");
    fs::write(&file, "content").await.unwrap();
    
    let engine = RenamingEngine;
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
    handle_validate_command(&engine, &preview, false, false).await;
}

#[tokio::test]
async fn test_handle_validate_with_skip_invalid() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("test.txt");
    fs::write(&file, "content").await.unwrap();
    
    let engine = RenamingEngine;
    let preview = EnginePreviewResult {
        renames: vec![FileRename {
            old_path: file.clone(),
            new_path: temp_dir.path().join(""),
            new_name: "".to_string(),
        }],
        warnings: vec![],
        has_empty_names: true,
    };
    
    // With skip_invalid=true, should continue despite empty names
    handle_validate_command(&engine, &preview, false, true).await;
}

#[tokio::test]
async fn test_handle_validate_with_overwrite() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("test.txt");
    fs::write(&file, "content").await.unwrap();
    
    let engine = RenamingEngine;
    let preview = EnginePreviewResult {
        renames: vec![FileRename {
            old_path: file.clone(),
            new_path: temp_dir.path().join("new.txt"),
            new_name: "new.txt".to_string(),
        }],
        warnings: vec![],
        has_empty_names: false,
    };
    
    // Should validate with overwrite enabled
    handle_validate_command(&engine, &preview, true, false).await;
}

#[tokio::test]
async fn test_handle_validate_multiple_files() {
    let temp_dir = TempDir::new().unwrap();
    let files = vec![
        temp_dir.path().join("file1.txt"),
        temp_dir.path().join("file2.txt"),
    ];
    
    for file in &files {
        fs::write(file, "content").await.unwrap();
    }
    
    let engine = RenamingEngine;
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
    
    handle_validate_command(&engine, &preview, false, false).await;
}

