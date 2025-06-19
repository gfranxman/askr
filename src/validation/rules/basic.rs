use super::super::{PartialValidationResult, Priority, ValidationResult, Validator};
use once_cell::sync::Lazy;
use regex::Regex;

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
                self.custom_message
                    .as_deref()
                    .unwrap_or("This field is required"),
            )
        } else {
            ValidationResult::success_with_priority("required", Priority::Critical)
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
                .with_metadata(
                    "min_length",
                    serde_json::Value::Number(self.min_length.into()),
                )
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
                .with_metadata(
                    "max_length",
                    serde_json::Value::Number(self.max_length.into()),
                )
                .with_metadata("actual_length", serde_json::Value::Number(length.into()))
        } else {
            ValidationResult::success("max_length")
        }
    }

    fn partial_validate(&self, input: &str, cursor_pos: usize) -> PartialValidationResult {
        let length = input.chars().count();
        if length > self.max_length {
            // Find the position where it exceeds max length
            let error_pos = input
                .char_indices()
                .nth(self.max_length)
                .map(|(pos, _)| pos)
                .unwrap_or(cursor_pos);

            PartialValidationResult::error_at(error_pos).with_suggestion(format!(
                "Too long by {} characters",
                length - self.max_length
            ))
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
            let default_message = format!("Must match pattern: {}", self.pattern_str);
            let message = self.custom_message.as_deref().unwrap_or(&default_message);
            ValidationResult::failure("pattern", self.priority, message).with_metadata(
                "pattern",
                serde_json::Value::String(self.pattern_str.clone()),
            )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_validator_valid_input() {
        let validator = RequiredValidator::new();

        let result = validator.validate("hello");
        assert!(result.passed);
        assert_eq!(result.rule_name, "required");

        let result = validator.validate("  hello  ");
        assert!(result.passed);
    }

    #[test]
    fn test_required_validator_invalid_input() {
        let validator = RequiredValidator::new();

        let result = validator.validate("");
        assert!(!result.passed);
        assert_eq!(result.message.unwrap(), "This field is required");
        assert_eq!(result.priority, Priority::Critical);

        let result = validator.validate("   ");
        assert!(!result.passed);

        let result = validator.validate("\t\n  ");
        assert!(!result.passed);
    }

    #[test]
    fn test_required_validator_custom_message() {
        let validator = RequiredValidator::with_message("Please enter a value");

        let result = validator.validate("");
        assert!(!result.passed);
        assert_eq!(result.message.unwrap(), "Please enter a value");
    }

    #[test]
    fn test_required_validator_partial_validation() {
        let validator = RequiredValidator::new();

        let result = validator.partial_validate("", 0);
        assert_eq!(result.first_error_pos, Some(0));

        let result = validator.partial_validate("   ", 2);
        assert_eq!(result.first_error_pos, Some(0));

        let result = validator.partial_validate("hello", 3);
        assert!(result.first_error_pos.is_none());
    }

    #[test]
    fn test_min_length_validator_valid_input() {
        let validator = MinLengthValidator::new(5);

        let result = validator.validate("hello");
        assert!(result.passed);

        let result = validator.validate("hello world");
        assert!(result.passed);

        let result = validator.validate("12345");
        assert!(result.passed);
    }

    #[test]
    fn test_min_length_validator_invalid_input() {
        let validator = MinLengthValidator::new(5);

        let result = validator.validate("hi");
        assert!(!result.passed);
        let message = result.message.unwrap();
        assert!(message.contains("Minimum length is 5"));
        assert!(message.contains("currently 2"));

        let result = validator.validate("");
        assert!(!result.passed);
        assert!(result.message.unwrap().contains("currently 0"));
    }

    #[test]
    fn test_min_length_validator_custom_message() {
        let validator = MinLengthValidator::new(3).with_message("Too short!");

        let result = validator.validate("hi");
        assert!(!result.passed);
        assert_eq!(result.message.unwrap(), "Too short!");
    }

    #[test]
    fn test_min_length_validator_priority() {
        let validator = MinLengthValidator::new(5).with_priority(Priority::Low);

        let result = validator.validate("hi");
        assert!(!result.passed);
        assert_eq!(result.priority, Priority::Low);
    }

    #[test]
    fn test_min_length_validator_metadata() {
        let validator = MinLengthValidator::new(5);

        let result = validator.validate("hi");
        assert!(!result.passed);

        // Check metadata
        assert_eq!(
            result.metadata.get("min_length").unwrap(),
            &serde_json::Value::Number(5.into())
        );
        assert_eq!(
            result.metadata.get("actual_length").unwrap(),
            &serde_json::Value::Number(2.into())
        );
    }

    #[test]
    fn test_max_length_validator_valid_input() {
        let validator = MaxLengthValidator::new(5);

        let result = validator.validate("hello");
        assert!(result.passed);

        let result = validator.validate("hi");
        assert!(result.passed);

        let result = validator.validate("");
        assert!(result.passed);
    }

    #[test]
    fn test_max_length_validator_invalid_input() {
        let validator = MaxLengthValidator::new(5);

        let result = validator.validate("hello world");
        assert!(!result.passed);
        let message = result.message.unwrap();
        assert!(message.contains("Maximum length is 5"));
        assert!(message.contains("currently 11"));
    }

    #[test]
    fn test_max_length_validator_partial_validation() {
        let validator = MaxLengthValidator::new(5);

        let result = validator.partial_validate("hello", 5);
        assert!(result.first_error_pos.is_none());

        let result = validator.partial_validate("hello world", 8);
        assert!(result.first_error_pos.is_some());
        assert!(result.suggestion.is_some());
        let suggestion = result.suggestion.unwrap();
        assert!(suggestion.contains("Too long by"));
    }

    #[test]
    fn test_pattern_validator_valid_input() {
        let validator = PatternValidator::new(r"^\d{3}-\d{3}-\d{4}$").unwrap();

        let result = validator.validate("123-456-7890");
        assert!(result.passed);

        let result = validator.validate("999-999-9999");
        assert!(result.passed);
    }

    #[test]
    fn test_pattern_validator_invalid_input() {
        let validator = PatternValidator::new(r"^\d{3}-\d{3}-\d{4}$").unwrap();

        let result = validator.validate("123-45-6789");
        assert!(!result.passed);

        let result = validator.validate("not-a-phone");
        assert!(!result.passed);

        let result = validator.validate("1234567890");
        assert!(!result.passed);
    }

    #[test]
    fn test_pattern_validator_custom_message() {
        let validator = PatternValidator::new(r"^\d+$")
            .unwrap()
            .with_message("Must contain only digits");

        let result = validator.validate("abc123");
        assert!(!result.passed);
        assert_eq!(result.message.unwrap(), "Must contain only digits");
    }

    #[test]
    fn test_pattern_validator_metadata() {
        let validator = PatternValidator::new(r"^\d+$").unwrap();

        let result = validator.validate("abc");
        assert!(!result.passed);

        assert_eq!(
            result.metadata.get("pattern").unwrap(),
            &serde_json::Value::String(r"^\d+$".to_string())
        );
    }

    #[test]
    fn test_unicode_handling() {
        // Test with emojis
        let min_validator = MinLengthValidator::new(3);
        let result = min_validator.validate("👋🌍🚀");
        assert!(result.passed);

        let max_validator = MaxLengthValidator::new(2);
        let result = max_validator.validate("👋🌍🚀");
        assert!(!result.passed);

        // Test with combining characters
        let result = min_validator.validate("é́́"); // e + combining acute accent + combining acute accent
        assert!(result.passed);

        // Test with various Unicode scripts
        let result = min_validator.validate("こんにちは"); // Japanese
        assert!(result.passed);

        let result = min_validator.validate("مرحبا"); // Arabic
        assert!(result.passed);
    }

    #[test]
    fn test_edge_cases() {
        let min_validator = MinLengthValidator::new(0);
        assert!(min_validator.validate("").passed);
        assert!(min_validator.validate("anything").passed);

        let max_validator = MaxLengthValidator::new(0);
        assert!(max_validator.validate("").passed);
        assert!(!max_validator.validate("x").passed);
    }

    #[test]
    fn test_pattern_validator_edge_cases() {
        // Test empty pattern (matches empty string only)
        let validator = PatternValidator::new("^$").unwrap();
        assert!(validator.validate("").passed);
        assert!(!validator.validate("anything").passed);

        // Test pattern that matches anything
        let validator = PatternValidator::new(".*").unwrap();
        assert!(validator.validate("").passed);
        assert!(validator.validate("anything").passed);

        // Test complex Unicode pattern
        let validator = PatternValidator::new(r"^[\p{L}\p{N}]+$").unwrap();
        assert!(validator.validate("hello123").passed);
        assert!(validator.validate("مرحبا123").passed);
        assert!(!validator.validate("hello!").passed);
    }

    #[test]
    fn test_invalid_regex_pattern() {
        let result = PatternValidator::new("[");
        assert!(result.is_err());
    }

    #[test]
    fn test_validator_names() {
        assert_eq!(RequiredValidator::new().name(), "required");
        assert_eq!(MinLengthValidator::new(5).name(), "min_length");
        assert_eq!(MaxLengthValidator::new(5).name(), "max_length");
        assert_eq!(PatternValidator::new(r"\d+").unwrap().name(), "pattern");
    }

    #[test]
    fn test_validator_priorities() {
        assert_eq!(RequiredValidator::new().priority(), Priority::Critical);
        assert_eq!(MinLengthValidator::new(5).priority(), Priority::Medium);
        assert_eq!(MaxLengthValidator::new(5).priority(), Priority::Medium);
        assert_eq!(
            PatternValidator::new(r"\d+").unwrap().priority(),
            Priority::High
        );
    }

    #[test]
    fn test_chain_configuration() {
        let validator = MinLengthValidator::new(5)
            .with_priority(Priority::Low)
            .with_message("Custom message");

        assert_eq!(validator.priority(), Priority::Low);

        let result = validator.validate("hi");
        assert!(!result.passed);
        assert_eq!(result.message.unwrap(), "Custom message");
    }
}
