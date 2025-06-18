# Implementation Notes

## Rust Dependencies

### Core Dependencies

#### Terminal Control
- **crossterm** (^0.27): Cross-platform terminal control
  - Raw mode input handling
  - ANSI color output
  - Cursor positioning and movement
  - Terminal size detection
  - Event handling (key presses, resize)

#### CLI Parsing  
- **clap** (^4.4): Command-line argument parsing
  - Derive macros for clean CLI definition
  - Validation of argument combinations
  - Help text generation
  - Subcommand support (if needed for future extensions)

#### Serialization
- **serde** (^1.0): Serialization framework
- **serde_json** (^1.0): JSON output format support

#### Date/Time Handling
- **chrono** (^0.4): Date and time parsing/validation
  - Multiple date format support
  - Timezone handling
  - Validation of date ranges

#### Regular Expressions
- **regex** (^1.10): Pattern validation
  - Compile-time regex optimization
  - Unicode support
  - Error message extraction

### Optional Dependencies

#### Async Support (Future)
- **tokio** (^1.35): Async runtime for network validators
- **reqwest** (^0.11): HTTP client for URL validation

#### Advanced Terminal Features
- **unicode-width** (^0.1): Proper Unicode character width calculation
- **unicode-segmentation** (^1.10): Grapheme cluster handling

#### Performance
- **once_cell** (^1.19): Lazy static initialization for validators
- **dashmap** (^5.5): Concurrent hashmap for caching validation results

## Architecture Overview

### Module Structure
```
src/
├── main.rs              # CLI entry point and orchestration
├── cli/
│   ├── mod.rs          # CLI argument parsing and validation
│   ├── args.rs         # Argument definitions and parsing
│   └── config.rs       # Configuration from CLI args
├── validation/
│   ├── mod.rs          # Validation system orchestration
│   ├── engine.rs       # Validation engine and rule management
│   ├── rules/          # Individual validation rule implementations
│   │   ├── mod.rs
│   │   ├── basic.rs    # Required, length, pattern validators
│   │   ├── format.rs   # Email, URL, IP address validators
│   │   ├── numeric.rs  # Number, range validators
│   │   ├── datetime.rs # Date/time validators
│   │   ├── choice.rs   # Choice/selection validators
│   │   └── filesystem.rs # File/directory validators
│   ├── priority.rs     # Priority system and error sorting
│   └── result.rs       # Validation result types
├── ui/
│   ├── mod.rs          # UI orchestration
│   ├── interactive.rs  # Interactive terminal UI
│   ├── quiet.rs        # Non-interactive/quiet mode
│   ├── terminal.rs     # Terminal control and capabilities
│   ├── colors.rs       # Color scheme and ANSI handling
│   └── layout.rs       # Screen layout and rendering
├── output/
│   ├── mod.rs          # Output format orchestration
│   ├── default.rs      # Default stdout + exit code output
│   ├── json.rs         # JSON format output
│   └── raw.rs          # Raw output mode
├── input/
│   ├── mod.rs          # Input handling orchestration
│   ├── interactive.rs  # Interactive input collection
│   ├── stdin.rs        # Stdin input for quiet mode  
│   └── processor.rs    # Input processing and normalization
└── error.rs            # Error types and handling
```

### Core Types

#### Configuration
```rust
#[derive(Debug, Clone)]
pub struct PromptConfig {
    pub prompt_text: Option<String>,
    pub output_format: OutputFormat,
    pub quiet_mode: bool,
    pub validation_rules: Vec<ValidationRuleConfig>,
    pub ui_config: UiConfig,
    pub timeout: Option<Duration>,
    pub max_attempts: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Default,
    Json,
    Raw,
}

#[derive(Debug, Clone)]
pub struct UiConfig {
    pub no_color: bool,
    pub width: Option<u16>,
    pub show_help: bool,
}
```

#### Validation System
```rust
pub trait Validator: Send + Sync {
    fn validate(&self, input: &str) -> ValidationResult;
    fn partial_validate(&self, input: &str, cursor_pos: usize) -> PartialValidationResult;
    fn priority(&self) -> Priority;
    fn name(&self) -> &str;
}

#[derive(Debug)]
pub struct ValidationEngine {
    rules: Vec<Box<dyn Validator>>,
    cache: Option<DashMap<String, Vec<ValidationResult>>>,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub rule_name: String,
    pub passed: bool,
    pub priority: Priority,
    pub message: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}
```

#### Terminal UI
```rust
pub struct InteractiveTerminal {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    input_buffer: String,
    cursor_pos: usize,
    error_area_height: u16,
    capabilities: TerminalCapabilities,
    reserved_lines: u16, // Smart prompt placement
}

#[derive(Debug)]
pub struct TerminalCapabilities {
    pub colors_supported: bool,
    pub cursor_control: bool,
    pub unicode_support: bool,
    pub width: u16,
    pub height: u16,
}
```

#### Smart Prompt Placement
```rust
impl InteractivePrompt {
    fn calculate_and_reserve_space(&self, width: u16, prompt_text: &str) -> Result<u16> {
        // Get all potential error messages by testing validators
        let messages = self.validation_engine.get_potential_error_messages();
        
        // Calculate space needed including text wrapping
        let mut total_lines = 1; // prompt line
        for message in &messages {
            total_lines += self.calculate_wrapped_lines(message, width);
        }
        
        // Reserve space and position cursor
        self.print_blank_lines(total_lines)?;
        self.move_cursor_up(total_lines)?;
        Ok(total_lines)
    }
    
    fn get_potential_error_messages(&self) -> Vec<String> {
        // Test validators with various inputs to discover all possible errors
        // This enables accurate space calculation before user interaction
    }
}
```

#### Enhanced Line Editing
```rust
impl KeyEventHandler {
    fn handle_emacs_shortcuts(&mut self, key: KeyEvent) -> InputAction {
        match key {
            // Ctrl+A: Jump to beginning
            KeyEvent { code: KeyCode::Char('a'), modifiers: KeyModifiers::CONTROL, .. } => {
                self.cursor_pos = 0;
                InputAction::Continue
            }
            // Ctrl+E: Jump to end
            KeyEvent { code: KeyCode::Char('e'), modifiers: KeyModifiers::CONTROL, .. } => {
                self.cursor_pos = self.input.chars().count();
                InputAction::Continue
            }
            // Ctrl+K: Kill to end of line
            KeyEvent { code: KeyCode::Char('k'), modifiers: KeyModifiers::CONTROL, .. } => {
                self.kill_to_end();
                InputAction::Continue
            }
            // Ctrl+U: Kill to beginning of line
            KeyEvent { code: KeyCode::Char('u'), modifiers: KeyModifiers::CONTROL, .. } => {
                self.kill_to_beginning();
                InputAction::Continue
            }
            // Ctrl+W: Delete word backward
            KeyEvent { code: KeyCode::Char('w'), modifiers: KeyModifiers::CONTROL, .. } => {
                self.delete_word_backward();
                InputAction::Continue
            }
            _ => self.handle_regular_key(key)
        }
    }
}
```

## Error Handling Strategy

### Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum PromptError {
    #[error("Invalid CLI arguments: {0}")]
    InvalidArguments(String),
    
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Terminal error: {0}")]
    Terminal(#[from] crossterm::ErrorKind),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Timeout exceeded")]
    Timeout,
    
    #[error("User interrupted")]
    Interrupted,
    
    #[error("Maximum attempts exceeded")]
    MaxAttemptsExceeded,
}
```

### Error Recovery
- **Terminal errors**: Graceful fallback to simpler UI
- **Validation errors**: Continue prompting with error display
- **Timeout/interruption**: Clean exit with appropriate codes
- **Configuration errors**: Early exit with help message

## Performance Considerations

### Memory Management
- **String interning**: Reuse common error messages
- **Validation caching**: Cache results for identical inputs
- **Buffer reuse**: Reuse terminal buffers to avoid allocations
- **Lazy initialization**: Initialize expensive validators only when needed

### Real-time Performance
- **Debouncing**: Prevent excessive validation calls
  ```rust
  struct DebouncedValidator {
      validator: Box<dyn Validator>,
      debounce_duration: Duration,
      last_validation: Instant,
      cached_result: Option<ValidationResult>,
  }
  ```

- **Async validation**: Non-blocking validation for network calls
- **Partial validation**: Fast validation during typing
- **Screen updates**: Minimize terminal I/O operations

### Startup Performance
- **Lazy regex compilation**: Compile patterns only when used
- **Plugin loading**: Dynamic validator loading only if needed
- **Terminal detection**: Cache capabilities for session

## Testing Strategy

### Unit Tests
- **Validation rules**: Comprehensive test cases for each validator
- **CLI parsing**: Test argument combinations and edge cases
- **Output formats**: Verify JSON schema compliance and format correctness
- **Error handling**: Test all error paths and recovery

### Integration Tests
- **End-to-end scenarios**: Full user workflows with various inputs
- **Terminal simulation**: Mock terminal for UI testing
- **Shell script integration**: Test actual shell script usage patterns
- **Cross-platform**: Tests on Linux, macOS, Windows

### Property Testing
- **Input fuzzing**: Random input generation for validators
- **Unicode handling**: Test with various Unicode characters
- **Terminal edge cases**: Different terminal sizes and capabilities

### Performance Tests
- **Response time**: Measure keystroke response latency
- **Memory usage**: Profile memory allocation patterns
- **Stress testing**: Large inputs, many validation rules

## Deployment Considerations

### Binary Distribution
- **Static linking**: Minimize runtime dependencies
- **Cross-compilation**: Support multiple architectures
- **Binary size**: Optimize for reasonable executable size
- **Stripping**: Remove debug symbols for release builds

### Installation Methods
- **Cargo install**: Primary distribution via crates.io
- **Package managers**: Consider homebrew, apt, etc.
- **GitHub releases**: Pre-built binaries for major platforms
- **Container images**: Docker image for containerized usage

### Configuration
- **Environment variables**: Support standard Unix conventions
- **Config files**: None initially, but consider for future
- **Defaults**: Sensible defaults for all configuration options

## Future Extensibility

### Plugin System
```rust
pub trait ValidatorPlugin {
    fn name(&self) -> &str;
    fn create_validator(&self, config: &serde_json::Value) -> Result<Box<dyn Validator>>;
    fn cli_options(&self) -> Vec<clap::Arg>;
}

pub struct PluginManager {
    plugins: HashMap<String, Box<dyn ValidatorPlugin>>,
}
```

### Network Validators
- **DNS resolution**: Hostname validation with actual lookup
- **HTTP validation**: URL validation with connectivity check
- **API validation**: Custom endpoint validation

### Advanced UI Features
- **Multi-line input**: Support for text area input
- **Auto-completion**: Tab completion for choice validators
- **History**: Input history across sessions
- **Themes**: Customizable color schemes

### Configuration Language
```yaml
# ~/.prompt/config.yaml
validators:
  email:
    priority: high
    message: "Please enter a valid email address"
  
  strong_password:
    rules:
      - min_length: 12
      - pattern: ".*[A-Z].*"
      - pattern: ".*[0-9].*"
    priority: medium
```