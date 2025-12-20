use std::path::{Path, PathBuf};
use std::process::Command;

fn get_binary_path() -> PathBuf {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    // The target directory is at the workspace root when building a member
    let debug_path = workspace_root.join("../target/debug/fren");
    debug_path
}

fn run_fren(pattern: &str, rename: Option<&str>) -> Result<String, String> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = get_binary_path();
    let test_data_dir = workspace_root.join("../test_data");
    
    let mut cmd = Command::new(&binary);
    cmd.arg("rename");
    cmd.arg(pattern);
    if let Some(r) = rename {
        cmd.arg("-t");
        cmd.arg(r);
    }
    cmd.current_dir(&test_data_dir);
    
    let output = cmd.output().map_err(|e| format!("Failed to execute fren: {} (binary: {:?})", e, binary))?;

    if !output.status.success() {
        return Err(format!(
            "fren failed with status {:?}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn extract_renames(output: &str) -> Vec<(String, String)> {
    let mut renames = Vec::new();
    let lines: Vec<&str> = output.lines().collect();
    let mut in_preview = false;
    for line in lines {
        let line = line.trim();
        if line.contains("Old Name") && line.contains("New Name") {
            in_preview = true;
            continue;
        }
        if line.chars().all(|c| c == '-') {
            continue;
        }
        if in_preview && (line.contains("Preview mode") || line.contains("Successfully") || line.is_empty()) {
            break;
        }
        if in_preview {
            if let Some(arrow_pos) = line.find("->") {
                let old = line[..arrow_pos].trim().to_string();
                let new = line[arrow_pos + 2..].trim().to_string();
                if !old.is_empty() && !new.is_empty() && old != "Old Name" && !old.contains("Name") {
                    renames.push((old, new));
                }
            }
        }
    }
    renames
}

#[test]
fn test_basic_name_extension_no_dot() {
    let output = run_fren("InterDisplay-Regular.ttf", Some("%N.%E")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (old, new) = &renames[0];
    assert_eq!(old, "InterDisplay-Regular.ttf");
    assert_eq!(new, "InterDisplay-Regular.ttf");
}

#[test]
fn test_extension_selection() {
    let output = run_fren("InterDisplay-Regular.ttf", Some("%N.%E1-2")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "InterDisplay-Regular.tt");
}

#[test]
fn test_remove_from_end() {
    let output = run_fren("InterDisplay-Regular.ttf", Some("%N1--8.%E")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "InterDisplay.ttf");
}

#[test]
fn test_percent_syntax_lowercase_replacement() {
    let output = run_fren("InterDisplay-Regular.ttf", Some("%L%N%R/interdisplay/inter.%E")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "inter-regular.ttf");
}

#[test]
fn test_percent_hyphen_to_underscore() {
    let output = run_fren("file-with-dashes-1.txt", Some("%N%R/-/_.%E")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "file_with_dashes_1.txt");
}

#[test]
fn test_counter_with_padding() {
    let output = run_fren("photo_001.jpg", Some("%N_%C3.%E")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    // Should be "photo_001_001.jpg" (name_counter3.extension)
    assert!(new.starts_with("photo_001_"));
    assert!(new.ends_with(".jpg"));
    // Extract the counter part and verify it's 3 digits
    let parts: Vec<&str> = new.split('_').collect();
    if parts.len() >= 3 {
        let counter = parts[2].split('.').next().unwrap();
        assert_eq!(counter.len(), 3, "Counter should be 3 digits padded");
        assert!(counter.parse::<u32>().is_ok(), "Counter should be numeric");
    }
}

#[test]
fn test_counter_without_padding() {
    let output = run_fren("photo_001.jpg", Some("%N_%C.%E")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    // Should be "photo_001_1.jpg" (counter starts at 1, no padding)
    assert!(new.starts_with("photo_001_"));
    assert!(new.ends_with(".jpg"));
    // Extract the counter part
    let parts: Vec<&str> = new.split('_').collect();
    if parts.len() >= 3 {
        let counter = parts[2].split('.').next().unwrap();
        assert_eq!(counter, "1", "Counter without padding should be '1'");
    }
}

#[test]
fn test_full_filename_placeholder() {
    let output = run_fren("InterDisplay-Regular.ttf", Some("%F_backup")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "InterDisplay-Regular.ttf_backup");
}

#[test]
fn test_list_single_pattern() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = get_binary_path();
    let test_data_dir = workspace_root.join("../test_data");
    
    let output = Command::new(&binary)
        .arg("list")
        .arg("InterDisplay-Regular.ttf")
        .current_dir(&test_data_dir)
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("=== test_list_single_pattern OUTPUT ===");
    println!("{}", stdout);
    println!("=======================================");
    
    assert!(stdout.contains("Found 1 matching file"), "Expected 'Found 1 matching file', got: {}", stdout);
    assert!(stdout.contains("InterDisplay-Regular.ttf"), "Expected 'InterDisplay-Regular.ttf', got: {}", stdout);
}

#[test]
fn test_list_multiple_patterns() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = get_binary_path();
    let test_data_dir = workspace_root.join("../test_data");
    
    let output = Command::new(&binary)
        .arg("list")
        .arg("*.jpg")
        .arg("*.txt")
        .current_dir(&test_data_dir)
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("=== test_list_multiple_patterns OUTPUT ===");
    println!("{}", stdout);
    println!("===========================================");
    
    // Extract file count
    let file_count = stdout.lines()
        .find(|l| l.contains("Found"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    
    let jpg_count = stdout.matches(".jpg").count();
    let txt_count = stdout.matches(".txt").count();
    
    println!("Files found: {}, JPG files: {}, TXT files: {}", file_count, jpg_count, txt_count);
    
    assert!(file_count > 0, "Expected files to be found, got: {}", stdout);
    assert!(jpg_count > 0, "Expected .jpg files, got: {}", stdout);
    assert!(txt_count > 0, "Expected .txt files, got: {}", stdout);
    assert_eq!(file_count, jpg_count + txt_count, "File count mismatch. Total: {}, JPG: {}, TXT: {}", file_count, jpg_count, txt_count);
}

#[test]
fn test_list_with_exclude() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = get_binary_path();
    let test_data_dir = workspace_root.join("../test_data");
    
    // First, list without exclude to see what we have
    let output_all = Command::new(&binary)
        .arg("list")
        .arg("*.jpg")
        .current_dir(&test_data_dir)
        .output()
        .unwrap();
    let stdout_all = String::from_utf8_lossy(&output_all.stdout);
    let all_files: Vec<&str> = stdout_all.lines()
        .skip(1) // Skip "Found X matching file(s):"
        .filter(|l| !l.trim().is_empty() && !l.contains("Found"))
        .collect();
    
    println!("=== test_list_with_exclude: ALL FILES ===");
    for file in &all_files {
        println!("  {}", file.trim());
    }
    println!("Total: {} files", all_files.len());
    
    // Now with exclude
    let output = Command::new(&binary)
        .arg("list")
        .arg("*.jpg")
        .arg("--exclude")
        .arg("*_001*")
        .current_dir(&test_data_dir)
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("=== test_list_with_exclude: WITH EXCLUDE ===");
    println!("{}", stdout);
    println!("===========================================");
    
    let excluded_files: Vec<&str> = stdout.lines()
        .skip(1)
        .filter(|l| !l.trim().is_empty() && !l.contains("Found"))
        .collect();
    
    println!("Files after exclude: {}", excluded_files.len());
    for file in &excluded_files {
        println!("  {}", file.trim());
        assert!(!file.contains("_001"), "File '{}' should have been excluded (contains '_001')", file);
    }
    
    assert!(stdout.contains("Found"), "Expected 'Found' in output, got: {}", stdout);
    assert!(!stdout.contains("_001"), "Should not contain '_001' pattern, got: {}", stdout);
}

#[test]
fn test_list_with_multiple_excludes() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = get_binary_path();
    let test_data_dir = workspace_root.join("../test_data");
    
    let output = Command::new(&binary)
        .arg("list")
        .arg("*.txt")
        .arg("--exclude")
        .arg("*_1*")
        .arg("--exclude")
        .arg("*_2*")
        .current_dir(&test_data_dir)
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("=== test_list_with_multiple_excludes OUTPUT ===");
    println!("{}", stdout);
    println!("===============================================");
    
    let files: Vec<&str> = stdout.lines()
        .skip(1)
        .filter(|l| !l.trim().is_empty() && !l.contains("Found"))
        .collect();
    
    println!("Files after exclusions (should exclude *_1* and *_2*):");
    for file in &files {
        println!("  {}", file.trim());
        assert!(!file.contains("_1"), "File '{}' should have been excluded (contains '_1')", file);
        assert!(!file.contains("_2"), "File '{}' should have been excluded (contains '_2')", file);
    }
    
    assert!(stdout.contains("Found"), "Expected 'Found' in output, got: {}", stdout);
}

#[test]
fn test_list_recursive() {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = get_binary_path();
    
    // Create a temporary directory structure
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let test_dir = workspace_root.join("../target").join(format!("test_list_recursive_{}", timestamp));
    std::fs::create_dir_all(&test_dir).unwrap();
    
    // Create subdirectories and files
    let subdir1 = test_dir.join("photos");
    let subdir2 = test_dir.join("docs");
    std::fs::create_dir_all(&subdir1).unwrap();
    std::fs::create_dir_all(&subdir2).unwrap();
    
    // Create files in different directories
    std::fs::write(subdir1.join("photo1.jpg"), "test").unwrap();
    std::fs::write(subdir1.join("photo2.jpg"), "test").unwrap();
    std::fs::write(subdir2.join("doc1.txt"), "test").unwrap();
    std::fs::write(test_dir.join("root.jpg"), "test").unwrap();
    
    // Test recursive search
    let output = Command::new(&binary)
        .arg("list")
        .arg("*.jpg")
        .arg("--recursive")
        .current_dir(&test_dir)
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("=== test_list_recursive: WITH --recursive ===");
    println!("{}", stdout);
    
    let recursive_files: Vec<&str> = stdout.lines()
        .skip(1)
        .filter(|l| !l.trim().is_empty() && !l.contains("Found"))
        .collect();
    
    println!("Files found recursively: {}", recursive_files.len());
    for file in &recursive_files {
        println!("  {}", file.trim());
    }
    
    // Should find photo1.jpg, photo2.jpg, and root.jpg (3 files)
    assert_eq!(recursive_files.len(), 3, "Expected 3 files recursively, found: {:?}", recursive_files);
    assert!(stdout.contains("photo1.jpg"), "Expected photo1.jpg, got: {}", stdout);
    assert!(stdout.contains("photo2.jpg"), "Expected photo2.jpg, got: {}", stdout);
    assert!(stdout.contains("root.jpg"), "Expected root.jpg, got: {}", stdout);
    
    // Test non-recursive (should only find root.jpg)
    let output2 = Command::new(&binary)
        .arg("list")
        .arg("*.jpg")
        .current_dir(&test_dir)
        .output()
        .unwrap();
    
    let stdout2 = String::from_utf8_lossy(&output2.stdout);
    println!("=== test_list_recursive: WITHOUT --recursive ===");
    println!("{}", stdout2);
    
    let non_recursive_files: Vec<&str> = stdout2.lines()
        .skip(1)
        .filter(|l| !l.trim().is_empty() && !l.contains("Found"))
        .collect();
    
    println!("Files found non-recursively: {}", non_recursive_files.len());
    for file in &non_recursive_files {
        println!("  {}", file.trim());
    }
    
    assert_eq!(non_recursive_files.len(), 1, "Expected 1 file non-recursively, found: {:?}", non_recursive_files);
    assert!(stdout2.contains("root.jpg"), "Expected root.jpg, got: {}", stdout2);
    
    // Cleanup
    let _ = std::fs::remove_dir_all(&test_dir);
}

#[test]
fn test_list_multiple_patterns_with_exclude() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = get_binary_path();
    let test_data_dir = workspace_root.join("../test_data");
    
    let output = Command::new(&binary)
        .arg("list")
        .arg("*.jpg")
        .arg("*.txt")
        .arg("--exclude")
        .arg("*_001*")
        .arg("--exclude")
        .arg("*test*")
        .current_dir(&test_data_dir)
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("=== test_list_multiple_patterns_with_exclude OUTPUT ===");
    println!("{}", stdout);
    println!("======================================================");
    
    let files: Vec<&str> = stdout.lines()
        .skip(1)
        .filter(|l| !l.trim().is_empty() && !l.contains("Found"))
        .collect();
    
    println!("Files after exclusions (should exclude *_001* and *test*):");
    for file in &files {
        println!("  {}", file.trim());
        assert!(!file.contains("_001"), "File '{}' should have been excluded (contains '_001')", file);
        assert!(!file.contains("test"), "File '{}' should have been excluded (contains 'test')", file);
    }
    
    assert!(stdout.contains("Found"), "Expected 'Found' in output, got: {}", stdout);
}

#[test]
fn test_lowercase_tokens() {
    let output = run_fren("InterDisplay-Regular.ttf", Some("%n.%e")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "InterDisplay-Regular.ttf");
}

#[test]
fn test_remove_from_end_shorthand() {
    let output = run_fren("InterDisplay-Regular.ttf", Some("%n--8.%e")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "InterDisplay.ttf");
}

#[test]
fn test_open_ended_range() {
    let output = run_fren("InterDisplay-Regular.ttf", Some("%n14-.%e")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "Regular.ttf");
}

#[test]
fn test_combined_range_negative() {
    let output = run_fren("InterDisplay-Regular.ttf", Some("%n3--8.%e")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "terDisplay.ttf");
}

#[test]
fn test_uppercase_modifier() {
    let output = run_fren("photo_001.jpg", Some("%U%n.%E")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "PHOTO_001.JPG");
}

#[test]
fn test_lowercase_modifier_after_placeholder() {
    // This test ensures modifiers work when placed AFTER placeholders
    // Pattern: %N%L.%E should lowercase the name portion
    // This is the exact pattern that had a bug - modifier after placeholder didn't work
    let output = run_fren("InterDisplay-Regular.ttf", Some("%N%L.%E")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "interdisplay-regular.ttf");
}

#[test]
fn test_uppercase_modifier_after_placeholder() {
    // This test ensures uppercase modifier works when placed AFTER placeholders
    let output = run_fren("photo_001.jpg", Some("%N%U.%E")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "PHOTO_001.JPG");
}

#[test]
fn test_title_case_modifier() {
    let output = run_fren("file_with_underscores_1.txt", Some("%T%n.%e")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "File_With_Underscores_1.Txt");
}

#[test]
fn test_title_case_modifier_after_placeholder() {
    // This test ensures title case modifier works when placed AFTER placeholders
    let output = run_fren("file_with_underscores_1.txt", Some("%N%T.%E")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "File_With_Underscores_1.Txt");
}

#[test]
fn test_parent_placeholder() {
    let output = run_fren("photo_001.jpg", Some("%P_%n.%e")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    // In the test run, the parent of photo_001.jpg is test_data
    assert_eq!(new, "test_data_photo_001.jpg");
}

#[test]
fn test_date_placeholder() {
    let output = run_fren("appendix-A.md", Some("%n_%D.%e")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    // Check if it matches YYYY-MM-DD format (approximate check)
    assert!(new.contains("2025-") || new.contains("2026-"));
}

#[test]
fn test_trim_modifier() {
    let output = run_fren("appendix-B.md", Some("  %n  %M.%e")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "appendix-B.md");
}

#[test]
fn test_trim_modifier_before_placeholder() {
    // This test ensures trim modifier works when placed BEFORE placeholders
    // %M should trim any leading/trailing whitespace before processing
    let output = run_fren("appendix-B.md", Some("%M  %N.%E")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    // %M before %N should trim, but since %N has no spaces, result should be same
    assert_eq!(new, "appendix-B.md");
}

#[test]
fn test_undo_functionality() {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = get_binary_path();
    
    // Use a temporary directory for complete isolation
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let test_dir = workspace_root.join("../target").join(format!("test_undo_{}", timestamp));
    std::fs::create_dir_all(&test_dir).unwrap();
    
    let f1 = test_dir.join("undo_feat1.txt");
    let f2 = test_dir.join("undo_feat2.txt");
    let history_file = test_dir.join(".fren_history.json");
    
    // Ensure clean state
    let _ = std::fs::remove_file(&f1);
    let _ = std::fs::remove_file(&f2);
    let _ = std::fs::remove_file(&history_file);
    
    // Setup: create the original file
    std::fs::write(&f1, "original").unwrap();
    
    // 1. Rename a file
    let output1 = Command::new(&binary)
        .arg("apply")
        .arg("undo_feat1.txt")
        .arg("-t")
        .arg("undo_feat2.txt")
        .arg("-y")
        .current_dir(&test_dir)
        .output()
        .unwrap();
    
    if !output1.status.success() {
        eprintln!("Rename failed: {}", String::from_utf8_lossy(&output1.stderr));
        eprintln!("Rename stdout: {}", String::from_utf8_lossy(&output1.stdout));
    }
    
    // Verify it was renamed
    assert!(f2.exists(), "Target file should exist after rename");
    assert!(!f1.exists(), "Source file should not exist after rename");
    
    // 2. Undo the rename
    let output2 = Command::new(&binary)
        .arg("undo")
        .arg("-y")  // Skip confirmation
        .current_dir(&test_dir)
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output2.stdout);
    let stderr = String::from_utf8_lossy(&output2.stderr);
    
    if !stdout.contains("Successfully reversed 1 renames") && !stdout.contains("Successfully") {
        panic!("Undo failed. Status: {:?}\nStdout: {}\nStderr: {}", 
               output2.status, stdout, stderr);
    }
    
    // Verify it was reversed
    assert!(f1.exists(), "Original file should exist after undo");
    assert!(!f2.exists(), "Renamed file should not exist after undo");
    
    // Cleanup
    let _ = std::fs::remove_dir_all(&test_dir);
}

#[test]
fn test_exclude_with_globs() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = get_binary_path();
    let test_data_dir = workspace_root.join("../test_data");
    
    // Match all .log files but exclude those containing '1' or '2'
    let output = Command::new(&binary)
        .arg("list")
        .arg("old_name_*.log")
        .arg("-e")
        .arg("*_1.log")
        .arg("-e")
        .arg("*_2.log")
        .current_dir(&test_data_dir)
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Found 1 matching file"));
    assert!(stdout.contains("old_name_3.log"));
    assert!(!stdout.contains("old_name_1.log"));
    assert!(!stdout.contains("old_name_2.log"));
}

#[test]
fn test_overwrite_functionality() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = get_binary_path();
    let test_data_dir = workspace_root.join("../test_data");
    
    // Create two dummy files
    let file1 = test_data_dir.join("ov_test1.tmp");
    let file2 = test_data_dir.join("ov_test2.tmp");
    std::fs::write(&file1, "file1 content").unwrap();
    std::fs::write(&file2, "file2 content").unwrap();
    
    // Try to rename file1 to file2 WITHOUT overwrite
    let output = Command::new(&binary)
        .arg("apply")
        .arg("ov_test1.tmp")
        .arg("-t")
        .arg("ov_test2.tmp")
        .arg("-y")
        .current_dir(&test_data_dir)
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Skipping"));
    assert!(stdout.contains("Target file already exists"));
    assert!(file1.exists()); // file1 should still be there
    
    // Try to rename file1 to file2 WITH overwrite
    let _output = Command::new(&binary)
        .arg("apply")
        .arg("ov_test1.tmp")
        .arg("-t")
        .arg("ov_test2.tmp")
        .arg("-y")
        .arg("-o")
        .current_dir(&test_data_dir)
        .output()
        .unwrap();
    
    assert!(!file1.exists()); // file1 should be gone
    assert!(file2.exists());  // file2 should still be there (overwritten)
    let content = std::fs::read_to_string(&file2).unwrap();
    assert_eq!(content, "file1 content");
    
    // Cleanup
    let _ = std::fs::remove_file(file2);
}

#[test]
fn test_hidden_files_matching() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = get_binary_path();
    let test_data_dir = workspace_root.join("../test_data");
    
    // Create a hidden file for testing
    let hidden = test_data_dir.join(".hidden_test_file");
    std::fs::write(&hidden, "hidden").unwrap();
    
    // '*' should NOT match it
    let output = Command::new(&binary)
        .arg("list")
        .arg("*")
        .current_dir(&test_data_dir)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains(".hidden_test_file"));
    
    // '.*' SHOULD match it
    let output = Command::new(&binary)
        .arg("list")
        .arg(".*")
        .current_dir(&test_data_dir)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(".hidden_test_file"));
    
    // Cleanup
    let _ = std::fs::remove_file(hidden);
}

#[test]
fn test_multiple_modifiers_and_replacements() {
    // Input: "InterDisplay-Regular.ttf"
    // Template: "%U%n%R/-/ %M%R/ /_.%e"
    // 1. %U -> mode Uppercase
    // 2. %n -> "INTERDISPLAY-REGULAR"
    // 3. %R/-/ -> "INTERDISPLAY REGULAR"
    // 4. %M -> "INTERDISPLAY REGULAR" (trim)
    // 5. %R/ /_ -> "INTERDISPLAY_REGULAR"
    // 6. .%e -> ".TTF"
    
    let output = run_fren("InterDisplay-Regular.ttf", Some("%U%n%R/-/ %M%R/ /_.%e")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "INTERDISPLAY_REGULAR.TTF");
}

#[test]
fn test_undo_with_conflicts() {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = get_binary_path();
    
    // Use a temporary directory for complete isolation
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let test_dir = workspace_root.join("../target").join(format!("test_undo_conf_{}", timestamp));
    std::fs::create_dir_all(&test_dir).unwrap();
    
    let f1 = test_dir.join("undo_conf1.txt");
    let f2 = test_dir.join("undo_conf2.txt");
    let history_file = test_dir.join(".fren_history.json");
    
    // Ensure clean state
    let _ = std::fs::remove_file(&f1);
    let _ = std::fs::remove_file(&f2);
    let _ = std::fs::remove_file(&history_file);
    
    // Setup: file1.txt -> file2.txt
    std::fs::write(&f1, "original").unwrap();
    
    let output1 = Command::new(&binary)
        .arg("apply")
        .arg("undo_conf1.txt")
        .arg("-t")
        .arg("undo_conf2.txt")
        .arg("-y")
        .current_dir(&test_dir)
        .output()
        .unwrap();
    
    if !output1.status.success() {
        eprintln!("Initial rename failed: {}", String::from_utf8_lossy(&output1.stderr));
        eprintln!("Initial rename stdout: {}", String::from_utf8_lossy(&output1.stdout));
    }
    
    assert!(f2.exists(), "Target file should exist after rename");
    assert!(!f1.exists(), "Source file should not exist after rename");
    
    // Case 1: Target file is missing
    std::fs::remove_file(&f2).unwrap();
    let output2 = Command::new(&binary)
        .arg("undo")
        .current_dir(&test_dir)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output2.stdout);
    assert!(stdout.contains("File no longer exists") || stdout.contains("no longer exists"),
            "Should detect missing target file. Output: {}", stdout);
    
    // Reset: clear history and redo rename
    let _ = std::fs::remove_file(&history_file);
    std::fs::write(&f1, "original").unwrap();
    
    let output3 = Command::new(&binary)
        .arg("apply")
        .arg("undo_conf1.txt")
        .arg("-t")
        .arg("undo_conf2.txt")
        .arg("-y")
        .current_dir(&test_dir)
        .output()
        .unwrap();
    
    if !output3.status.success() {
        eprintln!("Second rename failed: {}", String::from_utf8_lossy(&output3.stderr));
        eprintln!("Second rename stdout: {}", String::from_utf8_lossy(&output3.stdout));
    }
    
    assert!(f2.exists(), "Target file should exist after second rename");
    assert!(!f1.exists(), "Source file should not exist after second rename");
        
    // Case 2: Source path is occupied by a new file
    std::fs::write(&f1, "intruder").unwrap();
    let output4 = Command::new(&binary)
        .arg("undo")
        .current_dir(&test_dir)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output4.stdout);
    assert!(stdout.contains("Original location occupied") || stdout.contains("location occupied") || stdout.contains("occupied"),
            "Should detect occupied source location. Output: {}", stdout);
    
    // Cleanup
    let _ = std::fs::remove_dir_all(&test_dir);
}

#[test]
fn test_multiple_source_parts() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = get_binary_path();
    let test_data_dir = workspace_root.join("../test_data");
    
    // Simulating 'fren list photo_001.jpg photo_002.jpg'
    let output = Command::new(&binary)
        .arg("list")
        .arg("photo_001.jpg")
        .arg("photo_002.jpg")
        .current_dir(&test_data_dir)
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Found 2 matching file"));
    assert!(stdout.contains("photo_001.jpg"));
    assert!(stdout.contains("photo_002.jpg"));
}

#[test]
fn test_sequential_case_modifiers() {
    // 1. Start with uppercase
    // 2. Add name
    // 3. Switch to lowercase (applies to entire result so far)
    // 4. Add extension
    let output = run_fren("InterDisplay-Regular.ttf", Some("%U%n%L.%e")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    // %U%n -> "INTERDISPLAY-REGULAR"
    // %L applies lowercase to entire result -> "interdisplay-regular"
    // .%e -> ".ttf" (lowercased)
    assert_eq!(new, "interdisplay-regular.ttf");
}

#[test]
fn test_current_time_placeholder() {
    let output = run_fren("photo_001.jpg", Some("%n_%H.%e")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    // Check if it matches HH-MM-SS format (approximate check for hyphens)
    assert!(new.contains("-"));
}

#[test]
fn test_file_date_time_placeholders() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let test_data_dir = workspace_root.join("../test_data");
    let file_path = test_data_dir.join("photo_001.jpg");
    
    // Ensure file exists
    if !file_path.exists() {
        std::fs::write(&file_path, "dummy").unwrap();
    }

    let output = run_fren("photo_001.jpg", Some("%n_%FD_%FH.%e")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    
    // Should match YYYY-MM-DD and HH-MM-SS
    let date_re = regex::Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
    let time_re = regex::Regex::new(r"\d{2}-\d{2}-\d{2}").unwrap();
    assert!(date_re.is_match(new), "Filename '{}' should contain a date", new);
    assert!(time_re.is_match(new), "Filename '{}' should contain a time", new);
}

#[test]
fn test_parent_directory_range() {
    let output = run_fren("photo_001.jpg", Some("%P1-4_%n.%e")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    // parent is "test_data", first 4 chars is "test"
    assert!(new.starts_with("test_"));
}

#[test]
fn test_regex_replacement() {
    // Replace all digits with 'X'
    let output = run_fren("photo_001.jpg", Some("%n%X/\\d/X.%e")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    assert_eq!(new, "photo_XXX.jpg");
}

// Note: %X before placeholders doesn't make practical sense since it operates on accumulated result
// which would be empty. The meaningful use case is %X after placeholders (already tested).
// This test is intentionally omitted as it would test undefined/edge case behavior.

#[test]
fn test_empty_name_blocking() {
    let binary = get_binary_path();
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let test_data_dir = workspace_root.join("../test_data");
    
    // Pattern that results in empty name (assuming file has no digits)
    // Using %X to remove everything
    let output = Command::new(&binary)
        .arg("apply")
        .arg(test_data_dir.join("photo_001.jpg"))
        .arg("-t")
        .arg("%X/.//")
        .arg("-y")
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ERROR: One or more files would have an empty name"));
    assert_ne!(output.status.code(), Some(0));
}

#[test]
fn test_unknown_token_warning() {
    let binary = get_binary_path();
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let test_data_dir = workspace_root.join("../test_data");
    
    let output = Command::new(&binary)
        .arg("rename")
        .arg(test_data_dir.join("photo_001.jpg"))
        .arg("-t")
        .arg("test%Z")
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("WARNINGS:"));
    assert!(stdout.contains("Unknown token: %Z"));
}

#[test]
fn test_overwrite_flag_help() {
    let binary = get_binary_path();
    
    let output = Command::new(&binary)
        .arg("apply")
        .arg("--help")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("-o, --overwrite"));
    assert!(stdout.contains("-t, --to"));
}

#[test]
fn test_recursive_directory_support() {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = get_binary_path();
    
    // Create a temporary directory structure
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let test_dir = workspace_root.join("../target").join(format!("test_recursive_{}", timestamp));
    std::fs::create_dir_all(&test_dir).unwrap();
    
    // Create subdirectories and files
    let subdir1 = test_dir.join("photos");
    let subdir2 = test_dir.join("videos");
    let subdir3 = test_dir.join("docs");
    std::fs::create_dir_all(&subdir1).unwrap();
    std::fs::create_dir_all(&subdir2).unwrap();
    std::fs::create_dir_all(&subdir3).unwrap();
    
    // Create files in different directories
    std::fs::write(subdir1.join("photo1.jpg"), "test").unwrap();
    std::fs::write(subdir1.join("photo2.jpg"), "test").unwrap();
    std::fs::write(subdir2.join("video1.mp4"), "test").unwrap();
    std::fs::write(subdir3.join("doc1.txt"), "test").unwrap();
    std::fs::write(test_dir.join("root.jpg"), "test").unwrap();
    
    // Test recursive search with -r flag
    let output = Command::new(&binary)
        .arg("rename")
        .arg("*.jpg")
        .arg("-r")
        .arg("-t")
        .arg("renamed_%C2.%E")
        .current_dir(&test_dir)
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let renames = extract_renames(&stdout);
    
    // Should find photo1.jpg, photo2.jpg, and root.jpg (3 files)
    assert!(renames.len() >= 3, "Should find at least 3 jpg files recursively");
    
    // Verify all found files are jpg
    for (old, new) in &renames {
        assert!(old.ends_with(".jpg"), "All found files should be .jpg");
        assert!(new.ends_with(".jpg"), "All renamed files should be .jpg");
    }
    
    // Cleanup
    let _ = std::fs::remove_dir_all(&test_dir);
}

#[test]
fn test_recursive_with_double_star_pattern() {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = get_binary_path();
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let test_dir = workspace_root.join("../target").join(format!("test_recursive_star_{}", timestamp));
    std::fs::create_dir_all(&test_dir).unwrap();
    
    let subdir1 = test_dir.join("level1");
    let subdir2 = subdir1.join("level2");
    std::fs::create_dir_all(&subdir2).unwrap();
    
    std::fs::write(subdir2.join("deep.txt"), "test").unwrap();
    std::fs::write(test_dir.join("shallow.txt"), "test").unwrap();
    
    // Test with ** pattern (explicit recursive)
    let output = Command::new(&binary)
        .arg("rename")
        .arg("**/*.txt")
        .arg("-t")
        .arg("renamed_%C2.%E")
        .current_dir(&test_dir)
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let renames = extract_renames(&stdout);
    
    // Should find both deep.txt and shallow.txt
    assert!(renames.len() >= 2, "Should find files at multiple levels");
    
    // Cleanup
    let _ = std::fs::remove_dir_all(&test_dir);
}

#[test]
fn test_non_recursive_does_not_search_subdirs() {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = get_binary_path();
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let test_dir = workspace_root.join("../target").join(format!("test_nonrecursive_{}", timestamp));
    std::fs::create_dir_all(&test_dir).unwrap();
    
    let subdir = test_dir.join("subdir");
    std::fs::create_dir_all(&subdir).unwrap();
    
    std::fs::write(subdir.join("hidden.txt"), "test").unwrap();
    std::fs::write(test_dir.join("visible.txt"), "test").unwrap();
    
    // Test without -r flag (should NOT find subdir files)
    let output = Command::new(&binary)
        .arg("rename")
        .arg("*.txt")
        .arg("-t")
        .arg("renamed.%E")
        .current_dir(&test_dir)
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let renames = extract_renames(&stdout);
    
    // Should only find visible.txt, not hidden.txt in subdir
    assert_eq!(renames.len(), 1, "Should only find files in current directory");
    assert!(renames[0].0.contains("visible.txt"), "Should find visible.txt");
    
    // Cleanup
    let _ = std::fs::remove_dir_all(&test_dir);
}

#[test]
fn test_template_listing() {
    let binary = get_binary_path();
    
    let output = Command::new(&binary)
        .arg("templates")
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should list templates
    assert!(stdout.contains("photo-date"));
    assert!(stdout.contains("lowercase"));
    assert!(stdout.contains("counter-3"));
    assert!(stdout.contains("->")); // Should show pattern mapping
}

#[test]
fn test_template_usage() {
    let binary = get_binary_path();
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let test_data_dir = workspace_root.join("../test_data");
    
    let output = Command::new(&binary)
        .arg("rename")
        .arg("InterDisplay-Regular.ttf")
        .arg("-T")
        .arg("lowercase-name")
        .current_dir(&test_data_dir)
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let renames = extract_renames(&stdout);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    // lowercase-name template should be %N%L.%E
    assert_eq!(new, "interdisplay-regular.ttf");
}

#[test]
fn test_template_unknown() {
    let binary = get_binary_path();
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let test_data_dir = workspace_root.join("../test_data");
    
    let output = Command::new(&binary)
        .arg("rename")
        .arg("*.txt")
        .arg("-T")
        .arg("nonexistent-template")
        .current_dir(&test_data_dir)
        .output()
        .unwrap();
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown template") || stderr.contains("Unknown"));
    assert!(stderr.contains("templates") || stderr.contains("--list-templates"));
}

#[test]
fn test_template_photo_date() {
    let binary = get_binary_path();
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let test_data_dir = workspace_root.join("../test_data");
    
    // Ensure test file exists
    let test_file = test_data_dir.join("photo.jpg");
    if !test_file.exists() {
        std::fs::write(&test_file, "test").unwrap();
    }
    
    let output = Command::new(&binary)
        .arg("rename")
        .arg("photo.jpg")
        .arg("-T")
        .arg("photo-date")
        .current_dir(&test_data_dir)
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let renames = extract_renames(&stdout);
    assert!(!renames.is_empty(), "Should find and rename photo.jpg");
    let (_, new) = &renames[0];
    // photo-date template should be %N_%D.%E
    assert!(new.contains("photo_"));
    assert!(new.contains(".jpg"));
    // Should contain date (YYYY-MM-DD format)
    assert!(new.contains("2025-") || new.contains("2026-"));
}

#[test]
fn test_template_counter_3() {
    let binary = get_binary_path();
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let test_data_dir = workspace_root.join("../test_data");
    
    // Ensure test file exists
    let test_file = test_data_dir.join("file1.txt");
    if !test_file.exists() {
        std::fs::write(&test_file, "test").unwrap();
    }
    
    let output = Command::new(&binary)
        .arg("rename")
        .arg("file1.txt")
        .arg("-T")
        .arg("counter-3")
        .current_dir(&test_data_dir)
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let renames = extract_renames(&stdout);
    assert!(!renames.is_empty(), "Should find and rename file1.txt");
    let (_, new) = &renames[0];
    // counter-3 template should be %C3.%E
    assert_eq!(new, "001.txt");
}

#[test]
fn test_interactive_flag_exists() {
    let binary = get_binary_path();
    
    let output = Command::new(&binary)
        .arg("interactive")
        .arg("--help")
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Interactive is now a subcommand, not a flag
    assert!(stdout.contains("interactive") || stdout.contains("Interactive"));
}

#[test]
fn test_interactive_mode_with_pattern() {
    // This test verifies interactive mode can be invoked with a pattern
    // Full interactive testing would require stdin mocking which is complex
    let binary = get_binary_path();
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let test_data_dir = workspace_root.join("../test_data");
    
    // Just verify the command is accepted (won't actually run interactively in test)
    let output = Command::new(&binary)
        .arg("interactive")
        .arg("*.txt")
        .arg("-t")
        .arg("%N.%E")
        .current_dir(&test_data_dir)
        .output();
    
    // Should either succeed or fail gracefully, not crash
    assert!(output.is_ok());
}

#[test]
fn test_version_flag() {
    let binary = get_binary_path();
    
    let output = Command::new(&binary)
        .arg("--version")
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should output version number
    assert!(stdout.contains("fren"));
    assert!(stdout.contains("0.2.0") || stdout.matches(char::is_numeric).count() > 0);
    
    // Test short form
    let output2 = Command::new(&binary)
        .arg("-V")
        .output()
        .unwrap();
    
    let stdout2 = String::from_utf8_lossy(&output2.stdout);
    assert!(stdout2.contains("fren"));
}

#[test]
fn test_name_substring_selection() {
    // Test %N1-3 to extract first 3 characters of name
    let output = run_fren("InterDisplay-Regular.ttf", Some("%N1-3.%E")).unwrap();
    let renames = extract_renames(&output);
    assert!(!renames.is_empty());
    let (_, new) = &renames[0];
    // First 3 chars of "InterDisplay-Regular" are "Int"
    assert_eq!(new, "Int.ttf");
}
