//! Template subcommand for listing and using template patterns.
//! 
//! This module handles the `fren template` command which can list available
//! templates or output a template pattern for use in transform operations.

use crate::templates::TemplateRegistry;

/// Handles the template subcommand.
/// 
/// # Arguments
/// 
/// * `template_registry` - The template registry
/// * `list` - If true, list all available templates
/// * `use_template` - If Some, output the pattern for the specified template
/// 
/// # Returns
/// 
/// * `Ok(Option<String>)` - If `use_template` is Some, returns the pattern; otherwise None
/// * `Err(String)` - If template not found
pub fn handle_template_command(
    template_registry: &TemplateRegistry,
    list: bool,
    use_template: Option<String>,
) -> Result<Option<String>, String> {
    if list {
        // List all templates
        let templates = template_registry.list();
        println!("Available template patterns:\n");
        for (i, (name, pattern)) in templates.iter().enumerate() {
            println!("  {:2}. {:<25} -> {}", i + 1, name, pattern);
        }
        Ok(None)
    } else if let Some(template_name) = use_template {
        // Use a specific template
        // Check if it's a number (index)
        if let Ok(index) = template_name.parse::<usize>() {
            let templates = template_registry.list();
            if index == 0 || index > templates.len() {
                return Err(format!("Template index {} out of range (1-{})", index, templates.len()));
            }
            let (_, pattern) = templates[index - 1];
            println!("{}", pattern);
            Ok(Some(pattern.clone()))
        } else {
            // It's a template name
            match template_registry.get(&template_name) {
                Some(pattern) => {
                    println!("{}", pattern);
                    Ok(Some(pattern.clone()))
                }
                None => {
                    Err(format!("Unknown template '{}'. Use 'template --list' to see all available templates.", template_name))
                }
            }
        }
    } else {
        // No action specified
        Err("Template command requires either --list or --use <NAME|NUMBER>".to_string())
    }
}

