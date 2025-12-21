use freneng::RenamingEngine;

mod ui;
mod templates;
mod subcommands;
mod template;
pub mod list;
pub mod transform;
pub mod rename;
pub mod validate;
pub mod undo;
pub mod audit;
use templates::TemplateRegistry;
use list::find_files;
use transform::handle_transform_command;
use rename::handle_rename_command;
use template::handle_template_command;
use validate::handle_validate_command;
use undo::{handle_undo_check, handle_undo_apply};
use audit::handle_audit_command;
use subcommands::{parse_multi_subcommand, get_flag_value, has_flag, get_flag_values};
use std::path::PathBuf;

/// Print version information
fn print_version() {
    println!("fren {}", env!("CARGO_PKG_VERSION"));
}

/// Print main help message
fn print_help() {
    println!("Batch file renamer with pattern matching");
    println!();
    println!("Usage: fren <SUBCOMMAND>...");
    println!();
    println!("Subcommands:");
    println!("  list <PATTERN>...              List files matching patterns");
    println!("  transform <PATTERN>            Transform file names using a pattern");
    println!("  validate <PATTERN>...          Validate a rename pattern");
    println!("  rename <PATTERN>               Rename files (applies immediately)");
    println!("  template --list                List available templates");
    println!("  template --use <NAME>           Use a template pattern");
    println!("  undo --check                   Check undo status");
    println!("  undo --apply                   Apply undo");
    println!("  audit                          View audit log");
    println!("  interactive                    Apply rename interactively");
    println!();
    println!("Options:");
    println!("  --help                         Print help");
    println!("  --version                      Print version");
    println!();
    println!("Examples:");
    println!("  fren list *.txt");
    println!("  fren list *.txt transform \"%N_backup.%E\"");
    println!("  fren list *.txt transform \"%N_backup.%E\" rename --yes");
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
            print_help();
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
        print_help();
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
        
        // Manually print help for each subcommand
        match subcmd_name {
            "rename" => {
                println!("Directly rename files (applies immediately)");
                println!();
                println!("Operates on files from the last `list` command.");
                println!("Run `fren list` first to select files, then use `fren rename` to rename them.");
                println!();
                println!("Usage: fren rename <RENAME_PATTERN> [OPTIONS]");
                println!();
                println!("Arguments:");
                println!("  <RENAME_PATTERN>  Rename pattern/template (e.g., \"%N.%E\", \"%N2-7.%E\")");
                println!();
                println!("Options:");
                println!("      --overwrite    Overwrite existing files");
                println!("      --yes          Skip confirmation prompt");
                println!("  -h, --help         Print help");
            }
            "list" => {
                println!("List files matching patterns");
                println!();
                println!("Usage: fren list <PATTERN>... [OPTIONS]");
                println!();
                println!("Arguments:");
                println!("  <PATTERN>...  Search patterns (glob patterns, e.g., \"*.txt\")");
                println!();
                println!("Options:");
                println!("      --recursive    Recursively search subdirectories (supports ** glob pattern)");
                println!("      --exclude <EXCLUDE>...  Exclude files matching these patterns");
                println!("      --fullpath     Display full paths instead of just filenames");
                println!("      --rename <RENAME_PATTERN>  Chain to rename command with this pattern");
                println!("      --overwrite    Overwrite existing files (when using --rename)");
                println!("      --yes          Skip confirmation prompt (when using --rename)");
                println!("  -h, --help         Print help");
            }
            "validate" => {
                println!("Validate a rename pattern");
                println!();
                println!("Usage: fren validate <PATTERN>... [OPTIONS]");
                println!();
                println!("Arguments:");
                println!("  <PATTERN>...  Search patterns (glob patterns, e.g., \"*.txt\")");
                println!();
                println!("Options:");
                println!("  -r, --recursive    Recursively search subdirectories (supports ** glob pattern)");
                println!("  -e, --exclude <EXCLUDE>...  Exclude files matching these patterns");
                println!("      --skip-invalid  Skip invalid files instead of aborting");
                println!("      --change <TEMPLATE>  Renaming template");
                println!("      --template <TEMPLATE_NAME>  Use a preset template pattern");
                println!("  -h, --help         Print help");
            }
            "transform" => {
                eprintln!("Transform subcommand");
                eprintln!("Usage: fren list <patterns> transform <RENAME_PATTERN>");
                eprintln!("\nTransforms file names using a pattern without applying the rename.");
            }
            "template" => {
                eprintln!("Template subcommand");
                eprintln!("Usage:");
                eprintln!("  fren template --list                    List available templates");
                eprintln!("  fren list <patterns> template --use <NAME>  Use a template");
            }
            "undo" => {
                eprintln!("Undo subcommand");
                eprintln!("Usage:");
                eprintln!("  fren undo --check   Check undo status");
                eprintln!("  fren undo --apply    Apply undo (use --yes to skip confirmation)");
            }
            "audit" => {
                eprintln!("Audit subcommand");
                eprintln!("Usage: fren audit [--limit <n>] [--json]");
                eprintln!("View audit log of rename operations.");
            }
            "interactive" => {
                println!("Apply rename interactively");
                println!();
                println!("Usage: fren list <PATTERN>... [OPTIONS] transform <RENAME_PATTERN> interactive");
                println!();
                println!("Arguments:");
                println!("  <PATTERN>...  Search patterns (glob patterns, e.g., \"*.txt\")");
                println!("  <RENAME_PATTERN>  Rename pattern/template");
                println!();
                println!("Options:");
                println!("      --recursive    Recursively search subdirectories");
                println!("      --exclude <EXCLUDE>...  Exclude files matching these patterns");
                println!("      --overwrite    Overwrite existing files");
                println!("  -h, --help         Print help");
            }
            _ => {
                eprintln!("Unknown subcommand: {}", subcmd_name);
                print_help();
            }
        }
        return;
    }
    
    let engine = RenamingEngine;
    let template_registry = TemplateRegistry::new();
    
    // Check if template --list is present - if so, execute it alone and exit
    for subcmd in &subcommands {
        if subcmd.name == "template" && has_flag(&subcmd.flags, "list") {
            match handle_template_command(&template_registry, true, None) {
                Ok(_) => return,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
    
    // Check if undo is present - it must be used alone (standalone command)
    let has_undo = subcommands.iter().any(|s| s.name == "undo");
    if has_undo {
        // Check if undo is mixed with other subcommands
        if subcommands.len() > 1 {
            eprintln!("Error: 'undo' cannot be used with other subcommands.");
            eprintln!("The 'undo' subcommand is very important and must be used alone.");
            eprintln!("\nExamples:");
            eprintln!("  fren undo --check");
            eprintln!("  fren undo --apply");
            eprintln!("  fren undo --apply --yes");
            std::process::exit(1);
        }
        
        // Find the undo subcommand and execute it
        let undo_subcmd = subcommands.iter().find(|s| s.name == "undo").unwrap();
        let has_check = has_flag(&undo_subcmd.flags, "check");
        let has_apply = has_flag(&undo_subcmd.flags, "apply");
        let undo_yes = has_flag(&undo_subcmd.flags, "yes");
        
        if has_check && has_apply {
            eprintln!("Error: Cannot use both 'undo --check' and 'undo --apply' together.");
            eprintln!("Use either:");
            eprintln!("  - 'undo --check' to check what can be undone");
            eprintln!("  - 'undo --apply' to actually perform the undo");
            std::process::exit(1);
        }
        
        if has_check {
            handle_undo_check(&engine).await;
            return;
        } else if has_apply {
            handle_undo_apply(&engine, undo_yes).await;
            return;
        } else {
            eprintln!("Error: 'undo' requires either '--check' or '--apply' flag.");
            eprintln!("Use:");
            eprintln!("  - 'undo --check' to check what can be undone");
            eprintln!("  - 'undo --apply' to actually perform the undo");
            std::process::exit(1);
        }
    }
    
    // Check if audit is present - it must be used alone (standalone command)
    let has_audit = subcommands.iter().any(|s| s.name == "audit");
    if has_audit {
        // Check if audit is mixed with other subcommands
        if subcommands.len() > 1 {
            eprintln!("Error: 'audit' cannot be used with other subcommands.");
            eprintln!("The 'audit' subcommand is standalone and must be used alone.");
            eprintln!("\nExamples:");
            eprintln!("  fren audit");
            eprintln!("  fren audit --limit 10");
            eprintln!("  fren audit --json");
            std::process::exit(1);
        }
        
        // Find the audit subcommand and execute it
        let audit_subcmd = subcommands.iter().find(|s| s.name == "audit").unwrap();
        let limit_str = get_flag_value(&audit_subcmd.flags, "limit");
        let limit = limit_str.and_then(|s| s.parse::<usize>().ok());
        let json = has_flag(&audit_subcmd.flags, "json");
        
        match handle_audit_command(limit, json).await {
            Ok(_) => return,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
    
    // Check for conflicting commands: transform and template --use cannot both be present
    let has_transform = subcommands.iter().any(|s| s.name == "transform");
    let has_template_use = subcommands.iter().any(|s| {
        s.name == "template" && has_flag(&s.flags, "use")
    });
    
    if has_transform && has_template_use {
        eprintln!("Error: Cannot use both 'transform' and 'template --use' in the same command.");
        eprintln!("Use either:");
        eprintln!("  - 'transform <PATTERN>' to specify a pattern directly");
        eprintln!("  - 'template --use <NAME|NUMBER>' to use a template pattern");
        std::process::exit(1);
    }
    
    // Collect information from all subcommands first (order doesn't matter)
    let mut list_patterns: Option<Vec<String>> = None;
    let mut list_recursive = false;
    let mut list_exclude: Vec<String> = Vec::new();
    let mut list_fullpath = false;
    let mut transform_pattern: Option<String> = None;
    let mut template_use: Option<String> = None;
    let mut validate_skip_invalid = false;
    let mut rename_overwrite = false;
    let mut rename_yes = false;
    let mut rename_interactive = false;
    
    for subcmd in &subcommands {
        match subcmd.name.as_str() {
            "list" => {
                let patterns = subcmd.args.clone();
                if patterns.is_empty() {
                    eprintln!("Error: No search pattern provided for 'list'.");
                    std::process::exit(1);
                }
                list_patterns = Some(patterns);
                list_recursive = has_flag(&subcmd.flags, "recursive");
                list_exclude = get_flag_values(&subcmd.flags, "exclude");
                list_fullpath = has_flag(&subcmd.flags, "fullpath");
            }
            "transform" => {
                let pattern = subcmd.args.first().cloned().unwrap_or_default();
                if pattern.is_empty() {
                    eprintln!("Error: Transform pattern required.");
                    std::process::exit(1);
                }
                transform_pattern = Some(pattern);
            }
            "template" => {
                let use_template = get_flag_value(&subcmd.flags, "use");
                if let Some(name) = use_template {
                    template_use = Some(name);
                }
            }
            "validate" => {
                validate_skip_invalid = has_flag(&subcmd.flags, "skip-invalid");
            }
            "rename" => {
                rename_overwrite = has_flag(&subcmd.flags, "overwrite");
                rename_yes = has_flag(&subcmd.flags, "yes");
                rename_interactive = has_flag(&subcmd.flags, "interactive");
                // Note: --no-audit flag is handled separately below
            }
            _ => {}
        }
    }
    
    // Now execute in logical order (order on command line is irrelevant)
    let mut files: Vec<PathBuf> = Vec::new();
    let mut preview_result: Option<freneng::EnginePreviewResult> = None;
    
    // Store pattern for audit logging (before it gets moved)
    let audit_pattern = transform_pattern.clone().or_else(|| {
        template_use.as_ref().and_then(|name| {
            if let Ok(index) = name.parse::<usize>() {
                let templates = template_registry.list();
                if index > 0 && index <= templates.len() {
                    Some(templates[index - 1].1.clone())
                } else {
                    None
                }
            } else {
                template_registry.get(name).cloned()
            }
        })
    });
    
    // Step 1: Execute list to get files (if present)
    if let Some(patterns) = list_patterns {
        match find_files(&patterns, list_recursive, &list_exclude).await {
            Ok(found_files) => {
                files = found_files;
                // Display files if transform/template --use/validate/rename is not present
                if transform_pattern.is_none() && template_use.is_none() 
                    && !subcommands.iter().any(|s| s.name == "validate")
                    && !subcommands.iter().any(|s| s.name == "rename") {
                    list::display_files(&files, list_fullpath);
                }
            }
            Err(e) => {
                eprintln!("Error finding files: {}", e);
                std::process::exit(1);
            }
        }
    }
    
    // Step 2: Execute transform or template --use to generate preview (if present)
    if let Some(pattern) = transform_pattern {
        if files.is_empty() {
            eprintln!("Error: No files to transform. 'list' subcommand is required to select files.");
            std::process::exit(1);
        }
        
        match handle_transform_command(&engine, files.clone(), pattern).await {
            Ok(result) => {
                preview_result = Some(result);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else if let Some(template_name) = template_use {
        if files.is_empty() {
            eprintln!("Error: 'template --use' requires 'list' subcommand to select files.");
            std::process::exit(1);
        }
        
        // Get the template pattern
        let pattern = if let Ok(index) = template_name.parse::<usize>() {
            let templates = template_registry.list();
            if index == 0 || index > templates.len() {
                eprintln!("Error: Template index {} out of range (1-{})", index, templates.len());
                std::process::exit(1);
            }
            templates[index - 1].1.clone()
        } else {
            match template_registry.get(&template_name) {
                Some(p) => p.clone(),
                None => {
                    eprintln!("Error: Unknown template '{}'. Use 'template --list' to see all available templates.", template_name);
                    std::process::exit(1);
                }
            }
        };
        
        // Use the template pattern like transform would
        match handle_transform_command(&engine, files.clone(), pattern).await {
            Ok(result) => {
                preview_result = Some(result);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
    
    // Step 3: Execute validate (if present)
    if subcommands.iter().any(|s| s.name == "validate") {
        let result = preview_result.as_ref().ok_or_else(|| {
            eprintln!("Error: No preview available. 'transform' or 'template --use' subcommand is required to generate preview.");
            std::process::exit(1);
        }).unwrap();
        
        handle_validate_command(&engine, result, validate_skip_invalid).await;
    }
    
    // Step 4: Execute rename (if present)
    if subcommands.iter().any(|s| s.name == "rename") {
        let result = preview_result.take().unwrap_or_else(|| {
            eprintln!("Error: No preview available. 'transform' or 'template --use' subcommand is required to generate preview.");
            std::process::exit(1);
        });
        
        // Check if audit is disabled
        let enable_audit = !subcommands.iter()
            .any(|s| s.name == "rename" && has_flag(&s.flags, "no-audit"));
        
        if let Err(e) = handle_rename_command(
            result, 
            rename_overwrite, 
            rename_yes, 
            rename_interactive,
            format!("fren {}", full_command),
            audit_pattern.clone(),
            enable_audit,
        ).await {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Template specification extracted from command line
pub struct TemplateSpec {
    pub change: Option<String>,
    pub template: Option<String>,
}

pub fn resolve_template(template_registry: &TemplateRegistry, template_spec: &TemplateSpec) -> String {
    if let Some(template) = &template_spec.change {
        template.clone()
    } else if let Some(name) = &template_spec.template {
        match template_registry.get(name) {
            Some(pattern) => {
                println!("Using template '{}': {}", name, pattern);
                pattern.clone()
            }
            None => {
                eprintln!("Error: Unknown template '{}'", name);
                eprintln!("Use 'fren template list' to see all available templates");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("Error: Template required. Use --change or --template");
        std::process::exit(1);
    }
}


