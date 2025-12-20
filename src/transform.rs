//! Transform subcommand for generating rename pattern previews.
//! 
//! This module handles the `fren transform` command which applies a rename pattern
//! (template) to matching files and generates a preview. All operations are async to match the async API of freneng.

use freneng::{RenamingEngine, FrenError, EnginePreviewResult};
use crate::ui::display_preview;
use std::path::PathBuf;

/// Handles the transform subcommand - generates and displays preview.
/// 
/// # Arguments
/// 
/// * `engine` - The renaming engine
/// * `files` - List of files to transform
/// * `template` - The rename pattern/template (e.g., "%N.%E")
/// 
/// # Returns
/// 
/// * `Ok(EnginePreviewResult)` - Preview result that can be used by rename command
/// * `Err(FrenError)` - If preview generation fails
pub async fn handle_transform_command(
    engine: &RenamingEngine,
    files: Vec<PathBuf>,
    template: String,
) -> Result<EnginePreviewResult, FrenError> {
    if files.is_empty() {
        eprintln!("Error: No files to transform.");
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
        eprintln!("\nERROR: One or more files would have an empty name. Transformation aborted.");
        eprintln!("Please check your pattern and ensure it generates valid filenames.");
        std::process::exit(1);
    }

    // transform command only shows preview - use 'rename' to actually rename
    println!("\nPreview mode. Use 'rename' subcommand to perform the renaming.");
    
    Ok(preview_result)
}

