//! Tests for the subcommands parsing module.
//! 
//! These tests verify the custom subcommand parser that handles multiple
//! subcommands in a single invocation.

use frencli::subcommands::{parse_multi_subcommand, get_flag_value, has_flag, get_flag_values};
use std::collections::HashMap;

#[test]
fn test_parse_single_subcommand() {
    let args = vec!["list".to_string(), "*.txt".to_string()];
    let result = parse_multi_subcommand(args);
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "list");
    assert_eq!(result[0].args, vec!["*.txt"]);
    assert!(result[0].flags.is_empty());
}

#[test]
fn test_parse_multiple_subcommands() {
    let args = vec![
        "list".to_string(),
        "*.txt".to_string(),
        "transform".to_string(),
        "%N.%E".to_string(),
        "rename".to_string(),
    ];
    let result = parse_multi_subcommand(args);
    
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].name, "list");
    assert_eq!(result[0].args, vec!["*.txt"]);
    assert_eq!(result[1].name, "transform");
    assert_eq!(result[1].args, vec!["%N.%E"]);
    assert_eq!(result[2].name, "rename");
    assert!(result[2].args.is_empty());
}

#[test]
fn test_parse_subcommand_with_flags() {
    let args = vec![
        "list".to_string(),
        "*.txt".to_string(),
        "--recursive".to_string(),
        "--exclude".to_string(),
        "*.tmp".to_string(),
    ];
    let result = parse_multi_subcommand(args);
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "list");
    assert_eq!(result[0].args, vec!["*.txt"]);
    assert!(has_flag(&result[0].flags, "recursive"));
    assert_eq!(get_flag_values(&result[0].flags, "exclude"), vec!["*.tmp"]);
}

#[test]
fn test_parse_subcommand_with_multiple_flag_values() {
    let args = vec![
        "list".to_string(),
        "--exclude".to_string(),
        "*.tmp".to_string(),
        "*.bak".to_string(),
    ];
    let result = parse_multi_subcommand(args);
    
    assert_eq!(result.len(), 1);
    assert_eq!(get_flag_values(&result[0].flags, "exclude"), vec!["*.tmp", "*.bak"]);
}

#[test]
fn test_parse_subcommand_with_boolean_flag() {
    let args = vec![
        "rename".to_string(),
        "--yes".to_string(),
        "--overwrite".to_string(),
    ];
    let result = parse_multi_subcommand(args);
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "rename");
    assert!(has_flag(&result[0].flags, "yes"));
    assert!(has_flag(&result[0].flags, "overwrite"));
}

#[test]
fn test_parse_complex_command() {
    let args = vec![
        "list".to_string(),
        "*.txt".to_string(),
        "*.jpg".to_string(),
        "--recursive".to_string(),
        "--exclude".to_string(),
        "*.tmp".to_string(),
        "transform".to_string(),
        "%N_backup.%E".to_string(),
        "validate".to_string(),
        "--skip-invalid".to_string(),
        "rename".to_string(),
        "--yes".to_string(),
    ];
    let result = parse_multi_subcommand(args);
    
    assert_eq!(result.len(), 4);
    
    // Check list
    assert_eq!(result[0].name, "list");
    assert_eq!(result[0].args, vec!["*.txt", "*.jpg"]);
    assert!(has_flag(&result[0].flags, "recursive"));
    assert_eq!(get_flag_values(&result[0].flags, "exclude"), vec!["*.tmp"]);
    
    // Check transform
    assert_eq!(result[1].name, "transform");
    assert_eq!(result[1].args, vec!["%N_backup.%E"]);
    
    // Check validate
    assert_eq!(result[2].name, "validate");
    assert!(has_flag(&result[2].flags, "skip-invalid"));
    
    // Check rename
    assert_eq!(result[3].name, "rename");
    assert!(has_flag(&result[3].flags, "yes"));
}

#[test]
fn test_parse_unknown_args_ignored() {
    let args = vec![
        "unknown".to_string(),
        "list".to_string(),
        "*.txt".to_string(),
        "also-unknown".to_string(),
    ];
    let result = parse_multi_subcommand(args);
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "list");
    // Unknown args before subcommands are ignored, but args after subcommands are collected
    assert_eq!(result[0].args, vec!["*.txt", "also-unknown"]);
}

#[test]
fn test_parse_empty_args() {
    let args = vec![];
    let result = parse_multi_subcommand(args);
    
    assert_eq!(result.len(), 0);
}

#[test]
fn test_parse_template_with_use_flag() {
    let args = vec![
        "template".to_string(),
        "--use".to_string(),
        "lowercase".to_string(),
    ];
    let result = parse_multi_subcommand(args);
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "template");
    assert_eq!(get_flag_value(&result[0].flags, "use"), Some("lowercase".to_string()));
}

#[test]
fn test_parse_undo_with_check() {
    let args = vec![
        "undo".to_string(),
        "--check".to_string(),
    ];
    let result = parse_multi_subcommand(args);
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "undo");
    assert!(has_flag(&result[0].flags, "check"));
}

#[test]
fn test_parse_undo_with_apply() {
    let args = vec![
        "undo".to_string(),
        "--apply".to_string(),
        "--yes".to_string(),
    ];
    let result = parse_multi_subcommand(args);
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "undo");
    assert!(has_flag(&result[0].flags, "apply"));
    assert!(has_flag(&result[0].flags, "yes"));
}

#[test]
fn test_get_flag_value() {
    let mut flags = HashMap::new();
    flags.insert("exclude".to_string(), vec!["*.tmp".to_string(), "*.bak".to_string()]);
    
    assert_eq!(get_flag_value(&flags, "exclude"), Some("*.tmp".to_string()));
    assert_eq!(get_flag_value(&flags, "nonexistent"), None);
}

#[test]
fn test_has_flag() {
    let mut flags = HashMap::new();
    flags.insert("recursive".to_string(), vec![]);
    
    assert!(has_flag(&flags, "recursive"));
    assert!(!has_flag(&flags, "nonexistent"));
}

#[test]
fn test_get_flag_values() {
    let mut flags = HashMap::new();
    flags.insert("exclude".to_string(), vec!["*.tmp".to_string(), "*.bak".to_string()]);
    
    assert_eq!(get_flag_values(&flags, "exclude"), vec!["*.tmp", "*.bak"]);
    assert_eq!(get_flag_values(&flags, "nonexistent"), Vec::<String>::new());
}

#[test]
fn test_parse_subcommand_order_independence() {
    // Test that order doesn't matter for parsing
    let args1 = vec![
        "list".to_string(),
        "*.txt".to_string(),
        "transform".to_string(),
        "%N.%E".to_string(),
    ];
    let args2 = vec![
        "transform".to_string(),
        "%N.%E".to_string(),
        "list".to_string(),
        "*.txt".to_string(),
    ];
    
    let result1 = parse_multi_subcommand(args1);
    let result2 = parse_multi_subcommand(args2);
    
    // Both should parse correctly, order is preserved in result
    assert_eq!(result1.len(), 2);
    assert_eq!(result2.len(), 2);
    assert_eq!(result1[0].name, "list");
    assert_eq!(result1[1].name, "transform");
    assert_eq!(result2[0].name, "transform");
    assert_eq!(result2[1].name, "list");
}

// ============================================================================
// Tests for short flag handling
// ============================================================================

#[test]
fn test_short_flag_allowed_as_list_positional_arg() {
    // Short flags should be allowed as positional arguments for 'list'
    // because they could be filenames (e.g., a file named "-y")
    let args = vec![
        "list".to_string(),
        "-y".to_string(),
        "-o".to_string(),
    ];
    let result = parse_multi_subcommand(args);
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "list");
    assert_eq!(result[0].args, vec!["-y", "-o"]);
}

#[test]
fn test_short_flag_allowed_as_transform_positional_arg() {
    // Short flags should be allowed as positional arguments for 'transform'
    // because they could be patterns
    let args = vec![
        "transform".to_string(),
        "-y".to_string(),
    ];
    let result = parse_multi_subcommand(args);
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "transform");
    assert_eq!(result[0].args, vec!["-y"]);
}

#[test]
fn test_short_flag_allowed_as_exclude_value() {
    // Short flags should be allowed as values for --exclude
    // because they could be exclude patterns (e.g., exclude files named "-y")
    let args = vec![
        "list".to_string(),
        "*.txt".to_string(),
        "--exclude".to_string(),
        "-y".to_string(),
        "-o".to_string(),
    ];
    let result = parse_multi_subcommand(args);
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "list");
    assert_eq!(get_flag_values(&result[0].flags, "exclude"), vec!["-y", "-o"]);
}

#[test]
fn test_short_flag_allowed_as_template_use_value() {
    // Short flags should be allowed as values for --use
    // because template names could theoretically start with '-'
    let args = vec![
        "template".to_string(),
        "--use".to_string(),
        "-y".to_string(),
    ];
    let result = parse_multi_subcommand(args);
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "template");
    assert_eq!(get_flag_value(&result[0].flags, "use"), Some("-y".to_string()));
}

#[test]
fn test_short_flag_allowed_as_mixed_list_args() {
    // Mix of normal args and short-flag-looking args should all be accepted
    let args = vec![
        "list".to_string(),
        "*.txt".to_string(),
        "-y".to_string(),
        "test.txt".to_string(),
        "-o".to_string(),
    ];
    let result = parse_multi_subcommand(args);
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "list");
    assert_eq!(result[0].args, vec!["*.txt", "-y", "test.txt", "-o"]);
}

