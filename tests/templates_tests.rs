//! Tests for the templates registry module.
//! 
//! These tests verify template registration, retrieval, and listing functionality.

use frencli::templates::TemplateRegistry;

#[test]
fn test_template_registry_new() {
    let registry = TemplateRegistry::new();
    
    // Should have templates
    let templates = registry.list();
    assert!(!templates.is_empty());
}

#[test]
fn test_template_registry_get_existing() {
    let registry = TemplateRegistry::new();
    
    assert_eq!(registry.get("lowercase"), Some(&"%L%N.%E".to_string()));
    assert_eq!(registry.get("uppercase"), Some(&"%U%N.%E".to_string()));
    assert_eq!(registry.get("photo-date"), Some(&"%N_%D.%E".to_string()));
}

#[test]
fn test_template_registry_get_nonexistent() {
    let registry = TemplateRegistry::new();
    
    assert_eq!(registry.get("nonexistent"), None);
    assert_eq!(registry.get(""), None);
}

#[test]
fn test_template_registry_list() {
    let registry = TemplateRegistry::new();
    let templates = registry.list();
    
    // Should have multiple templates
    assert!(templates.len() > 10);
    
    // Should be sorted by key
    for i in 1..templates.len() {
        assert!(templates[i-1].0 <= templates[i].0);
    }
}

#[test]
fn test_template_registry_list_contains_expected() {
    let registry = TemplateRegistry::new();
    let templates: std::collections::HashMap<_, _> = registry.list().iter().map(|(k, v)| (*k, *v)).collect();
    
    // Check some expected templates exist
    assert!(templates.contains_key(&"lowercase".to_string()));
    assert!(templates.contains_key(&"uppercase".to_string()));
    assert!(templates.contains_key(&"title-case".to_string()));
    assert!(templates.contains_key(&"photo-date".to_string()));
    assert!(templates.contains_key(&"counter-3".to_string()));
}

#[test]
fn test_template_registry_default() {
    let registry1 = TemplateRegistry::new();
    let registry2 = TemplateRegistry::default();
    
    // Both should have the same templates
    let list1 = registry1.list();
    let list2 = registry2.list();
    
    assert_eq!(list1.len(), list2.len());
    for (name, pattern) in list1 {
        assert_eq!(registry2.get(name), Some(pattern));
    }
}

#[test]
fn test_template_registry_all_templates_valid() {
    let registry = TemplateRegistry::new();
    let templates = registry.list();
    
    // All templates should have non-empty patterns
    for (name, pattern) in templates {
        assert!(!pattern.is_empty(), "Template '{}' has empty pattern", name);
        assert!(!name.is_empty(), "Found template with empty name");
    }
}

#[test]
fn test_template_registry_case_sensitive() {
    let registry = TemplateRegistry::new();
    
    // Template names are case-sensitive
    assert_eq!(registry.get("lowercase"), Some(&"%L%N.%E".to_string()));
    assert_eq!(registry.get("Lowercase"), None);
    assert_eq!(registry.get("LOWERCASE"), None);
}

