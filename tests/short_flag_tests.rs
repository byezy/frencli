//! Integration tests for short flag rejection behavior.
//! 
//! These tests verify that short flags (like -y, -o) are properly rejected
//! in invalid contexts while being allowed in valid contexts (filenames/patterns).

use std::process::{Command, Stdio};
use std::path::Path;

fn get_binary_path() -> std::path::PathBuf {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let target_dir = workspace_root.join("target").join("debug");
    let binary_name = if cfg!(target_os = "windows") { "fren.exe" } else { "fren" };
    target_dir.join(binary_name)
}

#[test]
fn test_short_flag_rejected_for_rename() {
    let binary = get_binary_path();
    
    let output = Command::new(&binary)
        .arg("rename")
        .arg("-y")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    
    assert!(!output.status.success(), "Command should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Short flags (like '-y') are not supported"), 
            "Should show short flag error. Stderr: {}", stderr);
    assert!(stderr.contains("--yes"), "Should suggest --yes");
}

#[test]
fn test_short_flag_rejected_for_undo_apply() {
    let binary = get_binary_path();
    
    let output = Command::new(&binary)
        .arg("undo")
        .arg("--apply")
        .arg("-y")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    
    assert!(!output.status.success(), "Command should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Short flags (like '-y') are not supported"), 
            "Should show short flag error. Stderr: {}", stderr);
    assert!(stderr.contains("--yes"), "Should suggest --yes");
}

#[test]
fn test_short_flag_rejected_for_undo_check() {
    let binary = get_binary_path();
    
    let output = Command::new(&binary)
        .arg("undo")
        .arg("--check")
        .arg("-y")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    
    assert!(!output.status.success(), "Command should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Short flags (like '-y') are not supported"), 
            "Should show short flag error. Stderr: {}", stderr);
}

#[test]
fn test_short_flag_rejected_for_validate() {
    let binary = get_binary_path();
    
    let output = Command::new(&binary)
        .arg("validate")
        .arg("-y")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    
    assert!(!output.status.success(), "Command should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Short flags (like '-y') are not supported"), 
            "Should show short flag error. Stderr: {}", stderr);
}

#[test]
fn test_short_flag_rejected_for_template() {
    let binary = get_binary_path();
    
    let output = Command::new(&binary)
        .arg("template")
        .arg("-y")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    
    assert!(!output.status.success(), "Command should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Short flags (like '-y') are not supported"), 
            "Should show short flag error. Stderr: {}", stderr);
}

#[test]
fn test_short_flag_rejected_for_audit() {
    let binary = get_binary_path();
    
    let output = Command::new(&binary)
        .arg("audit")
        .arg("-y")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    
    assert!(!output.status.success(), "Command should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Short flags (like '-y') are not supported"), 
            "Should show short flag error. Stderr: {}", stderr);
}

#[test]
fn test_short_flag_rejected_when_not_positional() {
    let binary = get_binary_path();
    
    // Test that short flags are rejected for subcommands that don't accept positional args
    // For subcommands that DO accept positional args (list, make), -X is treated as a filename
    let test_cases: Vec<Vec<&str>> = vec![
        vec!["rename", "-y"],   // rename doesn't accept positional args, so -y is rejected
        vec!["rename", "-o"],   // rename doesn't accept positional args, so -o is rejected
        vec!["undo", "--apply", "-y"],  // undo doesn't accept positional args after --apply
    ];
    
    for test_case in test_cases {
        let mut cmd = Command::new(&binary);
        for arg in &test_case {
            cmd.arg(arg);
        }
        let output = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        
        assert!(!output.status.success(), 
                "Command should fail. Args: {:?}", test_case);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Short flags"), 
                "Should show short flag error. Stderr: {}", stderr);
    }
}

#[test]
fn test_single_dash_allowed_as_filename() {
    // Single dash arguments (like -y) are allowed as filenames/patterns
    // Only --<something> is interpreted as flags
    let temp_dir = tempfile::TempDir::new().unwrap();
    let test_file = temp_dir.path().join("-y");
    std::fs::write(&test_file, "test content").unwrap();
    
    let binary = get_binary_path();
    
    let output = Command::new(&binary)
        .arg("list")
        .arg("-y")
        .current_dir(temp_dir.path())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    
    // Should succeed - -y is treated as a filename, not a flag
    assert!(output.status.success(), 
            "Should allow -y as filename. Stderr: {}", 
            String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("-y") || stdout.contains("1 matching"), 
            "Should find the file. Stdout: {}", stdout);
}

#[test]
fn test_short_flag_allowed_as_exclude_pattern() {
    let binary = get_binary_path();
    
    // This should parse correctly (even if no files match)
    let output = Command::new(&binary)
        .arg("list")
        .arg("*.txt")
        .arg("--exclude")
        .arg("-y")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    
    // Should not fail with short flag error
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("Short flags (like '-y') are not supported"), 
            "Should allow -y as exclude pattern. Stderr: {}", stderr);
}

#[test]
fn test_multiple_short_flags_rejected() {
    let binary = get_binary_path();
    
    let output = Command::new(&binary)
        .arg("rename")
        .arg("-y")
        .arg("-o")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    
    assert!(!output.status.success(), "Command should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Should fail on the first short flag encountered
    assert!(stderr.contains("Short flags"), 
            "Should show short flag error. Stderr: {}", stderr);
}

#[test]
fn test_short_flag_error_message_helpful() {
    let binary = get_binary_path();
    
    let output = Command::new(&binary)
        .arg("rename")
        .arg("-y")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Check that error message includes helpful mappings
    assert!(stderr.contains("Short flags"), "Should mention short flags");
    assert!(stderr.contains("--yes"), "Should suggest --yes");
    assert!(stderr.contains("--overwrite"), "Should show --overwrite mapping");
    assert!(stderr.contains("--recursive"), "Should show --recursive mapping");
    assert!(stderr.contains("--exclude"), "Should show --exclude mapping");
}

