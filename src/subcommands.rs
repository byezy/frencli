//! Custom parser for handling multiple subcommands in a single invocation.
//! 
//! Allows commands like: `fren list *.txt make "%N.%E" rename`
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
/// Recognizes subcommands: list, make, validate, rename, template, undo, audit
/// Extracts their arguments and flags.
pub fn parse_multi_subcommand(args: Vec<String>) -> Vec<ParsedSubcommand> {
    let mut subcommands = Vec::new();
    let mut i = 0;
    
    // Known subcommand names
    let known_subcommands = ["list", "make", "validate", "rename", "template", "undo", "audit", "interactive"];
    
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
                    
                    // Boolean flags that don't accept values
                    let boolean_flags = ["yes", "overwrite", "recursive", "fullpath", "skip-invalid", 
                                         "interactive", "check", "apply", "json", "no-audit", "help"];
                    let is_boolean_flag = boolean_flags.contains(&flag_name.as_str());
                    
                    if is_boolean_flag {
                        // Boolean flags don't accept values - just mark the flag as present
                        flags.insert(flag_name, Vec::new());
                    } else {
                        // Collect flag values (until next flag or subcommand)
                        while i < args.len() {
                            let val = &args[i];
                            if val.starts_with("--") || known_subcommands.contains(&val.as_str()) {
                                break;
                            }
                            // For non-boolean flags (like --exclude, --use), allow values starting with '-'
                            // as they could be filenames/patterns
                            flag_values.push(val.clone());
                            i += 1;
                        }
                        flags.insert(flag_name, flag_values);
                    }
                } else if next_arg.starts_with("-") && !next_arg.starts_with("--") && next_arg.len() > 1 {
                    // Single dash argument (like -y, -r, etc.)
                    // Only --<something> is interpreted as flags. Single dash arguments
                    // are treated as positional arguments (filenames/patterns) for subcommands
                    // that accept them, or rejected if the subcommand doesn't accept positional args.
                    let accepts_positional_args = matches!(subcommand_name.as_str(), "list" | "make");
                    
                    if accepts_positional_args {
                        // This could be a filename or pattern starting with '-', treat as positional arg
                        subcommand_args.push(next_arg.clone());
                        i += 1;
                    } else {
                        // This subcommand doesn't accept positional args, so -X is clearly a short flag attempt
                        eprintln!("Error: Short flags (like '{}') are not supported.", next_arg);
                        eprintln!("Please use the long form instead (e.g., '--yes' instead of '-y').");
                        eprintln!("\nCommon short flag mappings:");
                        eprintln!("  -y, -Y  →  --yes");
                        eprintln!("  -o, -O  →  --overwrite");
                        eprintln!("  -r, -R  →  --recursive");
                        eprintln!("  -e, -E  →  --exclude");
                        eprintln!("  -h, -H  →  --help");
                        eprintln!("  -f, -F  →  --fullpath");
                        eprintln!("  -V, -v  →  --version");
                        std::process::exit(1);
                    }
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

