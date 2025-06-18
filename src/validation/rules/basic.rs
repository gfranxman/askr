use regex::Regex;
use once_cell::sync::Lazy;
use super::super::{Validator, ValidationResult, PartialValidationResult, Priority};

/// Validator that ensures input is not empty
#[derive(Debug)]
pub struct RequiredValidator {
    custom_message: Option<String>,
}

impl RequiredValidator {
    pub fn new() -> Self {
        Self {
            custom_message: None,
        }
    }
    
    pub fn with_message(message: impl Into<String>) -> Self {
        Self {
            custom_message: Some(message.into()),
        }
    }
}

impl Validator for RequiredValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        if input.trim().is_empty() {
            ValidationResult::failure(
                "required",
                Priority::Critical,
                self.custom_message.as_deref().unwrap_or("This field is required")
            )
        } else {
            ValidationResult::success("required")
        }
    }
    
    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.trim().is_empty() {
            PartialValidationResult::error_at(0)
        } else {
            PartialValidationResult::valid()
        }
    }
    
    fn priority(&self) -> Priority {
        Priority::Critical
    }
    
    fn name(&self) -> &str {
        "required"
    }
}

/// Validator for minimum length
#[derive(Debug)]
pub struct MinLengthValidator {
    min_length: usize,
    priority: Priority,
    custom_message: Option<String>,
}

impl MinLengthValidator {
    pub fn new(min_length: usize) -> Self {
        Self {
            min_length,
            priority: Priority::Medium,
            custom_message: None,
        }
    }
    
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.custom_message = Some(message.into());
        self
    }
}

impl Validator for MinLengthValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        let length = input.chars().count();
        if length < self.min_length {
            let default_message = format!(
                "Minimum length is {} characters (currently {})",
                self.min_length, length
            );
            let message = self.custom_message.as_deref().unwrap_or(&default_message);
            ValidationResult::failure("min_length", self.priority, message)
                .with_metadata("min_length", serde_json::Value::Number(self.min_length.into()))
                .with_metadata("actual_length", serde_json::Value::Number(length.into()))
        } else {
            ValidationResult::success("min_length")
        }
    }
    
    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        let length = input.chars().count();
        if length < self.min_length {
            PartialValidationResult::error_at(0)
                .with_suggestion(format!("Need {} more characters", self.min_length - length))
        } else {
            PartialValidationResult::valid()
        }
    }
    
    fn priority(&self) -> Priority {
        self.priority
    }
    
    fn name(&self) -> &str {
        "min_length"
    }
}

/// Validator for maximum length
#[derive(Debug)]
pub struct MaxLengthValidator {
    max_length: usize,
    priority: Priority,
    custom_message: Option<String>,
}

impl MaxLengthValidator {
    pub fn new(max_length: usize) -> Self {
        Self {
            max_length,
            priority: Priority::Medium,
            custom_message: None,
        }
    }
    
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.custom_message = Some(message.into());
        self
    }
}

impl Validator for MaxLengthValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        let length = input.chars().count();
        if length > self.max_length {
            let default_message = format!(
                "Maximum length is {} characters (currently {})",
                self.max_length, length
            );
            let message = self.custom_message.as_deref().unwrap_or(&default_message);
            ValidationResult::failure("max_length", self.priority, message)
                .with_metadata("max_length", serde_json::Value::Number(self.max_length.into()))
                .with_metadata("actual_length", serde_json::Value::Number(length.into()))
        } else {
            ValidationResult::success("max_length")
        }
    }
    
    fn partial_validate(&self, input: &str, cursor_pos: usize) -> PartialValidationResult {
        let length = input.chars().count();
        if length > self.max_length {
            // Find the position where it exceeds max length
            let error_pos = input.char_indices()
                .nth(self.max_length)
                .map(|(pos, _)| pos)
                .unwrap_or(cursor_pos);
            
            PartialValidationResult::error_at(error_pos)
                .with_suggestion(format!("Too long by {} characters", length - self.max_length))
        } else {
            PartialValidationResult::valid()
        }
    }
    
    fn priority(&self) -> Priority {
        self.priority
    }
    
    fn name(&self) -> &str {
        "max_length"
    }
}

/// Validator for regex patterns
#[derive(Debug)]
pub struct PatternValidator {
    pattern: Regex,
    pattern_str: String,
    priority: Priority,
    custom_message: Option<String>,
}

impl PatternValidator {
    pub fn new(pattern: impl AsRef<str>) -> Result<Self, regex::Error> {
        let pattern_str = pattern.as_ref().to_string();
        let pattern = Regex::new(&pattern_str)?;
        
        Ok(Self {
            pattern,
            pattern_str,
            priority: Priority::High,
            custom_message: None,
        }) 
    }
    
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.custom_message = Some(message.into());
        self
    }
}

impl Validator for PatternValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        if self.pattern.is_match(input) {
            ValidationResult::success("pattern")
        } else {
            let default_message = format!(
                "Must match pattern: {}",
                self.pattern_str
            );
            let message = self.custom_message.as_deref().unwrap_or(&default_message);
            ValidationResult::failure("pattern", self.priority, message)
                .with_metadata("pattern", serde_json::Value::String(self.pattern_str.clone()))
        }
    }
    
    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }
        
        // For partial validation, we try to find where the pattern first fails
        // This is a simple approach - we could make it more sophisticated
        if self.pattern.is_match(input) {
            PartialValidationResult::valid()
        } else {
            // Try to find first character that breaks the pattern
            for (i, _) in input.char_indices() {
                if !self.pattern.is_match(&input[..=i]) {
                    return PartialValidationResult::error_at(i);
                }
            }
            
            // If we can't find a specific position, mark from the beginning
            PartialValidationResult::error_at(0)
        }
    }
    
    fn priority(&self) -> Priority {
        self.priority
    }
    
    fn name(&self) -> &str {
        "pattern"
    }
    
    fn description(&self) -> &str {
        &self.pattern_str
    }
}