mod cli;
mod error;
mod input;
mod output;
mod ui;
mod validation;

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell as CompletionShell};
use cli::{Args, Commands, PromptConfig, Shell};
use error::{PromptError, Result};
use output::{DefaultFormatter, JsonFormatter, OutputFormatter, RawFormatter};
use ui::interactive::InteractivePrompt;
use ui::Terminal;
use validation::rules::{
    ChoiceValidator, DateTimeValidator, DateValidator, DirExistsValidator, EmailValidator,
    ExecutableValidator, FileExistsValidator, FloatValidator, HostnameValidator, IntegerValidator,
    Ipv4Validator, Ipv6Validator, MaxLengthValidator, MinLengthValidator, NegativeValidator,
    PathExistsValidator, PatternValidator, PositiveValidator, RangeValidator, ReadableValidator,
    RequiredValidator, TimeValidator, UrlValidator, WritableValidator,
};
use validation::{ValidationEngine, ValidatorType};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(e.exit_code());
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    // Handle completion subcommand
    if let Some(Commands::Completion { shell }) = args.command {
        generate_completion(shell);
        return Ok(());
    }

    let config = PromptConfig::from_args(args.prompt_args)?;

    // Get input based on mode
    let input = if config.quiet_mode {
        read_from_stdin()?
    } else {
        // Check if we can use interactive mode
        let terminal = Terminal::new()?;

        if terminal.capabilities().cursor_control {
            // Use interactive terminal UI
            let engine = build_validation_engine(&config)?;
            let mut interactive = InteractivePrompt::new(terminal, engine, config.clone())?;
            interactive.prompt()?
        } else {
            // Fall back to simple prompt
            prompt_simple(&config.prompt_text.as_deref().unwrap_or("Enter input:"))?
        }
    };

    // Final validation for output
    let engine = build_validation_engine(&config)?;
    let summary = engine.validate(&input);

    // Format output based on config
    let formatter: Box<dyn OutputFormatter> = match config.output_format {
        cli::args::OutputFormat::Default => Box::new(DefaultFormatter),
        cli::args::OutputFormat::Json => Box::new(JsonFormatter),
        cli::args::OutputFormat::Raw => Box::new(RawFormatter),
    };

    let output = formatter.format(&summary)?;

    // Print output to stdout
    if !output.is_empty() {
        println!("{}", output);
    }

    // Exit with appropriate code
    if summary.valid {
        Ok(())
    } else {
        Err(PromptError::ValidationFailed(
            summary
                .error
                .unwrap_or_else(|| "Validation failed".to_string()),
        ))
    }
}

fn build_validation_engine(config: &PromptConfig) -> Result<ValidationEngine> {
    let mut engine = ValidationEngine::new();

    // Build validators from config
    for rule_config in &config.validation_rules {
        let validator = create_validator(&rule_config.validator_type, &rule_config)?;
        engine.add_validator(validator);
    }

    Ok(engine)
}

fn create_validator(
    validator_type: &ValidatorType,
    rule_config: &validation::ValidationRuleConfig,
) -> Result<Box<dyn validation::Validator>> {
    match validator_type {
        ValidatorType::Required => {
            let mut validator = RequiredValidator::new();
            if let Some(msg) = &rule_config.custom_message {
                validator = RequiredValidator::with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::MinLength(length) => {
            let mut validator = MinLengthValidator::new(*length);
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::MaxLength(length) => {
            let mut validator = MaxLengthValidator::new(*length);
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Pattern(pattern) => {
            let mut validator = PatternValidator::new(pattern)?;
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Email => {
            let mut validator = EmailValidator::new();
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Hostname => {
            let mut validator = HostnameValidator::new();
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Url => {
            let mut validator = UrlValidator::new();
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Ipv4 => {
            let mut validator = Ipv4Validator::new();
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Ipv6 => {
            let mut validator = Ipv6Validator::new();
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Integer => {
            let mut validator = IntegerValidator::new();
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Float => {
            let mut validator = FloatValidator::new();
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Range(min, max) => {
            let mut validator = RangeValidator::between(*min, *max);
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Positive => {
            let mut validator = PositiveValidator::new();
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Negative => {
            let mut validator = NegativeValidator::new();
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Choices(choices) => {
            let mut validator = ChoiceValidator::new(choices.clone());

            // Extract parameters
            if let Some(case_sensitive_str) = rule_config.parameters.get("case_sensitive") {
                if let Ok(case_sensitive) = case_sensitive_str.parse::<bool>() {
                    validator = validator.case_sensitive(case_sensitive);
                }
            }

            if let Some(min_choices_str) = rule_config.parameters.get("min_choices") {
                if let Ok(min_choices) = min_choices_str.parse::<usize>() {
                    validator = validator.min_choices(min_choices);
                }
            }

            if let Some(max_choices_str) = rule_config.parameters.get("max_choices") {
                if let Ok(max_choices) = max_choices_str.parse::<usize>() {
                    validator = validator.max_choices(max_choices);
                }
            }

            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Date(format) => {
            let mut validator = DateValidator::new(format.clone());
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Time(format) => {
            let mut validator = TimeValidator::new(format.clone());
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::DateTime(format) => {
            let mut validator = DateTimeValidator::new(format.clone());
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::FileExists => {
            let mut validator = FileExistsValidator::new();
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::DirExists => {
            let mut validator = DirExistsValidator::new();
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::PathExists => {
            let mut validator = PathExistsValidator::new();
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Readable => {
            let mut validator = ReadableValidator::new();
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Writable => {
            let mut validator = WritableValidator::new();
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
        ValidatorType::Executable => {
            let mut validator = ExecutableValidator::new();
            if let Some(priority) = &rule_config.priority {
                validator = validator.with_priority(*priority);
            }
            if let Some(msg) = &rule_config.custom_message {
                validator = validator.with_message(msg);
            }
            Ok(Box::new(validator))
        }
    }
}

fn read_from_stdin() -> Result<String> {
    use std::io::{self, Read};
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_simple(prompt_text: &str) -> Result<String> {
    use std::io::{self, Write};

    eprint!("{} ", prompt_text);
    io::stderr().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

fn generate_completion(shell: Shell) {
    let mut cmd = Args::command();
    let shell = match shell {
        Shell::Bash => CompletionShell::Bash,
        Shell::Zsh => CompletionShell::Zsh,
        Shell::Fish => CompletionShell::Fish,
        Shell::PowerShell => CompletionShell::PowerShell,
    };

    generate(shell, &mut cmd, "prompt", &mut std::io::stdout());
}
