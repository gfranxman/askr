# Validation System Specification

## Overview

The validation system provides real-time input validation with priority-based error reporting. It supports multiple validation types, custom error messages, and extensible architecture for adding new validators.

## Priority System

### Priority Levels

```rust
enum Priority {
    Critical = 0,  // Blocking errors (required field empty)
    High = 1,      // Format violations (invalid email, regex mismatch)
    Medium = 2,    // Constraint violations (too long, out of range)
    Low = 3,       // Recommendations (weak password, style suggestions)
}
```

### Display Logic

- **Always show**: Critical and High priority errors
- **Conditionally show**: Medium priority (max 3 errors)
- **Limited show**: Low priority (max 2 errors, only if no higher priority errors)
- **Sorting**: Errors displayed in priority order, then by rule order

### Example Display

```
Enter password: weak123█
❌ [Critical] Password is required
❌ [High] Must contain at least one uppercase letter
❌ [Medium] Minimum length is 12 characters (currently 7)
⚠️  [Low] Consider using special characters for better security
```

## Validation Rules

### Rule Structure

```rust
struct ValidationRule {
    id: String,
    name: String,
    priority: Priority,
    validator: Box<dyn Validator>,
    message_template: String,
    enabled: bool,
}

trait Validator {
    fn validate(&self, input: &str) -> ValidationResult;
    fn partial_validate(&self, input: &str, cursor_pos: usize) -> PartialValidationResult;
}

struct ValidationResult {
    passed: bool,
    message: Option<String>,
    metadata: HashMap<String, String>,
}

struct PartialValidationResult {
    first_error_pos: Option<usize>,
    can_continue: bool,
    suggestion: Option<String>,
}
```

### Built-in Validators

#### Required Validator
- **Priority**: Critical
- **Logic**: Input cannot be empty or whitespace-only
- **Message**: "This field is required"

#### Length Validators
- **Priority**: Medium (configurable)
- **Types**: MinLength, MaxLength, ExactLength
- **Logic**: Character count validation
- **Messages**: 
  - "Minimum length is {min} characters (currently {actual})"
  - "Maximum length is {max} characters (currently {actual})"

#### Pattern Validator
- **Priority**: High (configurable)
- **Logic**: Regex pattern matching
- **Message**: Custom message or "Must match pattern: {pattern}"
- **Partial**: Highlight first character that breaks pattern

#### Format Validators

**Email Validator**
- **Priority**: High
- **Regex**: Advanced email validation pattern
- **Message**: "Must be a valid email address"
- **Partial**: Validate as user types (@ symbol, domain, etc.)

**Hostname Validator**
- **Priority**: High
- **Logic**: RFC-compliant hostname validation
- **Message**: "Must be a valid hostname"
- **Rules**: Length limits, character restrictions, label validation

**URL Validator**
- **Priority**: High
- **Logic**: URL scheme, host, path validation
- **Message**: "Must be a valid URL"

**IP Address Validators**
- **Priority**: High
- **Types**: IPv4, IPv6
- **Logic**: Address format and range validation
- **Messages**: "Must be a valid IPv4/IPv6 address"

#### Number Validators
- **Priority**: High for format, Medium for range
- **Types**: Integer, Float, Range, Positive, Negative
- **Logic**: Numeric parsing and constraint checking
- **Messages**: 
  - "Must be a valid number"
  - "Must be between {min} and {max}"
  - "Must be a positive/negative number"

#### Choice Validator
- **Priority**: High
- **Logic**: Input must match one of predefined choices
- **Options**: Case sensitivity, multiple selection
- **Message**: "Must be one of: {choices}"
- **Suggestions**: Show closest matches for typos

#### Date/Time Validators
- **Priority**: High
- **Types**: Date, Time, DateTime
- **Logic**: Format parsing with configurable patterns
- **Message**: "Must be a valid date in format {format}"
- **Partial**: Validate as user types date components

#### File System Validators
- **Priority**: High
- **Types**: FileExists, DirExists, PathExists, Readable, Writable, Executable
- **Logic**: File system checks with appropriate permissions
- **Messages**: 
  - "File does not exist: {path}"
  - "Directory is not writable: {path}"

### Custom Validators

#### External Command Validator
```bash
prompt "Username:" --validate-cmd "check_username.sh" --validate-cmd-message "Username not available"
```

#### Async Validators
- Network-based validation (DNS lookup, API calls)
- Debounced execution to avoid excessive requests
- Timeout handling and fallback behavior

## Validation Engine

### Real-time Validation Flow

1. **On Input Change**:
   - Run all partial validators
   - Determine first error position for text coloring
   - Check if input can continue (some validators may block further typing)

2. **On Complete Input**:
   - Run all full validators
   - Collect all validation results
   - Sort by priority and rule order
   - Format messages for display

3. **On Submit**:
   - Final validation pass
   - Return success/failure with complete error set

### Performance Considerations

#### Debouncing
- **Fast validators**: No debouncing (length, required)
- **Medium validators**: 100ms debounce (regex, format)
- **Slow validators**: 500ms debounce (external commands, network)

#### Caching
- Cache validation results for identical inputs
- Clear cache on rule changes
- TTL for network-based validators

#### Optimization
- Short-circuit on first critical error
- Lazy evaluation of low-priority rules
- Batch similar operations (multiple regex patterns)

### Error Message Templates

#### Template Variables
- `{value}`: Current input value
- `{length}`: Current input length
- `{min}`, `{max}`: Constraint values
- `{pattern}`: Regex pattern
- `{choices}`: Available choices
- `{format}`: Expected format

#### Internationalization Support
- Message templates separated from validation logic
- Language-specific error messages
- Cultural formatting for dates, numbers

### Rule Composition

#### Rule Groups
```bash
# Password validation group
prompt "Password:" \
    --required \
    --min-length 8 --length-priority medium \
    --pattern ".*[A-Z].*" --pattern-message "Must contain uppercase" --pattern-priority high \
    --pattern ".*[0-9].*" --pattern-message "Must contain number" --pattern-priority high \
    --pattern ".*[!@#$%^&*].*" --pattern-message "Consider special characters" --pattern-priority low
```

#### Conditional Rules
- Rules that activate based on other rule results
- Complex validation workflows
- Context-dependent validation

### Extensibility

#### Plugin Architecture
```rust
trait ValidatorPlugin {
    fn name(&self) -> &str;
    fn create_validator(&self, config: &ValidationConfig) -> Box<dyn Validator>;
    fn supported_options(&self) -> Vec<CliOption>;
}
```

#### Custom Rule Registration
- Dynamic validator loading
- Configuration-driven rule creation
- Third-party validator integration