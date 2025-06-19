# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-06-19

### Added
- Initial release of askr
- Interactive CLI input tool with real-time validation
- Comprehensive validation system with priority levels
- Interactive choice menus with arrow key navigation
- Support for single and multiple choice selection
- Real-time validation feedback while typing
- Advanced validation types:
  - Basic validation (required, length, patterns)
  - Format validation (email, hostname, URL, IPv4, IPv6)
  - Numeric validation (integer, float, range, positive/negative)
  - Date/time validation with custom formats
  - File system validation (existence, permissions)
- Multiple output formats (default, JSON, raw)
- Shell completion support for bash, zsh, fish, PowerShell
- Comprehensive CLI options and configuration
- Both library and standalone CLI tool
- Cross-platform terminal support
- Line editing capabilities with keyboard shortcuts
- Smart prompt placement for script integration
- Proper error handling and exit codes
- Unicode support for international text
- Password masking support
- Timeout and retry mechanisms

### Features
- **Interactive Choice Menus**: Navigate with ↑↓ arrows, select with Enter/Spacebar
- **Real-time Validation**: Instant feedback as you type with partial validation
- **Advanced Validation Engine**: Priority-based validation with comprehensive rules
- **Shell Integration**: Complete shell completion for better CLI experience
- **Multiple Output Modes**: Default, JSON, and raw output for different use cases
- **Library Support**: Use as both standalone CLI and Rust library
- **Cross-platform**: Works on Linux, macOS, and Windows

[Unreleased]: https://github.com/gfranxman/askr/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/gfranxman/askr/releases/tag/v0.1.0