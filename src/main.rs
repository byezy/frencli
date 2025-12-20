use clap::{Parser, Subcommand};
use freneng::fs_ops::perform_renames;
use freneng::RenamingEngine;
use freneng::history::save_history;

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
use ui::{display_preview, confirm_renames, interactive_edit};
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

#[derive(Parser, Debug)]
#[command(name = "fren")]
#[command(about = "Batch file renamer with pattern matching", long_about = None)]
#[command(version)]
struct Args {
    /// Raw arguments for custom multi-subcommand parsing
    #[arg(skip)]
    raw_args: Vec<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List files matching patterns
    List {
        /// Search patterns (glob patterns, e.g., "*.txt")
        #[arg(num_args = 1.., value_name = "PATTERN")]
        pattern: Vec<String>,
        
        /// Recursively search subdirectories (supports ** glob pattern)
        #[arg(long = "recursive")]
        recursive: bool,
        
        /// Exclude files matching these patterns
        #[arg(long = "exclude")]
        exclude: Vec<String>,
        
        /// Display full paths instead of just filenames
        #[arg(long = "fullpath")]
        fullpath: bool,
        
        /// Chain to rename command with this pattern
        #[arg(long = "rename", value_name = "RENAME_PATTERN")]
        rename_pattern: Option<String>,
        
        /// Overwrite existing files (when using --rename)
        #[arg(long = "overwrite")]
        overwrite: bool,
        
        /// Skip confirmation prompt (when using --rename)
        #[arg(long = "yes")]
        yes: bool,
    },
    
    /// Validate a rename pattern
    Validate {
        /// Search patterns (glob patterns, e.g., "*.txt")
        #[arg(num_args = 1.., value_name = "PATTERN")]
        pattern: Vec<String>,
        
        /// Recursively search subdirectories (supports ** glob pattern)
        #[arg(short = 'r', long = "recursive")]
        recursive: bool,
        
        /// Exclude files matching these patterns
        #[arg(short = 'e', long = "exclude")]
        exclude: Vec<String>,
        
        /// Skip invalid files instead of aborting
        #[arg(long = "skip-invalid")]
        skip_invalid: bool,
        
        /// Template specification (use either --change or --template, not both)
        #[command(flatten)]
        template_spec: TemplateSpec,
    },
    
    /// Apply rename operations
    #[command(subcommand)]
    Apply(ApplyCommands),
    
    /// Directly rename files (applies immediately)
    /// 
    /// Operates on files from the last `list` command.
    /// Run `fren list` first to select files, then use `fren rename` to rename them.
    Rename {
        /// Rename pattern/template (e.g., "%N.%E", "%N2-7.%E")
        #[arg(value_name = "RENAME_PATTERN")]
        rename_pattern: String,
        
        /// Overwrite existing files
        #[arg(long = "overwrite")]
        overwrite: bool,
        
        /// Skip confirmation prompt
        #[arg(long = "yes")]
        yes: bool,
    },
    
    /// Template operations
    #[command(subcommand)]
    Template(TemplateCommands),
    
    /// Undo operations
    #[command(subcommand)]
    Undo(UndoCommands),
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
pub struct TemplateSpec {
    /// Renaming template (supports %N, %E, %F, %C, %P, %D, %L, %U, %T, %M, %R, %X)
    #[arg(long = "change", value_name = "TEMPLATE")]
    change: Option<String>,
    
    /// Use a preset template pattern (e.g., photo-date, lowercase, counter-3)
    #[arg(long = "template", value_name = "TEMPLATE_NAME")]
    template: Option<String>,
}

#[derive(Subcommand, Debug)]
enum ApplyCommands {
    /// Apply rename automatically
    Auto {
        /// Search patterns (glob patterns, e.g., "*.txt")
        #[arg(num_args = 1.., value_name = "PATTERN")]
        pattern: Vec<String>,
        
        /// Recursively search subdirectories (supports ** glob pattern)
        #[arg(short = 'r', long = "recursive")]
        recursive: bool,
        
        /// Exclude files matching these patterns
        #[arg(short = 'e', long = "exclude")]
        exclude: Vec<String>,
        
        /// Overwrite existing files
        #[arg(short = 'o', long = "overwrite")]
        overwrite: bool,
        
        /// Skip confirmation prompt
        #[arg(short = 'y', long = "yes")]
        yes: bool,
        
        /// Template specification (use either --change or --template, not both)
        #[command(flatten)]
        template_spec: TemplateSpec,
    },
    
    /// Apply rename interactively
    Interactive {
        /// Search patterns (glob patterns, e.g., "*.txt")
        #[arg(num_args = 1.., value_name = "PATTERN")]
        pattern: Vec<String>,
        
        /// Recursively search subdirectories (supports ** glob pattern)
        #[arg(short = 'r', long = "recursive")]
        recursive: bool,
        
        /// Exclude files matching these patterns
        #[arg(short = 'e', long = "exclude")]
        exclude: Vec<String>,
        
        /// Overwrite existing files
        #[arg(short = 'o', long = "overwrite")]
        overwrite: bool,
        
        /// Template specification (use either --change or --template, not both)
        #[command(flatten)]
        template_spec: TemplateSpec,
    },
}

#[derive(Subcommand, Debug)]
enum TemplateCommands {
    /// List all available template patterns
    List,
    
    /// Use a specific template (returns the pattern)
    Use {
        /// Template name
        #[arg(value_name = "NAME")]
        name: String,
    },
}

#[derive(Subcommand, Debug)]
enum UndoCommands {
    /// Check undo status
    Check,
    
    /// Apply undo
    Apply {
        /// Skip confirmation prompt
        #[arg(short = 'y', long = "yes")]
        yes: bool,
    },
}

#[tokio::main]
async fn main() {
    let raw_args: Vec<String> = std::env::args().skip(1).collect();
    
    // Store full command for audit logging
    let full_command = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
    
    // Parse subcommands
    let subcommands = parse_multi_subcommand(raw_args);
    
    if subcommands.is_empty() {
        // No subcommands - show help
        Args::parse_from(["fren", "--help"]);
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
    let mut validate_overwrite = false;
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
                validate_overwrite = has_flag(&subcmd.flags, "overwrite");
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
        
        handle_validate_command(&engine, result, validate_overwrite, validate_skip_invalid).await;
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

async fn handle_validate(engine: &RenamingEngine, matches: Vec<std::path::PathBuf>, template: String, skip_invalid: bool) {
    match engine.generate_preview(&matches, &template).await {
        Ok(result) => {
            let mut has_errors = false;
            
            if result.has_empty_names {
                if skip_invalid {
                    println!("WARNING: {} file(s) would have empty names (skipped)", 
                        result.renames.iter().filter(|r| r.new_name.trim().is_empty()).count());
                } else {
                    eprintln!("ERROR: Pattern would generate empty filenames.");
                    has_errors = true;
                }
            }
            
            if !result.warnings.is_empty() {
                println!("WARNINGS:");
                for warning in &result.warnings {
                    println!("  - {}", warning);
                }
                if !skip_invalid {
                    has_errors = true;
                }
            }
            
            if has_errors && !skip_invalid {
                std::process::exit(1);
            }
            
            let valid_count = result.renames.iter()
                .filter(|r| !r.new_name.trim().is_empty())
                .count();
            println!("✓ Pattern is valid for {} file(s)", valid_count);
        }
        Err(e) => {
            if skip_invalid {
                eprintln!("WARNING: {}", e);
            } else {
                eprintln!("ERROR: {}", e);
                std::process::exit(1);
            }
        }
    }
}

async fn handle_rename(
    engine: &RenamingEngine,
    _template_registry: &TemplateRegistry,
    matches: Vec<std::path::PathBuf>,
    template: String,
    apply: bool,
    overwrite: bool,
    yes: bool,
    already_confirmed: bool,
) {
    let preview_result = match engine.generate_preview(&matches, &template).await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error generating rename patterns: {}", e);
            std::process::exit(1);
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
        eprintln!("\nERROR: One or more files would have an empty name. Renaming aborted.");
        eprintln!("Please check your pattern and ensure it generates valid filenames.");
        std::process::exit(1);
    }

    // Apply if requested
    if apply {
        let should_proceed = already_confirmed || yes || confirm_renames();
        
        if should_proceed {
            match perform_renames(&preview_result.renames, overwrite).await {
                Ok(execution) => {
                    // Report skips/errors
                    for (path, reason) in execution.skipped {
                        println!("Skipping {}: {}", path.display(), reason);
                    }
                    for (path, err) in execution.errors {
                        eprintln!("Error renaming {}: {}", path.display(), err);
                    }

                    if !execution.successful.is_empty() {
                        println!("Successfully processed {} file(s)", execution.successful.len());
                        if let Err(e) = save_history(execution.successful).await {
                            eprintln!("Warning: Could not save rename history: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error during renaming: {}", e);
                    std::process::exit(1);
                }
            }
        } else {
            println!("Renaming cancelled.");
        }
    } else {
        println!("\nPreview mode. Use 'fren rename' or 'fren apply' to perform the renaming.");
    }
}

async fn handle_interactive(
    engine: &RenamingEngine,
    _template_registry: &TemplateRegistry,
    matches: Vec<std::path::PathBuf>,
    template: String,
    overwrite: bool,
) {
    let mut preview_result = match engine.generate_preview(&matches, &template).await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error generating rename patterns: {}", e);
            std::process::exit(1);
        }
    };

    // Interactive editing
    if !interactive_edit(&mut preview_result.renames) {
        println!("Interactive editing cancelled.");
        return;
    }

    // After interactive editing, re-validate
    preview_result.has_empty_names = preview_result.renames.iter()
        .any(|r| r.new_name.trim().is_empty());

    // Show warnings
    if !preview_result.warnings.is_empty() {
        println!("\nWARNINGS:");
        for warning in &preview_result.warnings {
            println!("  - {}", warning);
        }
    }

    // Block if empty names
    if preview_result.has_empty_names {
        eprintln!("\nERROR: One or more files would have an empty name. Renaming aborted.");
        eprintln!("Please check your pattern and ensure it generates valid filenames.");
        std::process::exit(1);
    }

    // Apply (interactive mode already confirmed during editing)
    match perform_renames(&preview_result.renames, overwrite).await {
        Ok(execution) => {
            // Report skips/errors
            for (path, reason) in execution.skipped {
                println!("Skipping {}: {}", path.display(), reason);
            }
            for (path, err) in execution.errors {
                eprintln!("Error renaming {}: {}", path.display(), err);
            }

            if !execution.successful.is_empty() {
                println!("Successfully processed {} file(s)", execution.successful.len());
                if let Err(e) = save_history(execution.successful).await {
                    eprintln!("Warning: Could not save rename history: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error during renaming: {}", e);
            std::process::exit(1);
        }
    }
}


