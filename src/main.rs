use freneng::RenamingEngine;

mod ui;
mod templates;
mod subcommands;
mod template;
mod help;
mod executor;
pub mod list;
pub mod make;
pub mod rename;
pub mod validate;
pub mod undo;
pub mod audit;
use subcommands::{parse_multi_subcommand, has_flag};
use executor::{handle_standalone_commands, validate_subcommand_combinations, extract_config, execute_command_pipeline};
use templates::TemplateRegistry;

/// Print version information
fn print_version() {
    println!("fren {}", env!("CARGO_PKG_VERSION"));
}

#[tokio::main]
async fn main() {
    let raw_args: Vec<String> = std::env::args().skip(1).collect();
    
    // Handle top-level flags before parsing subcommands
    // Only check if --help/--version is the first argument (top-level)
    // NO SHORT FLAGS - only long forms are supported
    if !raw_args.is_empty() {
        let first_arg = &raw_args[0];
        if first_arg == "--version" {
            print_version();
            return;
        }
        if first_arg == "--help" {
            help::print_main_help();
            return;
        }
        // Only --<something> is interpreted as flags at top level
        // Single dash arguments are not flags, they would be subcommands or positional args
        // So we don't need to reject them here - they'll be handled by the parser
    }
    
    // Store full command for audit logging
    let full_command = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
    
    // Parse subcommands
    let subcommands = parse_multi_subcommand(raw_args);
    
    if subcommands.is_empty() {
        // No subcommands - show help
        help::print_main_help();
        return;
    }
    
    // Check for --help flags in subcommands
    // --help is only valid for a single subcommand, not in mixed commands
    let help_subcommands: Vec<_> = subcommands.iter()
        .filter(|s| has_flag(&s.flags, "help"))
        .collect();
    
    if !help_subcommands.is_empty() {
        // If multiple subcommands present with --help, error
        if subcommands.len() > 1 {
            eprintln!("Error: '--help' cannot be used with multiple subcommands.");
            eprintln!("Use '--help' with a single subcommand, e.g.:");
            eprintln!("  fren list --help");
            eprintln!("  fren rename --help");
            eprintln!("  fren validate --help");
            std::process::exit(1);
        }
        
        // Show help for the single subcommand
        let subcmd_name = help_subcommands[0].name.as_str();
        help::print_subcommand_help(subcmd_name);
        return;
    }
    
    let engine = RenamingEngine;
    let template_registry = TemplateRegistry::new();
    
    // Handle standalone commands (undo, audit, template --list)
    match handle_standalone_commands(&subcommands, &engine, &template_registry).await {
        Ok(Some(_)) => return, // Command executed and completed
        Ok(None) => {}, // No standalone command, continue
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
    
    // Validate subcommand combinations
    if let Err(e) = validate_subcommand_combinations(&subcommands) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    
    // Extract configuration from subcommands
    let config = match extract_config(&subcommands) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    
    // Execute command pipeline
    if let Err(e) = execute_command_pipeline(
        config,
        &subcommands,
        &engine,
        &template_registry,
        full_command,
    ).await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}



