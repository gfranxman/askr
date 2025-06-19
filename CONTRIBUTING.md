# Contributing to askr

Thank you for your interest in contributing to askr! This document provides guidelines and information for contributors.

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). All contributors are expected to adhere to this code of conduct.

## How to Contribute

### Reporting Bugs

Before submitting a bug report:
- Check if the issue has already been reported
- Try to reproduce the bug with the latest version
- Gather relevant information (OS, terminal, Rust version, etc.)

When submitting a bug report, include:
- A clear description of the problem
- Steps to reproduce the issue
- Expected vs actual behavior
- System information (OS, terminal emulator, etc.)
- Rust and askr version information

### Suggesting Features

Feature requests are welcome! Please:
- Check if the feature has already been requested
- Provide a clear description of the feature
- Explain the use case and benefits
- Consider if it fits with askr's design goals

### Pull Requests

1. **Fork the repository** and create a feature branch
2. **Make your changes** following the coding guidelines below
3. **Add tests** for new functionality
4. **Update documentation** if needed
5. **Run the test suite** to ensure everything works
6. **Submit a pull request** with a clear description

## Development Setup

### Prerequisites

- Rust 1.70.0 or later
- Git

### Getting Started

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/askr.git
cd askr

# Build the project
cargo build

# Run tests
cargo test

# Run clippy for linting
cargo clippy --all-targets --all-features

# Format code
cargo fmt
```

### Project Structure

```
askr/
├── src/
│   ├── cli/           # CLI argument parsing and configuration
│   ├── validation/    # Validation engine and rules
│   ├── ui/           # Terminal UI and interactive components
│   ├── output/       # Output formatting
│   └── error.rs      # Error types and handling
├── tests/            # Integration tests
├── examples/         # Usage examples
└── spec/            # Design specifications
```

## Coding Guidelines

### Style

- Follow standard Rust formatting (use `cargo fmt`)
- Use `cargo clippy` and address all warnings
- Write clear, descriptive variable and function names
- Add documentation comments (`///`) for public APIs

### Code Quality

- **Error Handling**: Use proper error types and don't unwrap in library code
- **Testing**: Add unit tests for new functionality
- **Documentation**: Update docs for API changes
- **Performance**: Consider performance implications of changes

### Commit Messages

Use clear, descriptive commit messages:
- Use the imperative mood ("Add feature" not "Added feature")
- Limit the first line to 72 characters
- Reference issues and pull requests when applicable

Example:
```
Add support for custom date formats

- Implement DateValidator with configurable format strings
- Add --date-format CLI option
- Update documentation and tests

Fixes #123
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out html
```

### Writing Tests

- Add unit tests in the same file as the code being tested
- Add integration tests in the `tests/` directory
- Test both success and failure cases
- Include edge cases and boundary conditions

### Test Guidelines

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_success_case() {
        // Arrange
        let input = "test@example.com";
        let validator = EmailValidator::new();
        
        // Act
        let result = validator.validate(input);
        
        // Assert
        assert!(result.is_valid());
    }
    
    #[test]
    fn test_feature_failure_case() {
        // Test failure scenarios
    }
}
```

## Documentation

### API Documentation

- Add `///` comments for all public functions, structs, and modules
- Include examples in documentation comments
- Document parameters, return values, and potential errors

```rust
/// Validates email addresses according to RFC standards.
/// 
/// # Examples
/// 
/// ```
/// use askr::EmailValidator;
/// 
/// let validator = EmailValidator::new();
/// assert!(validator.validate("user@example.com").is_valid());
/// ```
/// 
/// # Errors
/// 
/// Returns validation error if the input is not a valid email format.
pub fn validate(&self, input: &str) -> ValidationResult {
    // Implementation
}
```

### README Updates

Update the README.md when:
- Adding new features or CLI options
- Changing installation instructions
- Adding new examples

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with new features and fixes
3. Create a git tag: `git tag v0.x.y`
4. Push changes and tags: `git push origin main --tags`
5. Publish to crates.io: `cargo publish`

## Questions?

- Open an issue for questions about contributing
- Join discussions in existing issues and pull requests
- Check the project's documentation and specifications

## Recognition

Contributors will be acknowledged in:
- The project's README
- Release notes
- Git commit history

Thank you for contributing to askr! 🦀