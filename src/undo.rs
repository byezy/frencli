//! Undo subcommand for reversing previous rename operations.
//! 
//! This module handles the `fren undo` command which can check undo status
//! or apply undo operations to reverse previous renames.

use freneng::RenamingEngine;
use freneng::history::{load_history, clear_history};
use crate::ui::confirm_undo_conflicts;

/// Handles the undo --check subcommand - checks what can be safely undone.
/// 
/// # Arguments
/// 
/// * `engine` - The renaming engine
/// 
/// # Returns
/// 
/// * Exits with code 0 on success, 1 on error
pub async fn handle_undo_check(engine: &RenamingEngine) {
    match load_history().await {
        Ok(Some(history)) => {
            println!("Checking undo state for {} renames from {}...", 
                history.actions.len(), 
                history.timestamp.format("%Y-%m-%d %H:%M:%S"));

            let (safe_actions, conflicts) = engine.check_undo(&history).await;

            if !conflicts.is_empty() {
                println!("\nFound {} conflict(s) that prevent a full undo:", conflicts.len());
                for conflict in &conflicts {
                    println!("  - {}", conflict);
                }
            }

            let safe_count = safe_actions.len();
            if safe_count == 0 {
                println!("\nAll files in this batch have conflicts. Cannot proceed with undo.");
            } else {
                println!("\n{} file(s) can be safely undone.", safe_count);
            }
        }
        Ok(None) => {
            println!("No rename history found in this directory.");
        }
        Err(e) => {
            eprintln!("Error loading history: {}", e);
            std::process::exit(1);
        }
    }
}

/// Handles the undo --apply subcommand - actually performs the undo operation.
/// 
/// # Arguments
/// 
/// * `engine` - The renaming engine
/// * `yes` - Skip confirmation prompt
/// 
/// # Returns
/// 
/// * Exits with code 0 on success, 1 on error
pub async fn handle_undo_apply(engine: &RenamingEngine, yes: bool) {
    match load_history().await {
        Ok(Some(history)) => {
            println!("Checking undo state for {} renames from {}...", 
                history.actions.len(), 
                history.timestamp.format("%Y-%m-%d %H:%M:%S"));

            let (safe_actions, conflicts) = engine.check_undo(&history).await;

            if !conflicts.is_empty() {
                println!("\nFound {} conflict(s) that prevent a full undo:", conflicts.len());
                for conflict in &conflicts {
                    println!("  - {}", conflict);
                }

                let safe_count = safe_actions.len();
                if safe_count == 0 {
                    println!("\nAll files in this batch have conflicts. Cannot proceed with undo.");
                    println!("Undo operation cancelled.");
                    std::process::exit(1);
                }

                if !yes && !confirm_undo_conflicts(safe_count) {
                    println!("Undo operation cancelled.");
                    std::process::exit(0);
                }
            }

            match engine.apply_undo(safe_actions).await {
                Ok(count) => {
                    println!("Successfully reversed {} renames.", count);
                    let _ = clear_history().await;
                }
                Err(e) => {
                    eprintln!("Error during undo: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Ok(None) => {
            println!("No rename history found in this directory.");
        }
        Err(e) => {
            eprintln!("Error loading history: {}", e);
            std::process::exit(1);
        }
    }
}

