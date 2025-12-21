//! Tests for the audit subcommand module.
//! 
//! These tests verify audit command functionality.
//! All tests use isolated temp directories without changing the global working directory.

use frencli::audit::handle_audit_command;
use freneng::audit::log_audit_entry;
use std::path::PathBuf;
use tempfile::TempDir;
mod test_utils;
use test_utils::DirGuard;

#[tokio::test]
async fn test_handle_audit_no_entries() {
    let temp_dir = TempDir::new().unwrap();
    let _keep_alive = &temp_dir;
    
    // Change to temp dir for read_audit_log (which reads from current directory)
    let _guard = DirGuard::new(temp_dir.path()).unwrap();
    
    // Should not panic with no entries
    let result = handle_audit_command(None, false).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_handle_audit_with_entries() {
    let temp_dir = TempDir::new().unwrap();
    let _keep_alive = &temp_dir;
    
    // Create some audit entries (using working_directory parameter, no cwd change needed)
    for i in 0..3 {
        let result = log_audit_entry(
            &format!("fren list file{}.txt rename \"%N_backup.%E\" --yes", i),
            Some("%N_backup.%E".to_string()),
            temp_dir.path().to_path_buf(),
            vec![(PathBuf::from(format!("file{}.txt", i)), PathBuf::from(format!("file{}_backup.txt", i)))],
            vec![],
            vec![],
        ).await;
        if let Err(e) = &result {
            eprintln!("Error logging entry {}: {}", i, e);
            panic!("Failed to log entry {}: {:?}", i, e);
        }
        assert!(result.is_ok(), "Failed to log entry {}: {:?}", i, result);
        
        // Immediately check if file exists after each write
        let audit_file = temp_dir.path().join(".fren_audit.log");
        if !audit_file.exists() {
            eprintln!("WARNING: Audit file does not exist immediately after write {} at: {}", i, audit_file.display());
        }
    }
    
    // Verify file exists in the temp directory
    let audit_file = temp_dir.path().join(".fren_audit.log");
    if !audit_file.exists() {
        eprintln!("Audit file does not exist at: {}", audit_file.display());
        eprintln!("Temp dir: {:?}", temp_dir.path());
        eprintln!("Temp dir exists: {}", temp_dir.path().exists());
        if let Ok(entries) = std::fs::read_dir(temp_dir.path()) {
            eprintln!("Temp dir contents: {:?}", entries.collect::<Result<Vec<_>, _>>());
        }
    }
    assert!(audit_file.exists(), "Audit file should exist at: {}", audit_file.display());
    
    // Change to temp dir for read_audit_log (which reads from current directory)
    let _guard = DirGuard::new(temp_dir.path()).unwrap();
    
    // Should display entries
    let result = handle_audit_command(None, false).await;
    assert!(result.is_ok(), "Failed to handle audit command: {:?}", result);
}

#[tokio::test]
async fn test_handle_audit_with_limit() {
    let temp_dir = TempDir::new().unwrap();
    let _keep_alive = &temp_dir;
    
    // Create multiple entries (using working_directory parameter, no cwd change needed)
    for i in 0..10 {
        let result = log_audit_entry(
            &format!("fren rename \"%N{}.%E\" --yes", i),
            Some(format!("%N{}.%E", i)),
            temp_dir.path().to_path_buf(),
            vec![],
            vec![],
            vec![],
        ).await;
        assert!(result.is_ok(), "Failed to log entry {}: {:?}", i, result);
    }
    
    // Verify file exists in the temp directory
    let audit_file = temp_dir.path().join(".fren_audit.log");
    assert!(audit_file.exists());
    
    // Change to temp dir for read_audit_log (which reads from current directory)
    let _guard = DirGuard::new(temp_dir.path()).unwrap();
    
    // Should limit to 5 most recent
    let result = handle_audit_command(Some(5), false).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_handle_audit_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let _keep_alive = &temp_dir;
    
    // Create an entry (using working_directory parameter, no cwd change needed)
    let result = log_audit_entry(
        "fren list *.txt rename \"%N.%E\" --yes",
        Some("%N.%E".to_string()),
        temp_dir.path().to_path_buf(),
        vec![],
        vec![],
        vec![],
    ).await;
    assert!(result.is_ok());
    
    // Verify file exists in the temp directory
    let audit_file = temp_dir.path().join(".fren_audit.log");
    assert!(audit_file.exists());
    
    // Change to temp dir for read_audit_log (which reads from current directory)
    let _guard = DirGuard::new(temp_dir.path()).unwrap();
    
    // Should output as JSON
    let result = handle_audit_command(None, true).await;
    assert!(result.is_ok());
}
