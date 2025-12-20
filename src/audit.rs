//! Audit subcommand for viewing audit logs.
//! 
//! This module handles the `fren audit` command which displays audit log entries
//! from previous rename operations.

use freneng::{read_audit_log, AuditEntry};

/// Handles the audit subcommand - displays audit log entries.
/// 
/// # Arguments
/// 
/// * `limit` - Maximum number of entries to display (None = all)
/// * `json` - If true, output as JSON; if false, output as human-readable table
/// 
/// # Returns
/// 
/// * `Ok(())` - Command completed successfully
/// * `Err(String)` - If audit log reading fails
pub async fn handle_audit_command(limit: Option<usize>, json: bool) -> Result<(), String> {
    let entries = read_audit_log().await.map_err(|e| format!("Failed to read audit log: {}", e))?;
    
    if entries.is_empty() {
        println!("No audit entries found.");
        return Ok(());
    }
    
    let display_entries: Vec<&AuditEntry> = if let Some(limit) = limit {
        entries.iter().take(limit).collect()
    } else {
        entries.iter().collect()
    };
    
    if json {
        // Output as JSON array
        let json = serde_json::to_string_pretty(&display_entries)
            .map_err(|e| format!("Failed to serialize audit entries: {}", e))?;
        println!("{}", json);
    } else {
        // Output as human-readable table
        display_audit_entries(&display_entries);
    }
    
    Ok(())
}

/// Displays audit entries in a human-readable format.
fn display_audit_entries(entries: &[&AuditEntry]) {
    println!("Audit Log Entries (showing {} of {}):\n", entries.len(), entries.len());
    println!("{:-<120}", "");
    
    for (i, entry) in entries.iter().enumerate() {
        println!("\nEntry #{}", i + 1);
        println!("  Timestamp:      {}", entry.timestamp.format("%Y-%m-%d %H:%M:%S"));
        if let Some(user) = &entry.user {
            println!("  User:           {}", user);
        }
        println!("  Directory:      {}", entry.working_directory.display());
        println!("  Command:        {}", entry.command);
        if let Some(pattern) = &entry.pattern {
            println!("  Pattern:        {}", pattern);
        }
        println!("  Results:        {} successful, {} skipped, {} errors",
            entry.successful_count, entry.skipped_count, entry.error_count);
        
        if !entry.successful.is_empty() {
            println!("  Successful renames:");
            for (old, new) in &entry.successful {
                println!("    {} -> {}", 
                    old.file_name().and_then(|n| n.to_str()).unwrap_or("?"),
                    new.file_name().and_then(|n| n.to_str()).unwrap_or("?"));
            }
        }
        
        if !entry.skipped.is_empty() {
            println!("  Skipped files:");
            for (path, reason) in &entry.skipped {
                println!("    {}: {}", 
                    path.file_name().and_then(|n| n.to_str()).unwrap_or("?"),
                    reason);
            }
        }
        
        if !entry.errors.is_empty() {
            println!("  Errors:");
            for (path, error) in &entry.errors {
                println!("    {}: {}", 
                    path.file_name().and_then(|n| n.to_str()).unwrap_or("?"),
                    error);
            }
        }
        
        if i < entries.len() - 1 {
            println!("{:-<120}", "");
        }
    }
}

