//! Custom parser for handling multiple subcommands in a single invocation.
//! 
//! Allows commands like: `fren list *.txt transform "%N.%E" rename`
//! Order of subcommands doesn't matter - they're executed in logical order.
//! 
//! Standalone commands (undo, audit) must be used alone.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ParsedSubcommand {
    pub name: String,
    pub args: Vec<String>,
    pub flags: HashMap<String, Vec<String>>,
}

/// Parses command line arguments into subcommands.
/// 
/// Recognizes subcommands: list, transform, validate, rename, template, undo, audit
/// Extracts their arguments and flags.
pub fn parse_multi_subcommand(args: Vec<String>) -> Vec<ParsedSubcommand> {
    let mut subcommands = Vec::new();
    let mut i = 0;
    
    // Known subcommand names
    let known_subcommands = ["list", "transform", "validate", "rename", "template", "undo", "audit"];
    
    while i < args.len() {
        let arg = &args[i];
        
        // Check if this is a subcommand
        if known_subcommands.contains(&arg.as_str()) {
            let subcommand_name = arg.clone();
            let mut subcommand_args = Vec::new();
            let mut flags: HashMap<String, Vec<String>> = HashMap::new();
            i += 1;
            
            // Collect arguments until next subcommand or end
            while i < args.len() {
                let next_arg = &args[i];
                
                // Check if next arg is a subcommand
                if known_subcommands.contains(&next_arg.as_str()) {
                    break;
                }
                
                // Check if it's a flag
                if next_arg.starts_with("--") {
                    let flag_name = next_arg[2..].to_string();
                    let mut flag_values = Vec::new();
                    i += 1;
                    
                    // Collect flag values (until next flag or subcommand)
                    while i < args.len() {
                        let val = &args[i];
                        if val.starts_with("--") || known_subcommands.contains(&val.as_str()) {
                            break;
                        }
                        flag_values.push(val.clone());
                        i += 1;
                    }
                    
                    flags.insert(flag_name, flag_values);
                } else {
                    // Regular argument
                    subcommand_args.push(next_arg.clone());
                    i += 1;
                }
            }
            
            subcommands.push(ParsedSubcommand {
                name: subcommand_name,
                args: subcommand_args,
                flags,
            });
        } else {
            i += 1;
        }
    }
    
    subcommands
}

/// Gets a flag value, returning the first value if multiple exist
pub fn get_flag_value(flags: &HashMap<String, Vec<String>>, flag_name: &str) -> Option<String> {
    flags.get(flag_name).and_then(|v| v.first().cloned())
}

/// Checks if a flag is present (boolean flag)
pub fn has_flag(flags: &HashMap<String, Vec<String>>, flag_name: &str) -> bool {
    flags.contains_key(flag_name)
}

/// Gets all values for a flag
pub fn get_flag_values(flags: &HashMap<String, Vec<String>>, flag_name: &str) -> Vec<String> {
    flags.get(flag_name).cloned().unwrap_or_default()
}

