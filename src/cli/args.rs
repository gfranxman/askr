use crate::validation::Priority;
use clap::{Parser, Subcommand, ValueEnum, ValueHint};

#[derive(Parser, Debug)]
#[command(name = "askr")]
#[command(about = "Interactive CLI input tool with real-time validation and choice menus")]
#[command(version = "0.1.2")]
#[command(long_about = "Interactive CLI input tool with real-time validation and choice menus.

SHELL INTEGRATION EXAMPLES:
  # Select from files in current directory
  askr \"Select file:\" --choices \"$(ls -1)\"

  # Pick from git branches  
  askr \"Switch to:\" --choices \"$(git branch --format='%(refname:short)')\"

  # Multiple selections with custom separators
  askr \"Select files:\" --choices \"$(find . -name '*.txt')\" --min-choices 2 --max-choices 5 --selection-separator \" | \"

  # Custom choice separator (semicolon-separated)
  askr \"Choose option:\" --choices \"apple;banana;cherry\" --choice-separator \";\"

  # Pipe-separated choices with space-separated output
  askr \"Select tags:\" --choices \"tag1|tag2|tag3\" --choice-separator \"|\" --selection-separator \" \" --min-choices 1 --max-choices 3")]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[command(flatten)]
    pub prompt_args: PromptArgs,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate shell completion scripts
    Completion {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

#[derive(Parser, Debug)]
pub struct PromptArgs {
    /// The text to display as the prompt
    pub prompt_text: Option<String>,

    // Output Control
    /// Output format
    #[arg(long, value_enum, default_value = "default")]
    pub output: OutputFormat,

    /// Non-interactive mode, read from stdin
    #[arg(long)]
    pub quiet: bool,

    /// Show detailed validation messages to stderr
    #[arg(long)]
    pub verbose: bool,

    // Basic Validation
    /// Input cannot be empty
    #[arg(long)]
    pub required: bool,

    /// Maximum character length
    #[arg(long)]
    pub max_length: Option<usize>,

    /// Minimum character length  
    #[arg(long)]
    pub min_length: Option<usize>,

    /// Custom regex pattern (can be used multiple times)
    #[arg(long, value_hint = ValueHint::Other)]
    pub pattern: Vec<String>,

    /// Custom error message for pattern validation (applies to most recent --pattern)
    #[arg(long)]
    pub pattern_message: Vec<String>,

    // Built-in Validators
    /// Email address validation
    #[arg(long)]
    pub validate_email: bool,

    /// Hostname/domain validation
    #[arg(long)]
    pub validate_hostname: bool,

    /// URL validation
    #[arg(long)]
    pub validate_url: bool,

    /// IPv4 address validation
    #[arg(long)]
    pub validate_ipv4: bool,

    /// IPv6 address validation
    #[arg(long)]
    pub validate_ipv6: bool,

    // Number Validation
    /// Accept only numeric input
    #[arg(long)]
    pub number: bool,

    /// Accept only integer input
    #[arg(long)]
    pub integer: bool,

    /// Accept only floating-point input
    #[arg(long)]
    pub float: bool,

    /// Numeric range (e.g., 1-100)
    #[arg(long)]
    pub range: Option<String>,

    /// Only positive numbers
    #[arg(long)]
    pub positive: bool,

    /// Only negative numbers
    #[arg(long)]
    pub negative: bool,

    // Date/Time Validation
    /// Accept date input
    #[arg(long)]
    pub date: bool,

    /// Expected date format (default: %Y-%m-%d)
    #[arg(long, value_hint = ValueHint::Other)]
    pub date_format: Option<String>,

    /// Accept time input
    #[arg(long)]
    pub time: bool,

    /// Expected time format (default: %H:%M:%S)
    #[arg(long, value_hint = ValueHint::Other)]
    pub time_format: Option<String>,

    /// Accept datetime input
    #[arg(long)]
    pub datetime: bool,

    /// Expected datetime format
    #[arg(long, value_hint = ValueHint::Other)]
    pub datetime_format: Option<String>,

    // Choice Validation
    /// Comma or newline-separated list of valid choices
    #[arg(long)]
    pub choices: Option<String>,

    /// Custom separator for parsing choices (default: auto-detect comma/newline)
    #[arg(long)]
    pub choice_separator: Option<String>,

    /// Custom separator for joining multiple selections in output (default: comma)
    #[arg(long)]
    pub selection_separator: Option<String>,

    /// Make choice matching case-sensitive
    #[arg(long)]
    pub choices_case_sensitive: bool,

    /// Minimum number of choices required (default: 1)
    #[arg(long)]
    pub min_choices: Option<usize>,

    /// Maximum number of choices allowed (default: 1)
    #[arg(long)]
    pub max_choices: Option<usize>,

    // File System Validation
    /// File must exist
    #[arg(long)]
    pub file_exists: bool,

    /// Directory must exist
    #[arg(long)]
    pub dir_exists: bool,

    /// File or directory must exist
    #[arg(long)]
    pub path_exists: bool,

    /// Path must be readable
    #[arg(long)]
    pub readable: bool,

    /// Path must be writable
    #[arg(long)]
    pub writable: bool,

    /// File must be executable
    #[arg(long)]
    pub executable: bool,

    // Priority Control
    /// Priority for required validation (default: critical)
    #[arg(long, value_enum)]
    pub required_priority: Option<PriorityArg>,

    /// Priority for length validation (default: medium)
    #[arg(long, value_enum)]
    pub length_priority: Option<PriorityArg>,

    /// Priority for pattern validation (default: high)
    #[arg(long, value_enum)]
    pub pattern_priority: Option<PriorityArg>,

    /// Priority for format validation (default: high)
    #[arg(long, value_enum)]
    pub format_priority: Option<PriorityArg>,

    // Interaction Control
    /// Maximum validation attempts (default: unlimited)
    #[arg(long)]
    pub max_attempts: Option<u32>,

    /// Input timeout in seconds
    #[arg(long)]
    pub timeout: Option<u64>,

    /// Default value if user presses Enter
    #[arg(long)]
    pub default: Option<String>,

    /// Mask input (for passwords)
    #[arg(long)]
    pub mask: bool,

    /// Require confirmation input
    #[arg(long)]
    pub confirm: bool,

    // Display Control
    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    /// Maximum display width
    #[arg(long)]
    pub width: Option<u16>,

    /// Additional help text displayed below prompt
    #[arg(long)]
    pub help_text: Option<String>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Default,
    Json,
    Raw,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum PriorityArg {
    Critical,
    High,
    Medium,
    Low,
}

impl From<PriorityArg> for Priority {
    fn from(arg: PriorityArg) -> Self {
        match arg {
            PriorityArg::Critical => Priority::Critical,
            PriorityArg::High => Priority::High,
            PriorityArg::Medium => Priority::Medium,
            PriorityArg::Low => Priority::Low,
        }
    }
}
