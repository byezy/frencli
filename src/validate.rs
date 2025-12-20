//! Validate subcommand for checking rename operations before execution.
//! 
//! This module handles the `fren validate` command which performs comprehensive
//! validation on proposed rename operations. It checks filename validity,
//! file system permissions, circular renames, and more.

use freneng::{RenamingEngine, EnginePreviewResult, ValidationIssue, ValidationResult};
use std::path::PathBuf;
use std::collections::HashMap;

/// Handles the validate subcommand - performs comprehensive validation on a preview.
/// 
/// # Arguments
/// 
/// * `engine` - The renaming engine
/// * `preview_result` - The preview result from transform/template --use command
/// * `overwrite` - Whether to check validation with overwrite enabled
/// * `skip_invalid` - If true, continue even if issues found (don't abort)
/// 
/// # Returns
/// 
/// * `Ok(())` - Validation completed (may have issues if skip_invalid=true)
/// * Exits with code 1 if validation fails and skip_invalid=false
pub async fn handle_validate_command(
    engine: &RenamingEngine,
    preview_result: &EnginePreviewResult,
    overwrite: bool,
    skip_invalid: bool,
) {
    // First check preview-level issues (empty names, warnings)
    if preview_result.has_empty_names {
        let empty_count = preview_result.renames.iter()
            .filter(|r| r.new_name.trim().is_empty())
            .count();
        
        if skip_invalid {
            println!("⚠ WARNING: {} file(s) would have empty names (skipped)", empty_count);
        } else {
            eprintln!("❌ ERROR: Pattern would generate {} empty filename(s).", empty_count);
            eprintln!("Please check your pattern and ensure it generates valid filenames.");
            std::process::exit(1);
        }
    }
    
    if !preview_result.warnings.is_empty() {
        println!("\n⚠ Pattern Warnings:");
        for warning in &preview_result.warnings {
            println!("  - {}", warning);
        }
        if !skip_invalid {
            println!("\nUse --skip-invalid to continue despite warnings.");
        }
    }
    
    // Run comprehensive validation
    let validation_result = engine.validate(&preview_result.renames, overwrite).await;
    
    // Display validation results
    display_validation_results(&validation_result, overwrite);
    
    // Summary
    let total = preview_result.renames.len();
    let valid_count = validation_result.valid.len();
    let issue_count = validation_result.issues.len();
    
    println!("\n📊 Validation Summary:");
    println!("  Total files: {}", total);
    println!("  ✓ Valid: {}", valid_count);
    println!("  ✗ Issues: {}", issue_count);
    
    // Exit with error if issues found and not skipping
    if !validation_result.issues.is_empty() && !skip_invalid {
        eprintln!("\n❌ Validation failed. Use --skip-invalid to continue despite issues.");
        std::process::exit(1);
    }
    
    if validation_result.issues.is_empty() && !preview_result.has_empty_names {
        println!("\n✓ All files passed validation!");
    }
}

/// Displays validation results in a clear, organized format.
fn display_validation_results(result: &ValidationResult, overwrite: bool) {
    if result.valid.is_empty() && result.issues.is_empty() {
        println!("\nNo files to validate.");
        return;
    }
    
    // Group issues by type for better readability
    let mut issues_by_type: HashMap<String, Vec<(PathBuf, ValidationIssue)>> = HashMap::new();
    
    for (path, issue) in &result.issues {
        let issue_type = match issue {
            ValidationIssue::InvalidCharacters(_) => "Invalid Characters",
            ValidationIssue::ReservedFilename(_) => "Reserved Filename",
            ValidationIssue::PathTooLong { .. } => "Path Too Long",
            ValidationIssue::SourceNotFound(_) => "Source Not Found",
            ValidationIssue::SourceNotReadable(_) => "Source Not Readable",
            ValidationIssue::ParentNotWritable(_) => "Parent Not Writable",
            ValidationIssue::TargetExists(_) => "Target Exists",
            ValidationIssue::CircularRename { .. } => "Circular Rename",
            ValidationIssue::InvalidFormat(_) => "Invalid Format",
            ValidationIssue::EmptyFilename => "Empty Filename",
        }.to_string();
        
        issues_by_type
            .entry(issue_type)
            .or_insert_with(Vec::new)
            .push((path.clone(), issue.clone()));
    }
    
    // Display valid renames
    if !result.valid.is_empty() {
        println!("\n✓ Valid Renames ({}):", result.valid.len());
        for rename in &result.valid {
            println!("  {} → {}", 
                rename.old_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("?"),
                rename.new_name);
        }
    }
    
    // Display issues grouped by type
    if !result.issues.is_empty() {
        println!("\n✗ Validation Issues ({}):", result.issues.len());
        
        for (issue_type, issues) in issues_by_type.iter() {
            println!("\n  {} ({} file(s)):", issue_type, issues.len());
            
            for (path, issue) in issues {
                let file_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("?");
                
                let details = match issue {
                    ValidationIssue::InvalidCharacters(msg) => format!("{}", msg),
                    ValidationIssue::ReservedFilename(msg) => format!("{}", msg),
                    ValidationIssue::PathTooLong { path, max_length } => {
                        format!("Path length {} exceeds maximum {} characters", path.len(), max_length)
                    },
                    ValidationIssue::SourceNotFound(_) => "Source file does not exist".to_string(),
                    ValidationIssue::SourceNotReadable(_) => "Source file is not readable".to_string(),
                    ValidationIssue::ParentNotWritable(_) => "Parent directory is not writable".to_string(),
                    ValidationIssue::TargetExists(_) => {
                        if overwrite {
                            "Target exists (will be overwritten)".to_string()
                        } else {
                            "Target file already exists".to_string()
                        }
                    },
                    ValidationIssue::CircularRename { file1, file2 } => {
                        format!("Circular dependency: {} ↔ {}", file1, file2)
                    },
                    ValidationIssue::InvalidFormat(msg) => format!("{}", msg),
                    ValidationIssue::EmptyFilename => "Generated filename is empty".to_string(),
                };
                
                println!("    {}: {}", file_name, details);
            }
        }
    }
}

