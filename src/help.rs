//! Help text generation for help-probe compatibility.
//! 
//! All help output follows the help-probe specification for optimal parsing.

/// Print main help message
pub fn print_main_help() {
    println!("Batch file renamer with pattern matching");
    println!();
    println!("Usage: frencli [OPTIONS] <SUBCOMMAND>...");
    println!();
    println!("SUBCOMMANDS:");
    println!("    list        List files matching patterns");
    println!("    rename      Generate rename preview using a pattern");
    println!("    validate    Validate a rename pattern");
    println!("    apply       Apply rename operations (performs the rename)");
    println!("    template    Manage templates");
    println!("    undo        Undo operations");
    println!("    audit       View audit log");
    println!("    interactive    Interactive workflow guide");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help          Print help");
    println!("    -V, --version       Print version");
    println!();
    println!("Pattern quick reference:");
    println!("  %N.%E               Keep name and extension");
    println!("  backup_%N.%E        Add prefix");
    println!("  %N_%C3.%E           Add zero-padded counter");
    println!("  %N1-3.%E            Use chars 1..3 from name");
    println!("  %L%N.%E             Lowercase result");
    println!("  %N%R/_/-.%E         Replace '_' with '-'");
    println!("  %N%X/[0-9]+/num.%E  Regex-replace digits");
    println!();
    println!("Examples:");
    println!("  frencli list *.txt");
    println!("  frencli list *.txt rename \"%N_backup.%E\"");
    println!("  frencli list *.txt rename \"%N_backup.%E\" apply --yes");
    println!();
    println!("See more pattern examples: frencli rename --help");
}

/// Print help for a specific subcommand
pub fn print_subcommand_help(subcommand: &str) {
    match subcommand {
        "list" => print_list_help(),
        "rename" => print_rename_help(),
        "validate" => print_validate_help(),
        "apply" => print_apply_help(),
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
    println!("Usage: frencli list [OPTIONS] <PATTERN>...");
    println!("   or: frencli list [OPTIONS] --files-from <FILE>");
    println!();
    println!("Arguments:");
    println!("    <PATTERN>...    Search patterns (glob patterns, e.g., \"*.txt\")");
    println!();
    println!("Options:");
    println!("    --files-from <FILE>           Read file paths from FILE (one per line)");
    println!("                                   Use \"-\" to read from stdin");
    println!("    --recursive              Recursively search subdirectories (supports ** glob pattern)");
    println!("    --exclude <EXCLUDE>...    Exclude files matching these patterns");
    println!("    --fullpath                Display full paths instead of just filenames");
    println!("    --json                    Output as JSON array");
    println!("    --apply <RENAME_PATTERN>  Chain to apply command with this pattern");
    println!("    --overwrite               Overwrite existing files (when using --apply)");
    println!("    --yes                     Skip confirmation prompt (when using --apply)");
    println!("    -h, --help                Print help");
}

fn print_apply_help() {
    println!("Apply rename operations (performs the rename)");
    println!();
    println!("Operates on files from the last `list` command.");
    println!("Run `frencli list` first to select files, then use `frencli rename` to generate a preview,");
    println!("and finally `frencli apply` to perform the rename.");
    println!();
    println!("Usage: frencli apply [OPTIONS]");
    println!();
    println!("Options:");
    println!("    --overwrite    Overwrite existing files");
    println!("    --yes          Skip confirmation prompt");
    println!("    --interactive  Interactive mode (edit filenames individually)");
    println!("    --json         Output as JSON");
    println!("    -h, --help     Print help");
}

fn print_validate_help() {
    println!("Validate a rename pattern");
    println!();
    println!("Usage: frencli validate [OPTIONS] <PATTERN>...");
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

fn print_rename_help() {
    println!("Generate rename preview using a pattern");
    println!();
    println!("Generates a preview of file names using a pattern without applying the rename.");
    println!("Operates on files from the last `list` command.");
    println!("Use `frencli apply` to actually perform the rename.");
    println!();
    println!("Usage: frencli rename [OPTIONS] <RENAME_PATTERN>");
    println!();
    println!("Arguments:");
    println!("    <RENAME_PATTERN>    Pattern to generate new file names (e.g., \"%N.%E\", \"%N2-7.%E\")");
    println!();
    println!("Pattern examples:");
    println!("    %N.%E                     Keep name and extension");
    println!("    backup_%N.%E              Add prefix");
    println!("    %N_%C3.%E                 Add zero-padded counter (001, 002, ...)");
    println!("    %F                        Use full original filename");
    println!("    %P_%N.%E                  Prefix with parent directory name");
    println!("    %D_%H_%N.%E               Prefix with current date and time");
    println!("    %FD_%FH_%N.%E             Prefix with file modified date and time");
    println!("    %N1-3.%E                  Use chars 1..3 from filename");
    println!("    %N5-.%E                   Use filename from char 5 to end");
    println!("    %N--3.%E                  Trim last 3 chars from filename");
    println!("    %L%N.%E                   Lowercase result");
    println!("    %U%N.%E                   Uppercase result");
    println!("    %T%N.%E                   Title-case result");
    println!("    \"  %N  %M.%E\"              Trim surrounding spaces");
    println!("    %N%R/_/-.%E               Replace all '_' with '-'");
    println!("    %N%X/[0-9]+/num.%E        Regex replace digits with 'num'");
    println!();
    println!("Options:");
    println!("    --json         Output as JSON");
    println!("    -h, --help     Print help");
}

fn print_template_help() {
    println!("Manage templates");
    println!();
    println!("Usage: frencli template [OPTIONS]");
    println!();
    println!("Options:");
    println!("    --list        List available templates");
    println!("    --use <NAME>   Use a template pattern");
    println!("    -h, --help    Print help");
    println!();
    println!("Examples:");
    println!("    frencli template --list");
    println!("    frencli list *.txt template --use photo-date");
}

fn print_undo_help() {
    println!("Undo operations");
    println!();
    println!("Usage: frencli undo [OPTIONS]");
    println!();
    println!("Options:");
    println!("    --check    Check undo status");
    println!("    --apply    Apply undo");
    println!("    --yes      Skip confirmation prompt (when using --apply)");
    println!("    -h, --help  Print help");
    println!();
    println!("Examples:");
    println!("    frencli undo --check");
    println!("    frencli undo --apply");
    println!("    frencli undo --apply --yes");
}

fn print_audit_help() {
    println!("View audit log");
    println!();
    println!("View audit log of rename operations.");
    println!();
    println!("Usage: frencli audit [OPTIONS]");
    println!();
    println!("Options:");
    println!("    --limit <N>    Limit number of entries to show");
    println!("    --json         Output in JSON format");
    println!("    -h, --help     Print help");
    println!();
    println!("Examples:");
    println!("    frencli audit");
    println!("    frencli audit --limit 10");
    println!("    frencli audit --json");
}

fn print_interactive_help() {
    println!("Interactive workflow guide");
    println!();
    println!("Usage: frencli interactive [OPTIONS]");
    println!();
    println!("Guides you through the standard frencli workflow step by step:");
    println!("  1. Select files to rename");
    println!("  2. Define rename pattern");
    println!("  3. Preview and validate");
    println!("  4. Apply rename");
    println!();
    println!("Options:");
    println!("    -h, --help     Print help");
    println!();
    println!("Examples:");
    println!("    frencli interactive");
}

