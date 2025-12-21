use std::io::{self, Write};
use freneng::FileRename;

pub fn display_preview(renames: &[FileRename]) {
    println!("{:<40} -> {:<40}", "Old Name", "New Name");
    println!("{:-<40}----{:-<40}", "", "");
    
    for rename in renames {
        let old = rename.old_path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
        let new = &rename.new_name;
        
        if new.trim().is_empty() {
            println!("{:<40} -> {:<40}", old, "[ERROR: EMPTY NAME]");
        } else {
            println!("{:<40} -> {:<40}", old, new);
        }
    }
}

pub fn confirm_undo_conflicts(safe_count: usize) -> bool {
    print!("\nProceed with undoing {} safe renames? (y/N): ", safe_count);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase() == "y"
}

pub fn interactive_edit(renames: &mut [FileRename]) -> bool {
    println!("\nInteractive mode: Edit filenames (press Enter to keep, type new name to change)");
    println!("Commands: 'q' to quit, 's' to skip file, 'a' to apply all remaining");
    println!("{:-<80}", "");
    
    let mut apply_all = false;
    
    for (i, rename) in renames.iter_mut().enumerate() {
        let old = rename.old_path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
        let current_new = &rename.new_name;
        
        loop {
            print!("\n[{}] {} -> [{}] ", i + 1, old, current_new);
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            
            if input.is_empty() {
                // Keep current name
                break;
            } else if input == "q" || input == "quit" {
                println!("Cancelled.");
                return false;
            } else if input == "s" || input == "skip" {
                // Skip this file by keeping old name
                rename.new_name = old.to_string();
                rename.new_path = rename.old_path.clone();
                break;
            } else if input == "a" || input == "apply" {
                // Apply all remaining
                apply_all = true;
                break;
            } else {
                // New name provided
                rename.new_name = input.to_string();
                if let Some(parent) = rename.old_path.parent() {
                    rename.new_path = parent.join(&rename.new_name);
                }
                break;
            }
        }
        
        if apply_all {
            // Apply pattern to all remaining files
            break;
        }
    }
    
    if apply_all {
        println!("\nApplying pattern to all remaining files...");
    }
    
    true
}
