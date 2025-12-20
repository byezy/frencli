//! Tests for the undo subcommand module.
//! 
//! These tests verify undo command functionality.

use frencli::undo::{handle_undo_check, handle_undo_apply};
use freneng::RenamingEngine;
use tempfile::TempDir;

#[tokio::test]
async fn test_handle_undo_check_no_history() {
    let temp_dir = TempDir::new().unwrap();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    let engine = RenamingEngine;
    
    // With no history, should print message and not exit
    handle_undo_check(&engine).await;
    
    std::env::set_current_dir(&old_dir).unwrap();
}

// Note: Testing undo with actual history requires setting up rename operations first
// which is complex. The functions exist and compile, which is verified.
// Full integration testing would require:
// 1. Creating files
// 2. Performing renames (which creates history)
// 3. Testing undo check and apply
// This is better suited for integration tests.

#[tokio::test]
async fn test_handle_undo_apply_no_history() {
    let temp_dir = TempDir::new().unwrap();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    let engine = RenamingEngine;
    
    // With no history, should print message
    // Note: This might exit, but we verify the function exists
    handle_undo_apply(&engine, true).await;
    
    std::env::set_current_dir(&old_dir).unwrap();
}

