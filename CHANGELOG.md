# Changelog

All notable changes to `frencli` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4] - 2026-01-03

### Changed
- Updated `freneng` dependency from `0.1.1` to `0.1.2` to benefit from bug fixes:
  - Fixed false positive `ParentNotWritable` validation errors
  - Improved write permission checking with unique test filenames

### Fixed
- Updated all tests to work correctly with the new `freneng` 0.1.2 API
- Fixed test command structures to use `rename` (preview) and `apply` (execution) instead of deprecated `make` subcommand

## [0.1.3] - 2025-12-22

### Added
- Comprehensive test coverage for all modules:
  - `executor_tests.rs` with 24 tests covering command execution orchestration
  - `help_tests.rs` with 11 tests covering help output
- Test coverage documentation (`TEST_COVERAGE.md`)

### Changed
- Major code refactoring: extracted command execution logic from `main.rs` into dedicated `executor.rs` module
- `main.rs` reduced from 407 lines to 124 lines (70% reduction) - now focuses solely on entry point and delegation
- Improved code organization and maintainability with clear separation of concerns

### Fixed
- All source modules now have corresponding test files ensuring comprehensive coverage
- Test infrastructure updated to include new test files in test runner script

## [0.1.2] - 2024-12-20

### Added
- Help output now fully compatible with `help-probe` for improved CLI tool discovery
- Short flags (`-h`, `-V`) added to help output (long forms still supported)
- Comprehensive help text for all subcommands (make, template, undo, audit)
- JSON output support for `list`, `make`, and `rename` subcommands via `--json` flag

### Changed
- Standardized help output format following help-probe specification:
  - Usage lines now include `[OPTIONS]` before arguments
  - Subcommand names in main help no longer include arguments (cleaner descriptions)
  - Consistent 4-space indentation throughout
  - Section headers standardized (`SUBCOMMANDS:`, `OPTIONS:`)
- Refactored help text into dedicated `help.rs` module for better code organization
- Improved help text for make, template, undo, and audit subcommands
- Renamed `transform` subcommand to `make` for better clarity and consistency

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
- Subcommand-based CLI structure (`list`, `validate`, `make`, `rename`, `template`, `undo`, `audit`)
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
