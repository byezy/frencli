# Changelog

All notable changes to `frencli` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
