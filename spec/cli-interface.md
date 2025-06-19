# CLI Interface Specification

## Basic Usage

```bash
prompt [OPTIONS] [PROMPT_TEXT]
prompt completion <SHELL>  # Generate shell completion scripts
```

## Arguments

### Positional Arguments

- `PROMPT_TEXT` (optional): The text to display as the prompt
  - If omitted, reads from stdin in quiet mode
  - Examples: "Enter your name:", "Select option:"

## Options

### Output Control

- `--output <FORMAT>`: Output format
  - `default` (default): Value to stdout + exit codes
  - `json`: JSON object with validation metadata
  - `raw`: Raw user input without processing
- `--quiet`: Non-interactive mode, read from stdin
- `--verbose`: Show detailed validation messages to stderr

### Basic Validation

- `--required`: Input cannot be empty
- `--max-length <N>`: Maximum character length
- `--min-length <N>`: Minimum character length
- `--pattern <REGEX>`: Custom regex pattern (can be used multiple times)
- `--pattern-message <TEXT>`: Custom error message for pattern validation (applies to most recent --pattern)

### Built-in Validators

- `--validate-email`: Email address validation
- `--validate-hostname`: Hostname/domain validation
- `--validate-url`: URL validation
- `--validate-ipv4`: IPv4 address validation
- `--validate-ipv6`: IPv6 address validation

### Number Validation

- `--number`: Accept only numeric input
- `--integer`: Accept only integer input
- `--float`: Accept only floating-point input
- `--range <MIN>-<MAX>`: Numeric range (e.g., `--range 1-100`)
- `--positive`: Only positive numbers
- `--negative`: Only negative numbers

### Date/Time Validation

- `--date`: Accept date input
- `--date-format <FORMAT>`: Expected date format (default: %Y-%m-%d)
- `--time`: Accept time input
- `--time-format <FORMAT>`: Expected time format (default: %H:%M:%S)
- `--datetime`: Accept datetime input
- `--datetime-format <FORMAT>`: Expected datetime format

### Choice Validation

- `--choices <LIST>`: Comma or newline-separated list of valid choices
- `--choice-separator <SEP>`: Custom separator for parsing choices (default: auto-detect comma/newline)
- `--selection-separator <SEP>`: Custom separator for joining multiple selections in output (default: comma)
- `--choices-case-sensitive`: Make choice matching case-sensitive
- `--min-choices <N>`: Minimum number of choices required (default: 1)
- `--max-choices <N>`: Maximum number of choices allowed (default: 1, or total choices if min_choices is specified)

### File System Validation

- `--file-exists`: File must exist
- `--dir-exists`: Directory must exist
- `--path-exists`: File or directory must exist
- `--readable`: Path must be readable
- `--writable`: Path must be writable
- `--executable`: File must be executable

### Priority Control

- `--required-priority <LEVEL>`: Priority for required validation (default: critical)
- `--length-priority <LEVEL>`: Priority for length validation (default: medium)
- `--pattern-priority <LEVEL>`: Priority for pattern validation (default: high)
- `--format-priority <LEVEL>`: Priority for format validation (default: high)

Priority levels: `critical`, `high`, `medium`, `low`

### Interaction Control

- `--max-attempts <N>`: Maximum validation attempts (default: unlimited)
- `--timeout <SECONDS>`: Input timeout
- `--default <VALUE>`: Default value if user presses Enter
- `--mask`: Mask input (for passwords)
- `--confirm`: Require confirmation input

### Display Control

- `--no-color`: Disable colored output
- `--width <N>`: Maximum display width
- `--help-text <TEXT>`: Additional help text displayed below prompt

## Usage Examples

### Basic Usage
```bash
# Simple text input
name=$(prompt "Enter your name:")

# Required input with length limits
username=$(prompt "Username:" --required --min-length 3 --max-length 20)
```

### Validation Examples
```bash
# Email validation
email=$(prompt "Email address:" --validate-email --required)

# Custom pattern with message
code=$(prompt "Product code:" --pattern "^[A-Z]{3}-\d{4}$" --pattern-message "Format: ABC-1234")

# Number with range
port=$(prompt "Port number:" --integer --range 1024-65535)

# Date input
birthday=$(prompt "Birthday:" --date --date-format "%m/%d/%Y")
```

### Multiple Validation Rules
```bash
# Email with domain restrictions and no hyphens
email=$(prompt "Company email:" \
    --validate-email \
    --pattern ".*@(sixfeetup\.com|gmail\.com)$" \
    --pattern-message "Must be @sixfeetup.com or @gmail.com domain" \
    --pattern "^[^-]*$" \
    --pattern-message "Hyphens not allowed")

# Strong password requirements
password=$(prompt "Password:" \
    --required \
    --min-length 12 --length-priority high \
    --pattern ".*[A-Z].*" --pattern-message "Must contain uppercase letter" \
    --pattern ".*[a-z].*" --pattern-message "Must contain lowercase letter" \
    --pattern ".*[0-9].*" --pattern-message "Must contain number" \
    --pattern ".*[!@#$%^&*].*" --pattern-message "Must contain special character" \
    --mask)

# Username with multiple constraints
username=$(prompt "Username:" \
    --required \
    --min-length 3 --max-length 20 \
    --pattern "^[a-zA-Z0-9_]+$" --pattern-message "Only letters, numbers, and underscores" \
    --pattern "^[a-zA-Z].*" --pattern-message "Must start with a letter")
```

### Choice Selection
```bash
# Single choice (default behavior)
env=$(prompt "Environment:" --choices "dev,staging,prod")

# Multiple choices (allow 1-3 selections)
features=$(prompt "Select features:" --choices "auth,db,cache,api" --max-choices 3)

# Require at least 2 selections, allow up to 4
tags=$(prompt "Select tags:" --choices "urgent,bug,feature,docs,test" --min-choices 2 --max-choices 4)

# Optional selection (0 or 1 choice)
optional=$(prompt "Optional feature:" --choices "ssl,cache,logs" --min-choices 0)
```

### Custom Separators
```bash
# Git tags with space-delimited output
tags=$(prompt "Select tags:" --choices "$(git tag)" --selection-separator " " --min-choices 2)

# Semicolon input, pipe output for modules
modules=$(prompt "Pick modules:" --choices "auth;db;api;ui" --choice-separator ";" --selection-separator " | ")

# Custom delimiters for file selection  
files=$(prompt "Choose files:" --choices "$(find . -name '*.rs')" --selection-separator " " --max-choices 5)

# Specialized workflows with custom separators
options=$(prompt "Select options:" --choices "option1::option2::option3" --choice-separator "::" --selection-separator " + ")

# Shell integration with newline-separated choices
dirs=$(prompt "Select directories:" --choices "$(ls -1 -d */)" --min-choices 1 --max-choices 3)
```

### File Validation
```bash
# File must exist and be readable
config=$(prompt "Config file:" --file-exists --readable)

# Directory for output
output_dir=$(prompt "Output directory:" --dir-exists --writable)
```

### Output Formats
```bash
# Default (value + exit code)
if hostname=$(prompt "Hostname:" --validate-hostname); then
    echo "Valid hostname: $hostname"
fi

# JSON output
result=$(prompt "Email:" --validate-email --output json)
email=$(echo "$result" | jq -r '.value')
valid=$(echo "$result" | jq -r '.valid')

# Quiet mode for batch processing
echo "test@example.com" | prompt --validate-email --quiet
```

### Advanced Usage
```bash
# Password with confirmation
password=$(prompt "Password:" --mask --min-length 8 --pattern ".*[A-Z].*" --confirm)

# With timeout and default
region=$(prompt "AWS Region:" --default "us-east-1" --timeout 30)

# Multiple validation rules with priorities
username=$(prompt "Username:" \
    --required --required-priority critical \
    --min-length 3 --length-priority high \
    --pattern "^[a-zA-Z0-9_]+$" --pattern-priority high \
    --max-length 20 --length-priority medium)
```

### Non-interactive Usage
```bash
# Validate from stdin
cat usernames.txt | while read username; do
    if echo "$username" | prompt --pattern "^[a-zA-Z0-9_]+$" --quiet; then
        echo "Valid: $username"
    else
        echo "Invalid: $username" >&2
    fi
done
```

## Exit Codes

- `0`: Success - valid input received
- `1`: Validation failed or invalid input
- `2`: Invalid command-line arguments
- `3`: Maximum attempts exceeded
- `130`: User interrupted (Ctrl+C)
- `124`: Timeout exceeded

## Shell Completion

The `prompt` tool supports shell completion for all major shells to improve command-line usability and discoverability of options.

### Installation

Generate and install completion scripts for your shell:

#### Bash
```bash
# Install for current user
prompt completion bash > ~/.bash_completion.d/prompt
source ~/.bash_completion.d/prompt

# Or install system-wide (requires sudo)
prompt completion bash | sudo tee /etc/bash_completion.d/prompt
```

#### Zsh
```bash
# Create completions directory if it doesn't exist
mkdir -p ~/.zsh/completions

# Install completion script
prompt completion zsh > ~/.zsh/completions/_prompt

# Add to .zshrc if not already present
echo 'fpath=(~/.zsh/completions $fpath)' >> ~/.zshrc
echo 'autoload -U compinit && compinit' >> ~/.zshrc
```

#### Fish
```bash
# Install completion script
prompt completion fish > ~/.config/fish/completions/prompt.fish
```

#### PowerShell
```powershell
# Install completion script
prompt completion powershell | Out-String | Invoke-Expression
```

### Features

Shell completion provides:

- **Flag completion**: All `--` flags and options
- **Value completion**: Enum values like output formats (`default`, `json`, `raw`) and priorities (`critical`, `high`, `medium`, `low`)
- **Subcommand completion**: `completion` subcommand and shell types
- **Pattern hints**: Smart completion for regex patterns and date formats

### Usage

After installation, you can:

```bash
# Tab completion for flags
prompt --<TAB>
# Shows: --required --validate-email --output --quiet --min-length ...

# Tab completion for output format values
prompt --output <TAB>
# Shows: default json raw

# Tab completion for priority levels
prompt --required-priority <TAB>
# Shows: critical high medium low

# Tab completion for shell types
prompt completion <TAB>
# Shows: bash zsh fish power-shell
```

## Environment Variables

- `PROMPT_NO_COLOR`: Disable colored output (same as --no-color)
- `PROMPT_WIDTH`: Default display width
- `PROMPT_TIMEOUT`: Default timeout in seconds