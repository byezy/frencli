//! Tests for the UI module.
//! 
//! These tests verify display and user interaction functions.
//! Note: Some functions require stdin/stdout, so we test what we can.

use frencli::ui::display_preview;
use freneng::FileRename;
use std::path::PathBuf;

#[test]
fn test_display_preview_empty() {
    let renames: Vec<FileRename> = vec![];
    
    // Should not panic
    display_preview(&renames);
}

#[test]
fn test_display_preview_single() {
    let renames = vec![FileRename {
        old_path: PathBuf::from("old.txt"),
        new_path: PathBuf::from("new.txt"),
        new_name: "new.txt".to_string(),
    }];
    
    // Should not panic
    display_preview(&renames);
}

#[test]
fn test_display_preview_multiple() {
    let renames = vec![
        FileRename {
            old_path: PathBuf::from("file1.txt"),
            new_path: PathBuf::from("renamed1.txt"),
            new_name: "renamed1.txt".to_string(),
        },
        FileRename {
            old_path: PathBuf::from("file2.jpg"),
            new_path: PathBuf::from("renamed2.jpg"),
            new_name: "renamed2.jpg".to_string(),
        },
    ];
    
    // Should not panic
    display_preview(&renames);
}

#[test]
fn test_display_preview_with_empty_name() {
    let renames = vec![
        FileRename {
            old_path: PathBuf::from("file.txt"),
            new_path: PathBuf::from(""),
            new_name: "".to_string(),
        },
    ];
    
    // Should not panic and should show error indicator
    display_preview(&renames);
}

#[test]
fn test_display_preview_with_path_without_filename() {
    // Test edge case where file_name() might return None
    let renames = vec![FileRename {
        old_path: PathBuf::from("/"),
        new_path: PathBuf::from("/new"),
        new_name: "new".to_string(),
    }];
    
    // Should not panic (should show "?" for old name)
    display_preview(&renames);
}

#[test]
fn test_display_preview_long_names() {
    let long_name = "a".repeat(100);
    let renames = vec![FileRename {
        old_path: PathBuf::from("short.txt"),
        new_path: PathBuf::from(&long_name),
        new_name: long_name,
    }];
    
    // Should not panic with long names
    display_preview(&renames);
}

