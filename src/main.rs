mod cli;
mod error;
mod validation;
mod output;
mod ui;
mod input;

use clap::Parser;
use cli::{Args, PromptConfig};
use error::{PromptError, Result};
use validation::{ValidationEngine, ValidatorType};
use validation::rules::{RequiredValidator, MinLengthValidator, MaxLengthValidator, PatternValidator};
use output::{OutputFormatter, DefaultFormatter, JsonFormatter, RawFormatter};
use ui::{Terminal, TerminalCapabilities};
use ui::interactive::InteractivePrompt;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(e.exit_code());
    }
}

fn run() -> Result<()> {
    let args = Args::parse();
    let config = PromptConfig::from_args(args)?;
    
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
            summary.error.unwrap_or_else(|| "Validation failed".to_string())
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

fn create_validator(validator_type: &ValidatorType, rule_config: &validation::ValidationRuleConfig) -> Result<Box<dyn validation::Validator>> {
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
        _ => {
            // For now, return an error for unimplemented validators
            Err(PromptError::InvalidArguments(
                format!("Validator type {:?} not yet implemented", validator_type)
            ))
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
