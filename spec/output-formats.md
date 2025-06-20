# Output Formats Specification

## Overview

The tool supports multiple output formats to accommodate different use cases, from simple shell script integration to complex data processing workflows. All modes respect the stdout/stderr separation principle for proper Unix tool behavior.

## Default Mode (Exit Codes + stdout)

### Behavior
- **Interactive prompts**: Displayed on stderr
- **User input**: Captured from stdin (terminal)
- **Valid result**: Written to stdout
- **Error messages**: Written to stderr
- **Exit codes**: Indicate success/failure status

### Usage Examples
```bash
# Simple assignment
hostname=$(askr "Enter hostname:" --validate-hostname)

# With error checking
if hostname=$(askr "Enter hostname:" --validate-hostname); then
    echo "Connecting to $hostname"
    ssh user@$hostname
else
    echo "Invalid hostname provided" >&2
    exit 1
fi

# Capture both value and exit status
hostname=$(askr "Enter hostname:" --validate-hostname)
exit_code=$?
if [ $exit_code -eq 0 ]; then
    echo "Valid hostname: $hostname"
fi
```

### Output Specification
- **stdout**: Only the validated input value (no trailing newline issues)
- **stderr**: All interactive elements (prompt, errors, help text)
- **Exit codes**: Standard Unix conventions

## JSON Mode (--output json)

### Output Structure
```json
{
  "value": "user@example.com",
  "valid": true,
  "error": null,
  "metadata": {
    "validation_time_ms": 15,
    "rules_checked": 3,
    "rules_passed": 3,
    "input_length": 16
  },
  "validation_results": [
    {
      "rule": "required",
      "passed": true,
      "priority": "critical",
      "message": null
    },
    {
      "rule": "email_format",
      "passed": true,
      "priority": "high",
      "message": null
    },
    {
      "rule": "max_length",
      "passed": true,
      "priority": "medium",
      "message": null
    }
  ]
}
```

### JSON Schema
```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["value", "valid"],
  "properties": {
    "value": {
      "type": "string",
      "description": "The input value provided by the user"
    },
    "valid": {
      "type": "boolean",
      "description": "Whether the input passed all validation rules"
    },
    "error": {
      "type": ["string", "null"],
      "description": "Primary error message if validation failed"
    },
    "metadata": {
      "type": "object",
      "properties": {
        "validation_time_ms": {"type": "number"},
        "rules_checked": {"type": "integer"},
        "rules_passed": {"type": "integer"},
        "input_length": {"type": "integer"},
        "attempts": {"type": "integer"}
      }
    },
    "validation_results": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["rule", "passed", "priority"],
        "properties": {
          "rule": {"type": "string"},
          "passed": {"type": "boolean"},
          "priority": {"enum": ["critical", "high", "medium", "low"]},
          "message": {"type": ["string", "null"]},
          "metadata": {"type": "object"}
        }
      }
    }
  }
}
```

### Usage Examples
```bash
# Parse with jq
result=$(askr "Email:" --validate-email --output json)
email=$(echo "$result" | jq -r '.value')
valid=$(echo "$result" | jq -r '.valid')

if [ "$valid" = "true" ]; then
    echo "Email: $email"
else
    error=$(echo "$result" | jq -r '.error')
    echo "Validation failed: $error" >&2
fi

# Extract validation details
result=$(askr "Password:" --min-length 8 --pattern ".*[A-Z].*" --output json)
failed_rules=$(echo "$result" | jq -r '.validation_results[] | select(.passed == false) | .rule')
```

## Raw Mode (--output raw)

### Purpose
Debug mode that shows exactly what the user typed without any processing or validation.

### Behavior
- **No validation**: Input returned as-is
- **No processing**: No trimming, normalization, or transformation
- **Debugging**: Useful for troubleshooting validation issues
- **Exit codes**: Always 0 unless interrupted

### Usage Examples
```bash
# Debug what user actually typed
raw_input=$(askr "Enter text:" --output raw)
echo "User typed: '$raw_input'"
echo "Length: ${#raw_input}"
echo "Hex dump: $(echo -n "$raw_input" | xxd)"
```

## Quiet Mode (--quiet)

### Non-interactive Validation
- **No prompts**: Reads from stdin instead of interactive input
- **No UI**: No colors, error displays, or interactive elements
- **Batch processing**: Designed for processing multiple inputs
- **Same output formats**: Works with default, JSON, and raw modes

### Usage Examples
```bash
# Single validation
echo "test@example.com" | askr --validate-email --quiet

# Batch processing
cat emails.txt | while read email; do
    if echo "$email" | askr --validate-email --quiet; then
        echo "Valid: $email"
    else
        echo "Invalid: $email" >&2
    fi
done

# With JSON output for batch processing
cat hostnames.txt | while read hostname; do
    result=$(echo "$hostname" | askr --validate-hostname --quiet --output json)
    valid=$(echo "$result" | jq -r '.valid')
    if [ "$valid" = "true" ]; then
        echo "✅ $hostname"
    else
        error=$(echo "$result" | jq -r '.error')
        echo "❌ $hostname: $error" >&2
    fi
done
```

## Exit Codes

### Standard Exit Codes
- `0`: Success - validation passed
- `1`: Validation failed - input did not meet requirements
- `2`: Invalid arguments - command-line usage error
- `3`: Maximum attempts exceeded - user gave up after max retries
- `124`: Timeout exceeded - user didn't respond within timeout
- `130`: User interrupted - Ctrl+C pressed

### Exit Code Usage
```bash
# Simple success/failure
askr "Name:" --required
case $? in
    0) echo "Success" ;;
    1) echo "Invalid input" ;;
    2) echo "Usage error" ;;
    3) echo "Too many attempts" ;;
    124) echo "Timeout" ;;
    130) echo "Cancelled" ;;
esac

# In conditionals
if askr "Continue?" --choices "yes,no" --quiet <<< "yes"; then
    echo "Proceeding..."
fi

# Choice validation with custom separators
env=$(askr "Environment:" --choices "dev,staging,prod")
modules=$(askr "Modules:" --choices "auth;db;api" --choice-separator ";" --selection-separator " | " --max-choices 2)
```

## Stream Handling

### stdout Behavior
- **Default mode**: Only the validated value
- **JSON mode**: Complete JSON object
- **Raw mode**: Exact user input
- **Quiet mode**: Same as interactive for output content
- **No extra newlines**: Output exactly what's needed

### stderr Behavior
- **Interactive elements**: Prompts, error messages, help text
- **Progress indicators**: Validation status, loading messages
- **Warnings**: Non-fatal issues or recommendations
- **Debug info**: When verbose mode enabled

### Examples
```bash
# Redirect streams appropriately
valid_email=$(askr "Email:" --validate-email 2>/dev/null)
if [ $? -eq 0 ]; then
    echo "Got email: $valid_email"
fi

# Capture errors separately
{
    hostname=$(askr "Hostname:" --validate-hostname)
} 2>error.log

if [ $? -ne 0 ]; then
    echo "Validation failed, see error.log"
fi
```

## Format Comparison

### Use Case Matrix

| Use Case | Recommended Mode | Rationale |
|----------|------------------|-----------|
| Simple shell scripts | Default | Unix conventions, easy error handling |
| Complex data processing | JSON | Rich metadata, structured parsing |
| Batch validation | Quiet + Default | Non-interactive, standard exit codes |
| Debugging validation | Raw | See exact user input |
| Automated testing | JSON + Quiet | Predictable structured output |
| Log processing | Quiet + JSON | Machine-readable validation results |

### Performance Characteristics

| Mode | Startup Time | Memory Usage | Output Size |
|------|-------------|--------------|-------------|
| Default | Fast | Low | Minimal |
| JSON | Medium | Medium | Verbose |
| Raw | Fastest | Lowest | Minimal |
| Quiet | Fast | Low | Same as base mode |

### Compatibility

| Output Mode | Shell Scripts | JSON Parsers | Log Analysis | Testing |
|-------------|---------------|--------------|--------------|---------|
| Default | ✅ Excellent | ❌ No | ⚠️ Limited | ✅ Good |
| JSON | ⚠️ Complex | ✅ Excellent | ✅ Excellent | ✅ Excellent |
| Raw | ✅ Good | ❌ No | ⚠️ Limited | ✅ Good |
| Quiet | ✅ Excellent | ➕ With JSON | ✅ Good | ✅ Excellent |
