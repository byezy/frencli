//! Command execution orchestration.
//! 
//! This module handles the execution of parsed subcommands, including:
//! - Standalone commands (undo, audit, template --list)
//! - Subcommand argument extraction
//! - Execution orchestration (list -> make -> validate -> rename)

use freneng::RenamingEngine;
use crate::subcommands::{ParsedSubcommand, get_flag_value, has_flag, get_flag_values};
use crate::templates::TemplateRegistry;
use crate::list::find_files;
use crate::make::handle_make_command;
use crate::rename::handle_rename_command;
use crate::template::handle_template_command;
use crate::validate::handle_validate_command;
use crate::undo::{handle_undo_check, handle_undo_apply};
use crate::audit::handle_audit_command;
use std::path::PathBuf;

/// Configuration extracted from subcommands
#[derive(Debug, Default)]
pub struct CommandConfig {
    pub list_patterns: Option<Vec<String>>,
    pub list_recursive: bool,
    pub list_exclude: Vec<String>,
    pub list_fullpath: bool,
    pub list_json: bool,
    pub make_pattern: Option<String>,
    pub make_json: bool,
    pub template_use: Option<String>,
    pub validate_skip_invalid: bool,
    pub rename_overwrite: bool,
    pub rename_yes: bool,
    pub rename_interactive: bool,
    pub rename_json: bool,
}

/// Handles standalone commands that must be used alone (undo, audit, template --list)
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
            return Err("'undo' cannot be used with other subcommands.\nThe 'undo' subcommand is very important and must be used alone.\n\nExamples:\n  fren undo --check\n  fren undo --apply\n  fren undo --apply --yes".to_string());
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
            return Err("'audit' cannot be used with other subcommands.\nThe 'audit' subcommand is standalone and must be used alone.\n\nExamples:\n  fren audit\n  fren audit --limit 10\n  fren audit --json".to_string());
        }
        
        let audit_subcmd = subcommands.iter().find(|s| s.name == "audit").unwrap();
        let limit_str = get_flag_value(&audit_subcmd.flags, "limit");
        let limit = limit_str.and_then(|s| s.parse::<usize>().ok());
        let json = has_flag(&audit_subcmd.flags, "json");
        
        handle_audit_command(limit, json).await
            .map_err(|e| format!("Error: {}", e))?;
        return Ok(Some(()));
    }
    
    Ok(None)
}

/// Validates subcommand combinations
pub fn validate_subcommand_combinations(subcommands: &[ParsedSubcommand]) -> Result<(), String> {
    let has_make = subcommands.iter().any(|s| s.name == "make");
    let has_template_use = subcommands.iter().any(|s| {
        s.name == "template" && has_flag(&s.flags, "use")
    });
    
    if has_make && has_template_use {
        return Err("Cannot use both 'make' and 'template --use' in the same command.\nUse either:\n  - 'make <PATTERN>' to specify a pattern directly\n  - 'template --use <NAME|NUMBER>' to use a template pattern".to_string());
    }
    
    Ok(())
}

/// Extracts configuration from parsed subcommands
pub fn extract_config(subcommands: &[ParsedSubcommand]) -> Result<CommandConfig, String> {
    let mut config = CommandConfig::default();
    
    for subcmd in subcommands {
        match subcmd.name.as_str() {
            "list" => {
                let patterns = subcmd.args.clone();
                if patterns.is_empty() {
                    return Err("No search pattern provided for 'list'.".to_string());
                }
                config.list_patterns = Some(patterns);
                config.list_recursive = has_flag(&subcmd.flags, "recursive");
                config.list_exclude = get_flag_values(&subcmd.flags, "exclude");
                config.list_fullpath = has_flag(&subcmd.flags, "fullpath");
                config.list_json = has_flag(&subcmd.flags, "json");
            }
            "make" => {
                let pattern = subcmd.args.first().cloned().unwrap_or_default();
                if pattern.is_empty() {
                    return Err("Make pattern required.".to_string());
                }
                config.make_pattern = Some(pattern);
                config.make_json = has_flag(&subcmd.flags, "json");
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
            "rename" => {
                config.rename_overwrite = has_flag(&subcmd.flags, "overwrite");
                config.rename_yes = has_flag(&subcmd.flags, "yes");
                config.rename_interactive = has_flag(&subcmd.flags, "interactive");
                config.rename_json = has_flag(&subcmd.flags, "json");
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

/// Gets the audit pattern from make pattern or template
pub fn get_audit_pattern(
    make_pattern: &Option<String>,
    template_use: &Option<String>,
    template_registry: &TemplateRegistry,
) -> Option<String> {
    make_pattern.clone().or_else(|| {
        template_use.as_ref().and_then(|name| {
            resolve_template_pattern(template_registry, name).ok()
        })
    })
}

/// Executes the command pipeline: list -> make -> validate -> rename
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
    
    if let Some(patterns) = config.list_patterns {
        files = find_files(&patterns, config.list_recursive, &config.list_exclude).await
            .map_err(|e| format!("Error finding files: {}", e))?;
        
        // Display files if make/template --use/validate/rename is not present
        if config.make_pattern.is_none() && config.template_use.is_none() 
            && !subcommands.iter().any(|s| s.name == "validate")
            && !subcommands.iter().any(|s| s.name == "rename") {
            if config.list_json {
                crate::list::display_files_json(&files, config.list_fullpath);
            } else {
                crate::list::display_files(&files, config.list_fullpath);
            }
        }
    }
    
    // Step 2: Execute make or template --use to generate preview (if present)
    if let Some(pattern) = config.make_pattern.clone() {
        if files.is_empty() {
            return Err("No files to process. 'list' subcommand is required to select files.".to_string());
        }
        
        preview_result = Some(handle_make_command(&engine, files.clone(), pattern, config.make_json).await
            .map_err(|e| format!("Error: {}", e))?);
    } else if let Some(template_name) = config.template_use.clone() {
        if files.is_empty() {
            return Err("'template --use' requires 'list' subcommand to select files.".to_string());
        }
        
        let pattern = resolve_template_pattern(template_registry, &template_name)?;
        preview_result = Some(handle_make_command(&engine, files.clone(), pattern, config.make_json).await
            .map_err(|e| format!("Error: {}", e))?);
    }
    
    // Step 3: Execute validate (if present)
    if subcommands.iter().any(|s| s.name == "validate") {
        let result = preview_result.as_ref()
            .ok_or("No preview available. 'make' or 'template --use' subcommand is required to generate preview.")?;
        handle_validate_command(&engine, result, config.validate_skip_invalid).await;
    }
    
    // Step 4: Execute rename (if present)
    if subcommands.iter().any(|s| s.name == "rename") {
        let result = preview_result.take()
            .ok_or("No preview available. 'make' or 'template --use' subcommand is required to generate preview.")?;
        
        let enable_audit = !subcommands.iter()
            .any(|s| s.name == "rename" && has_flag(&s.flags, "no-audit"));
        
        let audit_pattern = get_audit_pattern(
            &config.make_pattern,
            &config.template_use,
            template_registry,
        );
        
        handle_rename_command(
            result, 
            config.rename_overwrite, 
            config.rename_yes, 
            config.rename_interactive,
            format!("fren {}", full_command),
            audit_pattern,
            enable_audit,
            config.rename_json,
        ).await
            .map_err(|e| format!("Error: {}", e))?;
    }
    
    Ok(())
}

