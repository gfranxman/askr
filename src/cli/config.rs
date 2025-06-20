use super::args::{OutputFormat, PromptArgs};
use crate::error::{PromptError, Result};
use crate::validation::{ValidationRuleConfig, ValidatorType};
use std::collections::HashMap;
use std::time::Duration;

/// Main configuration for the prompt tool
#[derive(Debug, Clone)]
pub struct PromptConfig {
    pub prompt_text: Option<String>,
    pub output_format: OutputFormat,
    pub quiet_mode: bool,
    pub verbose: bool,
    pub validation_rules: Vec<ValidationRuleConfig>,
    pub ui_config: UiConfig,
    pub interaction_config: InteractionConfig,
}

#[derive(Debug, Clone)]
pub struct UiConfig {
    pub no_color: bool,
    pub width: Option<u16>,
    pub help_text: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InteractionConfig {
    pub timeout: Option<Duration>,
    pub max_attempts: Option<u32>,
    pub default_value: Option<String>,
    pub mask_input: bool,
    pub require_confirmation: bool,
}

impl PromptConfig {
    pub fn from_args(args: PromptArgs) -> Result<Self> {
        let validation_rules = Self::build_validation_rules(&args)?;

        Ok(Self {
            prompt_text: args.prompt_text,
            output_format: args.output,
            quiet_mode: args.quiet,
            verbose: args.verbose,
            validation_rules,
            ui_config: UiConfig {
                no_color: Self::resolve_no_color(args.no_color),
                width: Self::resolve_width(args.width),
                help_text: args.help_text,
            },
            interaction_config: InteractionConfig {
                timeout: Self::resolve_timeout(args.timeout),
                max_attempts: args.max_attempts,
                default_value: args.default,
                mask_input: args.mask,
                require_confirmation: args.confirm,
            },
        })
    }

    /// Resolve no_color setting from CLI args or environment variable
    fn resolve_no_color(cli_no_color: bool) -> bool {
        if cli_no_color {
            true
        } else {
            std::env::var("ASKR_NO_COLOR").is_ok()
        }
    }

    /// Resolve width setting from CLI args or environment variable
    fn resolve_width(cli_width: Option<u16>) -> Option<u16> {
        cli_width.or_else(|| {
            std::env::var("ASKR_WIDTH")
                .ok()
                .and_then(|s| s.parse::<u16>().ok())
        })
    }

    /// Resolve timeout setting from CLI args or environment variable
    fn resolve_timeout(cli_timeout: Option<u64>) -> Option<Duration> {
        cli_timeout
            .map(Duration::from_secs)
            .or_else(|| {
                std::env::var("ASKR_TIMEOUT")
                    .ok()
                    .and_then(|s| s.parse::<u64>().ok())
                    .map(Duration::from_secs)
            })
    }

    fn build_validation_rules(args: &PromptArgs) -> Result<Vec<ValidationRuleConfig>> {
        let mut rules = Vec::new();

        // Required validation
        if args.required {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Required,
                priority: args.required_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        // Length validations
        if let Some(min_length) = args.min_length {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::MinLength(min_length),
                priority: args.length_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if let Some(max_length) = args.max_length {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::MaxLength(max_length),
                priority: args.length_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        // Pattern validations (can have multiple)
        for (i, pattern) in args.pattern.iter().enumerate() {
            let custom_message = args.pattern_message.get(i).cloned();
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Pattern(pattern.clone()),
                priority: args.pattern_priority.clone().map(Into::into),
                custom_message,
                parameters: HashMap::new(),
            });
        }

        // Built-in format validators
        if args.validate_email {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Email,
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if args.validate_hostname {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Hostname,
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if args.validate_url {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Url,
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if args.validate_ipv4 {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Ipv4,
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if args.validate_ipv6 {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Ipv6,
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        // Number validations
        if args.integer {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Integer,
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if args.float {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Float,
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if args.number {
            // Generic number validation - could be integer or float
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Float, // Allow both int and float
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if let Some(range_str) = &args.range {
            let (min, max) = Self::parse_range(range_str)?;
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Range(min, max),
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if args.positive {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Positive,
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if args.negative {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Negative,
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        // Date/time validations
        if args.date {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Date(args.date_format.clone()),
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if args.time {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Time(args.time_format.clone()),
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if args.datetime {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::DateTime(args.datetime_format.clone()),
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        // Choice validation - support custom separators
        let choices_opt = if let Some(choices_str) = &args.choices {
            Some(Self::parse_choices(choices_str, args.choice_separator.as_deref()))
        } else {
            None
        };

        if let Some(choices) = choices_opt {
            let mut parameters = HashMap::new();
            parameters.insert(
                "case_sensitive".to_string(),
                args.choices_case_sensitive.to_string(),
            );
            
            let min_choices = args.min_choices.unwrap_or(1);
            let max_choices = args.max_choices.unwrap_or_else(|| {
                if args.min_choices.is_some() {
                    // If min_choices is specified, default max_choices to all available choices
                    choices.len()
                } else {
                    // If neither is specified, default to single selection
                    1
                }
            });
            
            parameters.insert("min_choices".to_string(), min_choices.to_string());
            parameters.insert("max_choices".to_string(), max_choices.to_string());
            parameters.insert(
                "selection_separator".to_string(),
                args.selection_separator.as_deref().unwrap_or(",").to_string(),
            );

            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Choices(choices),
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters,
            });
        }

        // File system validations
        if args.file_exists {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::FileExists,
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if args.dir_exists {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::DirExists,
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if args.path_exists {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::PathExists,
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if args.readable {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Readable,
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if args.writable {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Writable,
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        if args.executable {
            rules.push(ValidationRuleConfig {
                validator_type: ValidatorType::Executable,
                priority: args.format_priority.clone().map(Into::into),
                custom_message: None,
                parameters: HashMap::new(),
            });
        }

        Ok(rules)
    }

    fn parse_range(range_str: &str) -> Result<(f64, f64)> {
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            return Err(PromptError::InvalidArguments(format!(
                "Invalid range format: '{}'. Expected format: 'min-max'",
                range_str
            )));
        }

        let min = parts[0].parse::<f64>().map_err(|_| {
            PromptError::InvalidArguments(format!("Invalid minimum value in range: '{}'", parts[0]))
        })?;

        let max = parts[1].parse::<f64>().map_err(|_| {
            PromptError::InvalidArguments(format!("Invalid maximum value in range: '{}'", parts[1]))
        })?;

        if min >= max {
            return Err(PromptError::InvalidArguments(format!(
                "Range minimum ({}) must be less than maximum ({})",
                min, max
            )));
        }

        Ok((min, max))
    }

    /// Parse choices from string, supporting custom separators
    fn parse_choices(choices_str: &str, custom_separator: Option<&str>) -> Vec<String> {
        match custom_separator {
            Some(separator) => {
                // Use custom separator
                choices_str
                    .split(separator)
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            }
            None => {
                // Auto-detect: prioritize newlines over commas
                if choices_str.contains('\n') {
                    // Parse as newline-separated
                    choices_str
                        .lines()
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                } else {
                    // Parse as comma-separated
                    choices_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                }
            }
        }
    }
}
