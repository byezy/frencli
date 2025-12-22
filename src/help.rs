//! Help text generation for help-probe compatibility.
//! 
//! All help output follows the help-probe specification for optimal parsing.

/// Print main help message
pub fn print_main_help() {
    println!("Batch file renamer with pattern matching");
    println!();
    println!("Usage: fren [OPTIONS] <SUBCOMMAND>...");
    println!();
    println!("SUBCOMMANDS:");
    println!("    list        List files matching patterns");
    println!("    make        Make file names using a pattern (preview)");
    println!("    validate    Validate a rename pattern");
    println!("    rename      Rename files (applies immediately)");
    println!("    template    Manage templates");
    println!("    undo        Undo operations");
    println!("    audit       View audit log");
    println!("    interactive Apply rename interactively");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help          Print help");
    println!("    -V, --version       Print version");
    println!();
    println!("Examples:");
    println!("  fren list *.txt");
    println!("  fren list *.txt make \"%N_backup.%E\"");
    println!("  fren list *.txt make \"%N_backup.%E\" rename --yes");
}

/// Print help for a specific subcommand
pub fn print_subcommand_help(subcommand: &str) {
    match subcommand {
        "list" => print_list_help(),
        "rename" => print_rename_help(),
        "validate" => print_validate_help(),
        "make" => print_make_help(),
        "template" => print_template_help(),
        "undo" => print_undo_help(),
        "audit" => print_audit_help(),
        "interactive" => print_interactive_help(),
        _ => {
            eprintln!("Unknown subcommand: {}", subcommand);
            print_main_help();
        }
    }
}

fn print_list_help() {
    println!("List files matching patterns");
    println!();
    println!("Usage: fren list [OPTIONS] <PATTERN>...");
    println!();
    println!("Arguments:");
    println!("    <PATTERN>...    Search patterns (glob patterns, e.g., \"*.txt\")");
    println!();
    println!("Options:");
    println!("    --recursive              Recursively search subdirectories (supports ** glob pattern)");
    println!("    --exclude <EXCLUDE>...    Exclude files matching these patterns");
    println!("    --fullpath                Display full paths instead of just filenames");
    println!("    --json                    Output as JSON array");
    println!("    --rename <RENAME_PATTERN>  Chain to rename command with this pattern");
    println!("    --overwrite               Overwrite existing files (when using --rename)");
    println!("    --yes                     Skip confirmation prompt (when using --rename)");
    println!("    -h, --help                Print help");
}

fn print_rename_help() {
    println!("Directly rename files (applies immediately)");
    println!();
    println!("Operates on files from the last `list` command.");
    println!("Run `fren list` first to select files, then use `fren rename` to rename them.");
    println!();
    println!("Usage: fren rename [OPTIONS] <RENAME_PATTERN>");
    println!();
    println!("Arguments:");
    println!("    <RENAME_PATTERN>    Rename pattern/template (e.g., \"%N.%E\", \"%N2-7.%E\")");
    println!();
    println!("Options:");
    println!("    --overwrite    Overwrite existing files");
    println!("    --yes          Skip confirmation prompt");
    println!("    --json         Output as JSON");
    println!("    -h, --help     Print help");
}

fn print_validate_help() {
    println!("Validate a rename pattern");
    println!();
    println!("Usage: fren validate [OPTIONS] <PATTERN>...");
    println!();
    println!("Arguments:");
    println!("    <PATTERN>...    Search patterns (glob patterns, e.g., \"*.txt\")");
    println!();
    println!("Options:");
    println!("    -r, --recursive              Recursively search subdirectories (supports ** glob pattern)");
    println!("    -e, --exclude <EXCLUDE>...    Exclude files matching these patterns");
    println!("    --skip-invalid                Skip invalid files instead of aborting");
    println!("    --change <TEMPLATE>            Renaming template");
    println!("    --template <TEMPLATE_NAME>    Use a preset template pattern");
    println!("    -h, --help                     Print help");
}

fn print_make_help() {
    println!("Make file names using a pattern");
    println!();
    println!("Generates a preview of file names using a pattern without applying the rename.");
    println!("Operates on files from the last `list` command.");
    println!();
    println!("Usage: fren make [OPTIONS] <RENAME_PATTERN>");
    println!();
    println!("Arguments:");
    println!("    <RENAME_PATTERN>    Pattern to generate new file names");
    println!();
    println!("Options:");
    println!("    --json         Output as JSON");
    println!("    -h, --help     Print help");
}

fn print_template_help() {
    println!("Manage templates");
    println!();
    println!("Usage: fren template [OPTIONS]");
    println!();
    println!("Options:");
    println!("    --list        List available templates");
    println!("    --use <NAME>   Use a template pattern");
    println!("    -h, --help    Print help");
    println!();
    println!("Examples:");
    println!("    fren template --list");
    println!("    fren list *.txt template --use photo-date");
}

fn print_undo_help() {
    println!("Undo operations");
    println!();
    println!("Usage: fren undo [OPTIONS]");
    println!();
    println!("Options:");
    println!("    --check    Check undo status");
    println!("    --apply    Apply undo");
    println!("    --yes      Skip confirmation prompt (when using --apply)");
    println!("    -h, --help  Print help");
    println!();
    println!("Examples:");
    println!("    fren undo --check");
    println!("    fren undo --apply");
    println!("    fren undo --apply --yes");
}

fn print_audit_help() {
    println!("View audit log");
    println!();
    println!("View audit log of rename operations.");
    println!();
    println!("Usage: fren audit [OPTIONS]");
    println!();
    println!("Options:");
    println!("    --limit <N>    Limit number of entries to show");
    println!("    --json         Output in JSON format");
    println!("    -h, --help     Print help");
    println!();
    println!("Examples:");
    println!("    fren audit");
    println!("    fren audit --limit 10");
    println!("    fren audit --json");
}

fn print_interactive_help() {
    println!("Apply rename interactively");
    println!();
    println!("Usage: fren interactive [OPTIONS]");
    println!();
    println!("Note: This subcommand is typically used with `list` and `make`:");
    println!("    fren list <PATTERN>... [OPTIONS] make <RENAME_PATTERN> interactive");
    println!();
    println!("Arguments:");
    println!("    <PATTERN>...        Search patterns (glob patterns, e.g., \"*.txt\")");
    println!("    <RENAME_PATTERN>   Rename pattern/template");
    println!();
    println!("Options:");
    println!("    --recursive              Recursively search subdirectories");
    println!("    --exclude <EXCLUDE>...    Exclude files matching these patterns");
    println!("    --overwrite               Overwrite existing files");
    println!("    -h, --help                Print help");
}

