//! Tests for the audit subcommand module.
//! 
//! These tests verify audit command functionality.
//! 
//! Note: These tests change the current directory, so they must run sequentially
//! to avoid race conditions. A static mutex ensures tests don't run in parallel.

use frencli::audit::handle_audit_command;
use freneng::audit::{log_audit_entry, clear_audit_log};
use std::path::PathBuf;
use std::sync::Mutex;
use tempfile::TempDir;

// Mutex to ensure tests run sequentially (they change the current directory)
static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[tokio::test]
async fn test_handle_audit_no_entries() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let temp_dir = TempDir::new().unwrap();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();
    let _keep_alive = &temp_dir;
    
    // Should not panic with no entries
    let result = handle_audit_command(None, false).await;
    assert!(result.is_ok());
    
    std::env::set_current_dir(&old_dir).unwrap();
}

#[tokio::test]
async fn test_handle_audit_with_entries() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let temp_dir = TempDir::new().unwrap();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();
    let _keep_alive = &temp_dir;
    
    // Create some audit entries
    for i in 0..3 {
        let result = log_audit_entry(
            &format!("fren list file{}.txt rename \"%N_backup.%E\" --yes", i),
            Some("%N_backup.%E".to_string()),
            PathBuf::from("."),
            vec![(PathBuf::from(format!("file{}.txt", i)), PathBuf::from(format!("file{}_backup.txt", i)))],
            vec![],
            vec![],
        ).await;
        assert!(result.is_ok(), "Failed to log entry {}: {:?}", i, result);
    }
    
    // Verify file exists before reading
    let audit_file = std::env::current_dir().unwrap().join(".fren_audit.log");
    assert!(audit_file.exists(), "Audit file should exist at: {}", audit_file.display());
    
    // Should display entries
    let result = handle_audit_command(None, false).await;
    assert!(result.is_ok(), "Failed to handle audit command: {:?}", result);
    
    std::env::set_current_dir(&old_dir).unwrap();
}

#[tokio::test]
async fn test_handle_audit_with_limit() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let temp_dir = TempDir::new().unwrap();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();
    let _keep_alive = &temp_dir;
    
    // Create multiple entries
    for i in 0..10 {
        let result = log_audit_entry(
            &format!("fren rename \"%N{}.%E\" --yes", i),
            Some(format!("%N{}.%E", i)),
            PathBuf::from("."),
            vec![],
            vec![],
            vec![],
        ).await;
        assert!(result.is_ok(), "Failed to log entry {}: {:?}", i, result);
    }
    
    // Verify file exists before reading
    let audit_file = std::env::current_dir().unwrap().join(".fren_audit.log");
    assert!(audit_file.exists());
    
    // Should limit to 5 most recent
    let result = handle_audit_command(Some(5), false).await;
    assert!(result.is_ok());
    
    std::env::set_current_dir(&old_dir).unwrap();
}

#[tokio::test]
async fn test_handle_audit_json_output() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let temp_dir = TempDir::new().unwrap();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();
    let _keep_alive = &temp_dir;
    
    // Create an entry
    let result = log_audit_entry(
        "fren list *.txt rename \"%N.%E\" --yes",
        Some("%N.%E".to_string()),
        PathBuf::from("."),
        vec![],
        vec![],
        vec![],
    ).await;
    assert!(result.is_ok());
    
    // Verify file exists before reading
    let audit_file = std::env::current_dir().unwrap().join(".fren_audit.log");
    assert!(audit_file.exists());
    
    // Should output as JSON
    let result = handle_audit_command(None, true).await;
    assert!(result.is_ok());
    
    std::env::set_current_dir(&old_dir).unwrap();
}

