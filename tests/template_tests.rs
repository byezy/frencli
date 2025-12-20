//! Tests for the template subcommand module.
//! 
//! These tests verify template listing and retrieval functionality.

use frencli::template::handle_template_command;
use frencli::templates::TemplateRegistry;

#[test]
fn test_handle_template_list() {
    let registry = TemplateRegistry::new();
    
    // Should not panic and should return Ok(None)
    let result = handle_template_command(&registry, true, None);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_handle_template_use_by_name() {
    let registry = TemplateRegistry::new();
    
    let result = handle_template_command(&registry, false, Some("lowercase".to_string()));
    assert!(result.is_ok());
    let pattern = result.unwrap();
    assert_eq!(pattern, Some("%L%N.%E".to_string()));
}

#[test]
fn test_handle_template_use_by_index() {
    let registry = TemplateRegistry::new();
    let templates = registry.list();
    
    // Use first template (index 1)
    let result = handle_template_command(&registry, false, Some("1".to_string()));
    assert!(result.is_ok());
    let pattern = result.unwrap();
    assert_eq!(pattern, Some(templates[0].1.clone()));
}

#[test]
fn test_handle_template_use_invalid_name() {
    let registry = TemplateRegistry::new();
    
    let result = handle_template_command(&registry, false, Some("nonexistent".to_string()));
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("Unknown template"));
    assert!(error.contains("nonexistent"));
}

#[test]
fn test_handle_template_use_invalid_index_zero() {
    let registry = TemplateRegistry::new();
    
    let result = handle_template_command(&registry, false, Some("0".to_string()));
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("out of range"));
}

#[test]
fn test_handle_template_use_invalid_index_too_large() {
    let registry = TemplateRegistry::new();
    let templates = registry.list();
    let invalid_index = templates.len() + 1;
    
    let result = handle_template_command(&registry, false, Some(invalid_index.to_string()));
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("out of range"));
}

#[test]
fn test_handle_template_no_action() {
    let registry = TemplateRegistry::new();
    
    // Neither list nor use specified
    let result = handle_template_command(&registry, false, None);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("requires either --list or --use"));
}

#[test]
fn test_handle_template_use_all_templates() {
    let registry = TemplateRegistry::new();
    let templates = registry.list();
    
    // Test that all templates can be retrieved by name
    for (name, expected_pattern) in templates {
        let result = handle_template_command(&registry, false, Some(name.clone()));
        assert!(result.is_ok(), "Failed to get template: {}", name);
        let pattern = result.unwrap();
        assert_eq!(pattern, Some(expected_pattern.clone()), "Pattern mismatch for template: {}", name);
    }
}

