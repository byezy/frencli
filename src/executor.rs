//! Command execution orchestration.
//! 
//! This module handles the execution of parsed subcommands, including:
//! - Standalone commands (undo, audit, interactive, template --list)
//! - Subcommand argument extraction
//! - Execution orchestration (list -> rename -> validate -> apply)

use freneng::RenamingEngine;
use crate::subcommands::{ParsedSubcommand, get_flag_value, has_flag, get_flag_values};
use crate::templates::TemplateRegistry;
use crate::list::find_files;
use crate::rename::handle_rename_command;
use crate::apply::handle_apply_command;
use crate::template::handle_template_command;
use crate::validate::handle_validate_command;
use crate::undo::{handle_undo_check, handle_undo_apply};
use crate::audit::handle_audit_command;
use crate::interactive::handle_interactive_command;
use std::path::PathBuf;
use std::fs;
use std::io::{self, BufRead};

/// Configuration extracted from subcommands
#[derive(Debug, Default)]
pub struct CommandConfig {
    pub list_patterns: Option<Vec<String>>,
    pub list_files_from: Option<String>,  // Path to file containing file list, or "-" for stdin
    pub list_recursive: bool,
    pub list_exclude: Vec<String>,
    pub list_fullpath: bool,
    pub list_json: bool,
    pub rename_pattern: Option<String>,
    pub rename_json: bool,
    pub template_use: Option<String>,
    pub validate_skip_invalid: bool,
    pub apply_overwrite: bool,
    pub apply_yes: bool,
    pub apply_interactive: bool,
    pub apply_json: bool,
}

/// Reads file paths from a file or stdin
/// 
/// # Arguments
/// 
/// * `source` - File path, or "-" for stdin
/// 
/// # Returns
/// 
/// * `Ok(Vec<PathBuf>)` - List of file paths
/// * `Err(String)` - Error message
fn read_files_from_source(source: &str) -> Result<Vec<PathBuf>, String> {
    let reader: Box<dyn BufRead> = if source == "-" {
        // Read from stdin
        Box::new(io::BufReader::new(io::stdin()))
    } else {
        // Read from file
        let file = fs::File::open(source)
            .map_err(|e| format!("Failed to open file '{}': {}", source, e))?;
        Box::new(io::BufReader::new(file))
    };
    
    let mut files = Vec::new();
    for (line_num, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| format!("Error reading line {}: {}", line_num + 1, e))?;
        let trimmed = line.trim();
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        files.push(PathBuf::from(trimmed));
    }
    
    Ok(files)
}

/// Handles standalone commands that must be used alone (undo, audit, interactive, template --list)
pub async fn handle_standalone_commands(
    subcommands: &[ParsedSubcommand],
    engine: &RenamingEngine,
    template_registry: &TemplateRegistry,
) -> Result<Option<()>, String> {
    // Check if template --list is present
    for subcmd in subcommands {
        if subcmd.name == "template" && has_flag(&subcmd.flags, "list") {
            handle_template_command(template_registry, true, None)
                .map_err(|e| format!("Error: {}", e))?;
            return Ok(Some(()));
        }
    }
    
    // Check if undo is present - it must be used alone
    let has_undo = subcommands.iter().any(|s| s.name == "undo");
    if has_undo {
        if subcommands.len() > 1 {
            return Err("'undo' cannot be used with other subcommands.\nThe 'undo' subcommand is very important and must be used alone.\n\nExamples:\n  frencli undo --check\n  frencli undo --apply\n  frencli undo --apply --yes".to_string());
        }
        
        let undo_subcmd = subcommands.iter().find(|s| s.name == "undo").unwrap();
        let has_check = has_flag(&undo_subcmd.flags, "check");
        let has_apply = has_flag(&undo_subcmd.flags, "apply");
        let undo_yes = has_flag(&undo_subcmd.flags, "yes");
        
        if has_check && has_apply {
            return Err("Cannot use both 'undo --check' and 'undo --apply' together.\nUse either:\n  - 'undo --check' to check what can be undone\n  - 'undo --apply' to actually perform the undo".to_string());
        }
        
        if has_check {
            handle_undo_check(engine).await;
            return Ok(Some(()));
        } else if has_apply {
            handle_undo_apply(engine, undo_yes).await;
            return Ok(Some(()));
        } else {
            return Err("'undo' requires either '--check' or '--apply' flag.\nUse:\n  - 'undo --check' to check what can be undone\n  - 'undo --apply' to actually perform the undo".to_string());
        }
    }
    
    // Check if audit is present - it must be used alone
    let has_audit = subcommands.iter().any(|s| s.name == "audit");
    if has_audit {
        if subcommands.len() > 1 {
            return Err("'audit' cannot be used with other subcommands.\nThe 'audit' subcommand is standalone and must be used alone.\n\nExamples:\n  frencli audit\n  frencli audit --limit 10\n  frencli audit --json".to_string());
        }
        
        let audit_subcmd = subcommands.iter().find(|s| s.name == "audit").unwrap();
        let limit_str = get_flag_value(&audit_subcmd.flags, "limit");
        let limit = limit_str.and_then(|s| s.parse::<usize>().ok());
        let json = has_flag(&audit_subcmd.flags, "json");
        
        handle_audit_command(limit, json).await
            .map_err(|e| format!("Error: {}", e))?;
        return Ok(Some(()));
    }
    
    // Check if interactive is present - it must be used alone
    let has_interactive = subcommands.iter().any(|s| s.name == "interactive");
    if has_interactive {
        if subcommands.len() > 1 {
            return Err("'interactive' cannot be used with other subcommands.\nThe 'interactive' subcommand is standalone and must be used alone.\n\nExample:\n  frencli interactive".to_string());
        }
        
        handle_interactive_command().await
            .map_err(|e| format!("Error: {}", e))?;
        return Ok(Some(()));
    }
    
    Ok(None)
}

/// Validates subcommand combinations
pub fn validate_subcommand_combinations(subcommands: &[ParsedSubcommand]) -> Result<(), String> {
    let has_rename = subcommands.iter().any(|s| s.name == "rename");
    let has_template_use = subcommands.iter().any(|s| {
        s.name == "template" && has_flag(&s.flags, "use")
    });
    
    if has_rename && has_template_use {
        return Err("Cannot use both 'rename' and 'template --use' in the same command.\nUse either:\n  - 'rename <PATTERN>' to specify a pattern directly\n  - 'template --use <NAME|NUMBER>' to use a template pattern".to_string());
    }
    
    Ok(())
}

/// Extracts configuration from parsed subcommands
pub fn extract_config(subcommands: &[ParsedSubcommand]) -> Result<CommandConfig, String> {
    let mut config = CommandConfig::default();
    
    for subcmd in subcommands {
        match subcmd.name.as_str() {
            "list" => {
                // Check for --files-from flag first
                if let Some(files_from) = get_flag_value(&subcmd.flags, "files-from") {
                    config.list_files_from = Some(files_from);
                } else {
                    // Use patterns if --files-from not provided
                    let patterns = subcmd.args.clone();
                    if patterns.is_empty() {
                        return Err("No search pattern provided for 'list'. Use patterns or --files-from.".to_string());
                    }
                    config.list_patterns = Some(patterns);
                }
                config.list_recursive = has_flag(&subcmd.flags, "recursive");
                config.list_exclude = get_flag_values(&subcmd.flags, "exclude");
                config.list_fullpath = has_flag(&subcmd.flags, "fullpath");
                config.list_json = has_flag(&subcmd.flags, "json");
            }
            "rename" => {
                let pattern = subcmd.args.first().cloned().unwrap_or_default();
                if pattern.is_empty() {
                    return Err("Rename pattern required.".to_string());
                }
                config.rename_pattern = Some(pattern);
                config.rename_json = has_flag(&subcmd.flags, "json");
            }
            "template" => {
                let use_template = get_flag_value(&subcmd.flags, "use");
                if let Some(name) = use_template {
                    config.template_use = Some(name);
                }
            }
            "validate" => {
                config.validate_skip_invalid = has_flag(&subcmd.flags, "skip-invalid");
            }
            "apply" => {
                config.apply_overwrite = has_flag(&subcmd.flags, "overwrite");
                config.apply_yes = has_flag(&subcmd.flags, "yes");
                config.apply_interactive = has_flag(&subcmd.flags, "interactive");
                config.apply_json = has_flag(&subcmd.flags, "json");
            }
            _ => {}
        }
    }
    
    Ok(config)
}

/// Resolves template pattern from name or index
pub fn resolve_template_pattern(
    template_registry: &TemplateRegistry,
    template_name: &str,
) -> Result<String, String> {
    if let Ok(index) = template_name.parse::<usize>() {
        let templates = template_registry.list();
        if index == 0 || index > templates.len() {
            return Err(format!("Template index {} out of range (1-{})", index, templates.len()));
        }
        Ok(templates[index - 1].1.clone())
    } else {
        template_registry.get(template_name)
            .cloned()
            .ok_or_else(|| format!("Unknown template '{}'. Use 'template --list' to see all available templates.", template_name))
    }
}

/// Gets the audit pattern from rename pattern or template
pub fn get_audit_pattern(
    rename_pattern: &Option<String>,
    template_use: &Option<String>,
    template_registry: &TemplateRegistry,
) -> Option<String> {
    rename_pattern.clone().or_else(|| {
        template_use.as_ref().and_then(|name| {
            resolve_template_pattern(template_registry, name).ok()
        })
    })
}

/// Executes the command pipeline: list -> rename -> validate -> apply
pub async fn execute_command_pipeline(
    config: CommandConfig,
    subcommands: &[ParsedSubcommand],
    engine: &RenamingEngine,
    template_registry: &TemplateRegistry,
    full_command: String,
) -> Result<(), String> {
    // Step 1: Execute list to get files (if present)
    let mut files: Vec<PathBuf> = Vec::new();
    let mut preview_result: Option<freneng::EnginePreviewResult> = None;
    
    // Read files from --files-from if provided, otherwise use patterns
    if let Some(files_from) = &config.list_files_from {
        // Read files from file or stdin
        files = read_files_from_source(files_from)
            .map_err(|e| format!("Error reading files from {}: {}", files_from, e))?;
        
        // Display files if rename/template --use/validate/apply is not present
        if config.rename_pattern.is_none() && config.template_use.is_none() 
            && !subcommands.iter().any(|s| s.name == "validate")
            && !subcommands.iter().any(|s| s.name == "apply") {
            if config.list_json {
                crate::list::display_files_json(&files, config.list_fullpath);
            } else {
                crate::list::display_files(&files, config.list_fullpath);
            }
        }
    } else if let Some(patterns) = &config.list_patterns {
        files = find_files(patterns, config.list_recursive, &config.list_exclude).await
            .map_err(|e| format!("Error finding files: {}", e))?;
        
        // Display files if rename/template --use/validate/apply is not present
        if config.rename_pattern.is_none() && config.template_use.is_none() 
            && !subcommands.iter().any(|s| s.name == "validate")
            && !subcommands.iter().any(|s| s.name == "apply") {
            if config.list_json {
                crate::list::display_files_json(&files, config.list_fullpath);
            } else {
                crate::list::display_files(&files, config.list_fullpath);
            }
        }
    }
    
    // Step 2: Execute rename or template --use to generate preview (if present)
    if let Some(pattern) = config.rename_pattern.clone() {
        if files.is_empty() {
            return Err("No files to process. 'list' subcommand is required to select files.".to_string());
        }
        
        preview_result = Some(handle_rename_command(&engine, files.clone(), pattern, config.rename_json).await
            .map_err(|e| format!("Error: {}", e))?);
    } else if let Some(template_name) = config.template_use.clone() {
        if files.is_empty() {
            return Err("'template --use' requires 'list' subcommand to select files.".to_string());
        }
        
        let pattern = resolve_template_pattern(template_registry, &template_name)?;
        preview_result = Some(handle_rename_command(&engine, files.clone(), pattern, config.rename_json).await
            .map_err(|e| format!("Error: {}", e))?);
    }
    
    // Step 3: Execute validate (if present)
    if subcommands.iter().any(|s| s.name == "validate") {
        let result = preview_result.as_ref()
            .ok_or("No preview available. 'rename' or 'template --use' subcommand is required to generate preview.")?;
        handle_validate_command(&engine, result, config.validate_skip_invalid).await;
    }
    
    // Step 4: Execute apply (if present)
    if subcommands.iter().any(|s| s.name == "apply") {
        let result = preview_result.take()
            .ok_or("No preview available. 'rename' or 'template --use' subcommand is required to generate preview.")?;
        
        let enable_audit = !subcommands.iter()
            .any(|s| s.name == "apply" && has_flag(&s.flags, "no-audit"));
        
        let audit_pattern = get_audit_pattern(
            &config.rename_pattern,
            &config.template_use,
            template_registry,
        );
        
        handle_apply_command(
            result, 
            config.apply_overwrite, 
            config.apply_yes, 
            config.apply_interactive,
            format!("frencli {}", full_command),
            audit_pattern,
            enable_audit,
            config.apply_json,
        ).await
            .map_err(|e| format!("Error: {}", e))?;
    }
    
    Ok(())
}

