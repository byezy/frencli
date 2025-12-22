//! Tests for the help module.
//! 
//! These tests verify help text output for all subcommands.
//! 
//! Note: Since help functions print directly to stdout, we test that they
//! don't panic and can be called. Full output verification would require
//! stdout capture or integration tests.

use frencli::help::{print_main_help, print_subcommand_help};

#[test]
fn test_print_main_help_no_panic() {
    // Just verify it doesn't panic
    print_main_help();
}

#[test]
fn test_print_subcommand_help_list() {
    print_subcommand_help("list");
}

#[test]
fn test_print_subcommand_help_make() {
    print_subcommand_help("make");
}

#[test]
fn test_print_subcommand_help_rename() {
    print_subcommand_help("rename");
}

#[test]
fn test_print_subcommand_help_validate() {
    print_subcommand_help("validate");
}

#[test]
fn test_print_subcommand_help_template() {
    print_subcommand_help("template");
}

#[test]
fn test_print_subcommand_help_undo() {
    print_subcommand_help("undo");
}

#[test]
fn test_print_subcommand_help_audit() {
    print_subcommand_help("audit");
}

#[test]
fn test_print_subcommand_help_interactive() {
    print_subcommand_help("interactive");
}

#[test]
fn test_print_subcommand_help_unknown() {
    // Should print main help for unknown subcommand
    print_subcommand_help("unknown_command");
}

// Test that help output contains expected keywords
// This is a basic smoke test - full output verification would require stdout capture
#[test]
fn test_help_output_structure() {
    // We can't easily capture stdout in unit tests without external tools
    // But we can verify the functions exist and don't panic
    // Integration tests or binary tests would be better for verifying actual output
    
    // For now, just verify all help functions can be called
    let subcommands = vec![
        "list", "make", "rename", "validate", "template", 
        "undo", "audit", "interactive", "unknown"
    ];
    
    for subcmd in subcommands {
        print_subcommand_help(subcmd);
    }
    
    print_main_help();
}

