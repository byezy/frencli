# Changelog

All notable changes to `frencli` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2024-12-20

### Added
- Help output now fully compatible with `help-probe` for improved CLI tool discovery
- Short flags (`-h`, `-V`) added to help output (long forms still supported)
- Comprehensive help text for all subcommands (transform, template, undo, audit)

### Changed
- Standardized help output format following help-probe specification:
  - Usage lines now include `[OPTIONS]` before arguments
  - Subcommand names in main help no longer include arguments (cleaner descriptions)
  - Consistent 4-space indentation throughout
  - Section headers standardized (`SUBCOMMANDS:`, `OPTIONS:`)
- Refactored help text into dedicated `help.rs` module for better code organization
- Improved help text for transform, template, undo, and audit subcommands

### Fixed
- Subcommand descriptions now extract correctly (no longer include argument text)
- Removed duplicate subcommand entries (template and undo now appear once each)
- Fixed false "[OPTIONS]" argument extraction in help parsing

## [0.1.1] - 2024-12-20

### Fixed
- Removed dead code (`handle_validate`, `handle_rename`, `handle_interactive`, `confirm_renames`)
- Improved test isolation with shared test utilities (`test_utils.rs`)
- All tests now use sandboxed temporary directories without mutex serialization

### Changed
- Updated to use published `freneng` crate (v0.1.1) instead of path dependency
- Refactored test infrastructure to eliminate code duplication
- Tests now use `DirGuard` RAII pattern for directory changes where necessary

### Removed
- Removed unused imports and dead code warnings

## [0.1.0] - 2024-12-20

### Added
- Initial release of `frencli` as a standalone crate
- Command-line interface with preview and apply modes
- Subcommand-based CLI structure (`list`, `validate`, `transform`, `rename`, `template`, `undo`, `audit`)
- Multi-subcommand support - subcommands can be specified in any order
- Custom argument parser (no external CLI dependencies)
- Support for all `freneng` placeholders and modifiers
- Recursive directory support with `--recursive` flag
- Pattern templates with preset patterns (e.g., `photo-date`, `lowercase`, `counter-3`)
- Interactive mode for individual filename editing
- Exclude patterns support (`--exclude` with multiple patterns)
- Overwrite protection (`--overwrite` flag, rename subcommand only)
- Undo functionality with conflict detection
- Audit logging support
- Skip confirmation flag (`--yes`)
- Comprehensive error messages and warnings
- Integration with `freneng` library
- Strict long-flag-only policy (no short flags) for clarity and consistency
- Isolated test infrastructure with temporary sandboxes

[0.1.0]: https://github.com/byezy/frencli/releases/tag/v0.1.0
