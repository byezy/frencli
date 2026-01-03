//! Tests for the executor module.
//! 
//! These tests verify command execution orchestration, including:
//! - Standalone command handling
//! - Subcommand validation
//! - Configuration extraction
//! - Template resolution
//! - Command pipeline execution

use frencli::executor::{
    handle_standalone_commands,
    validate_subcommand_combinations,
    extract_config,
    resolve_template_pattern,
    get_audit_pattern,
};
use frencli::subcommands::ParsedSubcommand;
use freneng::RenamingEngine;
use frencli::templates::TemplateRegistry;
use std::collections::HashMap;

// Helper to create a ParsedSubcommand
fn create_subcommand(name: &str, args: Vec<String>, flags: HashMap<String, Vec<String>>) -> ParsedSubcommand {
    ParsedSubcommand {
        name: name.to_string(),
        args,
        flags,
    }
}

// Helper to create a simple flag map
fn create_flags(flag_name: &str, value: Option<&str>) -> HashMap<String, Vec<String>> {
    let mut flags = HashMap::new();
    if let Some(v) = value {
        flags.insert(flag_name.to_string(), vec![v.to_string()]);
    } else {
        flags.insert(flag_name.to_string(), vec![]);
    }
    flags
}

// ============================================================================
// validate_subcommand_combinations tests
// ============================================================================

#[test]
fn test_validate_subcommand_combinations_valid() {
    let subcommands = vec![
        create_subcommand("list", vec!["*.txt".to_string()], HashMap::new()),
        create_subcommand("rename", vec!["%N.%E".to_string()], HashMap::new()),
    ];
    assert!(validate_subcommand_combinations(&subcommands).is_ok());
}

#[test]
fn test_validate_subcommand_combinations_rename_and_template_use() {
    let mut template_flags = HashMap::new();
    template_flags.insert("use".to_string(), vec!["test".to_string()]);
    
    let subcommands = vec![
        create_subcommand("rename", vec!["%N.%E".to_string()], HashMap::new()),
        create_subcommand("template", vec![], template_flags),
    ];
    let result = validate_subcommand_combinations(&subcommands);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cannot use both 'rename' and 'template --use'"));
}

#[test]
fn test_validate_subcommand_combinations_template_without_use() {
    let subcommands = vec![
        create_subcommand("list", vec!["*.txt".to_string()], HashMap::new()),
        create_subcommand("template", vec![], HashMap::new()),
    ];
    // Template without --use flag should be fine
    assert!(validate_subcommand_combinations(&subcommands).is_ok());
}

// ============================================================================
// extract_config tests
// ============================================================================

#[test]
fn test_extract_config_list() {
    let mut flags = HashMap::new();
    flags.insert("recursive".to_string(), vec![]);
    flags.insert("exclude".to_string(), vec!["*.tmp".to_string()]);
    flags.insert("fullpath".to_string(), vec![]);
    flags.insert("json".to_string(), vec![]);
    
    let subcommands = vec![
        create_subcommand("list", vec!["*.txt".to_string(), "*.jpg".to_string()], flags),
    ];
    
    let config = extract_config(&subcommands).unwrap();
    assert_eq!(config.list_patterns, Some(vec!["*.txt".to_string(), "*.jpg".to_string()]));
    assert!(config.list_recursive);
    assert_eq!(config.list_exclude, vec!["*.tmp".to_string()]);
    assert!(config.list_fullpath);
    assert!(config.list_json);
}

#[test]
fn test_extract_config_list_empty_patterns() {
    let subcommands = vec![
        create_subcommand("list", vec![], HashMap::new()),
    ];
    
    let result = extract_config(&subcommands);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No search pattern provided"));
}

#[test]
fn test_extract_config_list_with_files_from() {
    let mut flags = HashMap::new();
    flags.insert("files-from".to_string(), vec!["/tmp/filelist.txt".to_string()]);
    
    let subcommands = vec![
        create_subcommand("list", vec![], flags),
    ];
    
    let config = extract_config(&subcommands).unwrap();
    assert_eq!(config.list_files_from, Some("/tmp/filelist.txt".to_string()));
    assert_eq!(config.list_patterns, None); // Should be None when --files-from is used
}

#[test]
fn test_extract_config_list_files_from_stdin() {
    let mut flags = HashMap::new();
    flags.insert("files-from".to_string(), vec!["-".to_string()]);
    
    let subcommands = vec![
        create_subcommand("list", vec![], flags),
    ];
    
    let config = extract_config(&subcommands).unwrap();
    assert_eq!(config.list_files_from, Some("-".to_string()));
    assert_eq!(config.list_patterns, None);
}

#[test]
fn test_extract_config_list_files_from_takes_precedence() {
    // If both --files-from and patterns are provided, --files-from should take precedence
    // Patterns should not be set when --files-from is used
    let mut flags = HashMap::new();
    flags.insert("files-from".to_string(), vec!["/tmp/filelist.txt".to_string()]);
    
    let subcommands = vec![
        create_subcommand("list", vec!["*.txt".to_string()], flags),
    ];
    
    let config = extract_config(&subcommands).unwrap();
    assert_eq!(config.list_files_from, Some("/tmp/filelist.txt".to_string()));
    // Patterns should be None when --files-from is used (it takes precedence)
    assert_eq!(config.list_patterns, None);
}

#[test]
fn test_extract_config_make() {
    let mut flags = HashMap::new();
    flags.insert("json".to_string(), vec![]);
    
    let subcommands = vec![
        create_subcommand("rename", vec!["%N_backup.%E".to_string()], flags),
    ];
    
    let config = extract_config(&subcommands).unwrap();
    assert_eq!(config.rename_pattern, Some("%N_backup.%E".to_string()));
    assert!(config.rename_json);
}

#[test]
fn test_extract_config_make_empty_pattern() {
    let subcommands = vec![
        create_subcommand("rename", vec![], HashMap::new()),
    ];
    
    let result = extract_config(&subcommands);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Rename pattern required"));
}

#[test]
fn test_extract_config_template_use() {
    let mut flags = HashMap::new();
    flags.insert("use".to_string(), vec!["photo-date".to_string()]);
    
    let subcommands = vec![
        create_subcommand("template", vec![], flags),
    ];
    
    let config = extract_config(&subcommands).unwrap();
    assert_eq!(config.template_use, Some("photo-date".to_string()));
}

#[test]
fn test_extract_config_validate() {
    let mut flags = HashMap::new();
    flags.insert("skip-invalid".to_string(), vec![]);
    
    let subcommands = vec![
        create_subcommand("validate", vec![], flags),
    ];
    
    let config = extract_config(&subcommands).unwrap();
    assert!(config.validate_skip_invalid);
}

#[test]
fn test_extract_config_apply() {
    let mut flags = HashMap::new();
    flags.insert("overwrite".to_string(), vec![]);
    flags.insert("yes".to_string(), vec![]);
    flags.insert("interactive".to_string(), vec![]);
    flags.insert("json".to_string(), vec![]);
    
    let subcommands = vec![
        create_subcommand("apply", vec![], flags),
    ];
    
    let config = extract_config(&subcommands).unwrap();
    assert!(config.apply_overwrite);
    assert!(config.apply_yes);
    assert!(config.apply_interactive);
    assert!(config.apply_json);
}

#[test]
fn test_extract_config_multiple_subcommands() {
    let mut list_flags = HashMap::new();
    list_flags.insert("recursive".to_string(), vec![]);
    
    let mut make_flags = HashMap::new();
    make_flags.insert("json".to_string(), vec![]);
    
    let mut rename_flags = HashMap::new();
    rename_flags.insert("yes".to_string(), vec![]);
    
    let subcommands = vec![
        create_subcommand("list", vec!["*.txt".to_string()], list_flags),
        create_subcommand("rename", vec!["%N.%E".to_string()], make_flags),
        create_subcommand("apply", vec![], rename_flags),
    ];
    
    let config = extract_config(&subcommands).unwrap();
    assert_eq!(config.list_patterns, Some(vec!["*.txt".to_string()]));
    assert!(config.list_recursive);
    assert_eq!(config.rename_pattern, Some("%N.%E".to_string()));
    assert!(config.rename_json);
    assert!(config.apply_yes);
}

// ============================================================================
// resolve_template_pattern tests
// ============================================================================

#[test]
fn test_resolve_template_pattern_by_name() {
    let registry = TemplateRegistry::new();
    
    // Add a test template if registry supports it, or use existing
    // For now, test with a template that might exist
    let result = resolve_template_pattern(&registry, "photo-date");
    
    // This might succeed or fail depending on what templates exist
    // We just verify the function works
    match result {
        Ok(pattern) => {
            assert!(!pattern.is_empty());
        }
        Err(e) => {
            // Expected if template doesn't exist
            assert!(e.contains("Unknown template") || e.contains("out of range"));
        }
    }
}

#[test]
fn test_resolve_template_pattern_by_index() {
    let registry = TemplateRegistry::new();
    let templates = registry.list();
    
    if templates.is_empty() {
        // No templates, test error case
        let result = resolve_template_pattern(&registry, "1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("out of range"));
    } else {
        // Test with valid index
        let result = resolve_template_pattern(&registry, "1");
        assert!(result.is_ok());
        
        // Test with invalid index (0)
        let result = resolve_template_pattern(&registry, "0");
        assert!(result.is_err());
        
        // Test with out of range index
        let result = resolve_template_pattern(&registry, &(templates.len() + 1).to_string());
        assert!(result.is_err());
    }
}

#[test]
fn test_resolve_template_pattern_invalid_index() {
    let registry = TemplateRegistry::new();
    let result = resolve_template_pattern(&registry, "999");
    assert!(result.is_err());
}

// ============================================================================
// get_audit_pattern tests
// ============================================================================

#[test]
fn test_get_audit_pattern_from_rename_pattern() {
    let registry = TemplateRegistry::new();
    let rename_pattern = Some("%N_backup.%E".to_string());
    let template_use = None;
    
    let result = get_audit_pattern(&rename_pattern, &template_use, &registry);
    assert_eq!(result, Some("%N_backup.%E".to_string()));
}

#[test]
fn test_get_audit_pattern_from_template() {
    let registry = TemplateRegistry::new();
    let rename_pattern = None;
    let templates = registry.list();
    
    if !templates.is_empty() {
        let template_use = Some("1".to_string());
        let result = get_audit_pattern(&rename_pattern, &template_use, &registry);
        // Should resolve to the template pattern
        assert!(result.is_some());
    }
}

#[test]
fn test_get_audit_pattern_none() {
    let registry = TemplateRegistry::new();
    let rename_pattern = None;
    let template_use = None;
    
    let result = get_audit_pattern(&rename_pattern, &template_use, &registry);
    assert_eq!(result, None);
}

#[test]
fn test_get_audit_pattern_prefers_rename_over_template() {
    let registry = TemplateRegistry::new();
    let rename_pattern = Some("%N.%E".to_string());
    let template_use = Some("1".to_string());
    
    let result = get_audit_pattern(&rename_pattern, &template_use, &registry);
    // Should prefer rename_pattern
    assert_eq!(result, Some("%N.%E".to_string()));
}

// ============================================================================
// handle_standalone_commands tests
// ============================================================================

#[tokio::test]
async fn test_handle_standalone_commands_no_standalone() {
    let engine = RenamingEngine;
    let registry = TemplateRegistry::new();
    
    let subcommands = vec![
        create_subcommand("list", vec!["*.txt".to_string()], HashMap::new()),
    ];
    
    let result = handle_standalone_commands(&subcommands, &engine, &registry).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

#[tokio::test]
async fn test_handle_standalone_commands_undo_with_others() {
    let engine = RenamingEngine;
    let registry = TemplateRegistry::new();
    
    let subcommands = vec![
        create_subcommand("undo", vec![], create_flags("check", None)),
        create_subcommand("list", vec!["*.txt".to_string()], HashMap::new()),
    ];
    
    let result = handle_standalone_commands(&subcommands, &engine, &registry).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("cannot be used with other subcommands"));
}

#[tokio::test]
async fn test_handle_standalone_commands_audit_with_others() {
    let engine = RenamingEngine;
    let registry = TemplateRegistry::new();
    
    let subcommands = vec![
        create_subcommand("audit", vec![], HashMap::new()),
        create_subcommand("list", vec!["*.txt".to_string()], HashMap::new()),
    ];
    
    let result = handle_standalone_commands(&subcommands, &engine, &registry).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("cannot be used with other subcommands"));
}

#[tokio::test]
async fn test_handle_standalone_commands_undo_both_flags() {
    let engine = RenamingEngine;
    let registry = TemplateRegistry::new();
    
    let mut flags = HashMap::new();
    flags.insert("check".to_string(), vec![]);
    flags.insert("apply".to_string(), vec![]);
    
    let subcommands = vec![
        create_subcommand("undo", vec![], flags),
    ];
    
    let result = handle_standalone_commands(&subcommands, &engine, &registry).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cannot use both 'undo --check' and 'undo --apply'"));
}

#[tokio::test]
async fn test_handle_standalone_commands_undo_no_flags() {
    let engine = RenamingEngine;
    let registry = TemplateRegistry::new();
    
    let subcommands = vec![
        create_subcommand("undo", vec![], HashMap::new()),
    ];
    
    let result = handle_standalone_commands(&subcommands, &engine, &registry).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("requires either '--check' or '--apply'"));
}

#[tokio::test]
async fn test_handle_standalone_commands_template_list() {
    let engine = RenamingEngine;
    let registry = TemplateRegistry::new();
    
    let subcommands = vec![
        create_subcommand("template", vec![], create_flags("list", None)),
    ];
    
    // template --list should execute and return Ok(Some(()))
    let result = handle_standalone_commands(&subcommands, &engine, &registry).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(()));
}

