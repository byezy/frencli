# Changelog

All notable changes to `frencli` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-19

### Added
- Initial release of `frencli` as a standalone crate
- Command-line interface with preview and apply modes
- Subcommand-based CLI structure (`list`, `validate`, `rename`, `apply`, `template`, `undo`, `audit`)
- Multi-subcommand support - subcommands can be specified in any order
- Support for all `freneng` placeholders and modifiers
- Recursive directory support with `--recursive` flag
- Pattern templates with preset patterns (e.g., `photo-date`, `lowercase`, `counter-3`)
- Interactive mode for individual filename editing
- Exclude patterns support (`-e/--exclude`)
- Overwrite protection (`-o/--overwrite` flag)
- Undo functionality with conflict detection
- Audit logging support
- Skip confirmation flag (`-y/--yes`)
- Comprehensive error messages and warnings
- Integration with `freneng` library

[0.1.0]: https://github.com/byezy/frencli/releases/tag/v0.1.0
