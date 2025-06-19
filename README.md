# askr

[![Crates.io](https://img.shields.io/crates/v/askr.svg)](https://crates.io/crates/askr)
[![Documentation](https://docs.rs/askr/badge.svg)](https://docs.rs/askr)
[![Build Status](https://github.com/gfranxman/askr/workflows/CI/badge.svg)](https://github.com/gfranxman/askr/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

**askr** (pronounced "asker") is a powerful, interactive CLI input tool for Rust that provides real-time validation, interactive choice menus, and a comprehensive validation system. Perfect for building robust command-line interfaces that need user input with validation.

## ✨ Features

- **🎯 Interactive Choice Menus** - Navigate with arrow keys, select with Enter/Spacebar
- **⚡ Real-time Validation** - Instant feedback as you type
- **🔍 Advanced Validation System** - Email, URLs, numbers, dates, files, and custom patterns
- **🎨 Beautiful UI** - Colored output, highlighted selections, and clear error messages
- **📊 Multiple Output Formats** - Default, JSON, and raw output modes
- **🐚 Shell Completion** - Support for bash, zsh, fish, and PowerShell
- **🔧 Highly Configurable** - Validation priorities, custom messages, and flexible options
- **📚 Library + CLI** - Use as a standalone tool or integrate into your Rust projects

## 📦 Installation

### Install from crates.io

```bash
cargo install askr
```

### Install from source

```bash
git clone https://github.com/gfranxman/askr.git
cd askr
cargo install --path .
```

## 🚀 Quick Start

### Basic Usage

```bash
# Simple text input
askr "Enter your name:"

# Required input with validation
askr "Username:" --required --min-length 3 --max-length 20

# Email validation
askr "Email address:" --validate-email --required

# Number with range
askr "Port number:" --integer --range 1024-65535
```

### Interactive Choice Menus

**Single Choice:**
```bash
askr "Environment:" --choices "dev,staging,prod"
```

**Multiple Choices:**
```bash
askr "Select features:" --choices "auth,db,cache,api" --max-choices 3
```

![Choice Menu Demo](https://github.com/gfranxman/askr/raw/main/docs/demo.gif)

## 📖 Detailed Usage

### Validation Options

#### Basic Validation
- `--required` - Input cannot be empty
- `--min-length <N>` - Minimum character length
- `--max-length <N>` - Maximum character length
- `--pattern <REGEX>` - Custom regex pattern

#### Built-in Validators
- `--validate-email` - Email address validation
- `--validate-hostname` - Hostname/domain validation
- `--validate-url` - URL validation
- `--validate-ipv4` - IPv4 address validation
- `--validate-ipv6` - IPv6 address validation

#### Number Validation
- `--integer` - Accept only integers
- `--float` - Accept only floating-point numbers
- `--range <MIN>-<MAX>` - Numeric range (e.g., `--range 1-100`)
- `--positive` - Only positive numbers
- `--negative` - Only negative numbers

#### Date/Time Validation
- `--date` - Date input (default format: YYYY-MM-DD)
- `--time` - Time input (default format: HH:MM:SS)
- `--datetime` - DateTime input
- `--date-format <FORMAT>` - Custom date format
- `--time-format <FORMAT>` - Custom time format

#### Choice Validation
- `--choices <LIST>` - Comma-separated list of valid choices
- `--min-choices <N>` - Minimum selections required (default: 1)
- `--max-choices <N>` - Maximum selections allowed (default: 1)
- `--choices-case-sensitive` - Case-sensitive choice matching

#### File System Validation
- `--file-exists` - File must exist
- `--dir-exists` - Directory must exist
- `--path-exists` - File or directory must exist
- `--readable` - Path must be readable
- `--writable` - Path must be writable
- `--executable` - File must be executable

### Output Options

#### Output Formats
- `--output default` - Standard output with exit codes
- `--output json` - JSON object with validation metadata
- `--output raw` - Raw user input without processing

#### Display Control
- `--no-color` - Disable colored output
- `--quiet` - Non-interactive mode, read from stdin
- `--verbose` - Show detailed validation messages

#### Interaction Control
- `--timeout <SECONDS>` - Input timeout
- `--max-attempts <N>` - Maximum validation attempts
- `--default <VALUE>` - Default value if user presses Enter
- `--mask` - Mask input (for passwords)

## 💻 Examples

### Complex Validation

```bash
# Strong password requirements
askr "Password:" \
    --required \
    --min-length 12 \
    --pattern ".*[A-Z].*" --pattern-message "Must contain uppercase letter" \
    --pattern ".*[a-z].*" --pattern-message "Must contain lowercase letter" \
    --pattern ".*[0-9].*" --pattern-message "Must contain number" \
    --pattern ".*[!@#$%^&*].*" --pattern-message "Must contain special character" \
    --mask

# Multi-constraint username
askr "Username:" \
    --required \
    --min-length 3 --max-length 20 \
    --pattern "^[a-zA-Z0-9_]+$" --pattern-message "Only letters, numbers, and underscores" \
    --pattern "^[a-zA-Z].*" --pattern-message "Must start with a letter"
```

### Choice Selection

```bash
# Single choice with arrow key navigation
env=$(askr "Environment:" --choices "dev,staging,prod")

# Multiple features selection
features=$(askr "Select features:" \
    --choices "auth,db,cache,api,logs" \
    --min-choices 2 \
    --max-choices 4)

# Optional selection (0 or 1 choice)
optional=$(askr "Optional feature:" \
    --choices "ssl,cache,logs" \
    --min-choices 0)
```

### JSON Output for Scripting

```bash
# Get structured output
result=$(askr "Email:" --validate-email --output json)
email=$(echo "$result" | jq -r '.value')
valid=$(echo "$result" | jq -r '.valid')

if [ "$valid" = "true" ]; then
    echo "Valid email: $email"
fi
```

### Non-interactive Usage

```bash
# Validate from stdin
echo "test@example.com" | askr --validate-email --quiet

# Batch processing
cat emails.txt | while read email; do
    if echo "$email" | askr --validate-email --quiet; then
        echo "Valid: $email"
    else
        echo "Invalid: $email" >&2
    fi
done
```

## 🐚 Shell Completion

Enable shell completion for better CLI experience:

### Bash
```bash
askr completion bash > ~/.bash_completion.d/askr
source ~/.bash_completion.d/askr
```

### Zsh
```bash
mkdir -p ~/.zsh/completions
askr completion zsh > ~/.zsh/completions/_askr
# Add to .zshrc: fpath=(~/.zsh/completions $fpath)
```

### Fish
```bash
askr completion fish > ~/.config/fish/completions/askr.fish
```

### PowerShell
```powershell
askr completion powershell | Out-String | Invoke-Expression
```

## 🛠️ Library Usage

Use askr as a library in your Rust projects:

```toml
[dependencies]
askr = "0.1"
```

```rust
use askr::{ValidationEngine, ValidatorType, InteractivePrompt, Terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = ValidationEngine::new();
    engine.add_validator(Box::new(RequiredValidator::new()));
    engine.add_validator(Box::new(EmailValidator::new()));
    
    let terminal = Terminal::new()?;
    let mut prompt = InteractivePrompt::new(terminal, engine, config)?;
    
    let email = prompt.prompt()?;
    println!("Email: {}", email);
    
    Ok(())
}
```

## 📊 Exit Codes

- `0` - Success (valid input received)
- `1` - Validation failed or invalid input
- `2` - Invalid command-line arguments
- `3` - Maximum attempts exceeded
- `130` - User interrupted (Ctrl+C)
- `124` - Timeout exceeded

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development

```bash
git clone https://github.com/gfranxman/askr.git
cd askr
cargo build
cargo test
```

## 📄 License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Acknowledgments

- Inspired by [Inquirer.js](https://github.com/SBoudrias/Inquirer.js/) for interactive prompts
- Built with [crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal control
- Uses [clap](https://github.com/clap-rs/clap) for CLI argument parsing