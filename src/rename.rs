//! Rename subcommand for generating rename pattern previews.
//! 
//! This module handles the `frencli rename` command which applies a rename pattern
//! (template) to matching files and generates a preview. All operations are async to match the async API of freneng.

use freneng::{RenamingEngine, FrenError, EnginePreviewResult};
use crate::ui::display_preview;
use std::path::PathBuf;
use serde::Serialize;

#[derive(Serialize)]
struct RenameJsonOutput {
    renames: Vec<RenameJsonItem>,
    warnings: Vec<String>,
    has_empty_names: bool,
}

#[derive(Serialize)]
struct RenameJsonItem {
    old_path: String,
    new_path: String,
    new_name: String,
}

/// Handles the rename subcommand - generates and displays preview.
/// 
/// # Arguments
/// 
/// * `engine` - The renaming engine
/// * `files` - List of files to process
/// * `template` - The rename pattern/template (e.g., "%N.%E")
/// * `json` - If true, output as JSON; if false, output as human-readable
/// 
/// # Returns
/// 
/// * `Ok(EnginePreviewResult)` - Preview result that can be used by apply command
/// * `Err(FrenError)` - If preview generation fails
pub async fn handle_rename_command(
    engine: &RenamingEngine,
    files: Vec<PathBuf>,
    template: String,
    json: bool,
) -> Result<EnginePreviewResult, FrenError> {
    if files.is_empty() {
        eprintln!("Error: No files to process.");
        return Err(FrenError::Pattern("No files provided".into()));
    }

    // Generate preview
    let preview_result = match engine.generate_preview(&files, &template).await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error generating rename patterns: {}", e);
            return Err(e);
        }
    };

    if json {
        // Output as JSON
        let json_output = RenameJsonOutput {
            renames: preview_result.renames.iter().map(|r| RenameJsonItem {
                old_path: r.old_path.to_string_lossy().to_string(),
                new_path: r.new_path.to_string_lossy().to_string(),
                new_name: r.new_name.clone(),
            }).collect(),
            warnings: preview_result.warnings.clone(),
            has_empty_names: preview_result.has_empty_names,
        };
        let json_str = serde_json::to_string_pretty(&json_output)
            .map_err(|e| FrenError::Pattern(format!("Failed to serialize JSON: {}", e)))?;
        println!("{}", json_str);
    } else {
        // Display preview
        display_preview(&preview_result.renames);

        // Show warnings
        if !preview_result.warnings.is_empty() {
            println!("\nWARNINGS:");
            for warning in &preview_result.warnings {
                println!("  - {}", warning);
            }
        }

        // Block if empty names
        if preview_result.has_empty_names {
            eprintln!("\nERROR: One or more files would have an empty name. Operation aborted.");
            eprintln!("Please check your pattern and ensure it generates valid filenames.");
            std::process::exit(1);
        }

        // rename command only shows preview - use 'apply' to actually rename
        println!("\nPreview mode. Use 'apply' subcommand to perform the renaming.");
    }
    
    Ok(preview_result)
}

