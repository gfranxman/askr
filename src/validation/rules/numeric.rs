use super::super::{PartialValidationResult, Priority, ValidationResult, Validator};
use std::str::FromStr;

/// Integer validator
#[derive(Debug)]
pub struct IntegerValidator {
    priority: Priority,
    custom_message: Option<String>,
}

impl IntegerValidator {
    pub fn new() -> Self {
        Self {
            priority: Priority::High,
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

impl Validator for IntegerValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        if input.parse::<i64>().is_ok() {
            ValidationResult::success("integer")
        } else {
            let message = self
                .custom_message
                .as_deref()
                .unwrap_or("Must be a valid integer");
            ValidationResult::failure("integer", self.priority, message)
        }
    }

    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }

        // Check for invalid characters
        for (i, ch) in input.char_indices() {
            if i == 0 && (ch == '+' || ch == '-') {
                continue; // Allow sign at start
            }
            if !ch.is_ascii_digit() {
                return PartialValidationResult::error_at(i);
            }
        }

        PartialValidationResult::valid()
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn name(&self) -> &str {
        "integer"
    }
}

/// Float validator
#[derive(Debug)]
pub struct FloatValidator {
    priority: Priority,
    custom_message: Option<String>,
}

impl FloatValidator {
    pub fn new() -> Self {
        Self {
            priority: Priority::High,
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

impl Validator for FloatValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        if input.parse::<f64>().is_ok() {
            ValidationResult::success("float")
        } else {
            let message = self
                .custom_message
                .as_deref()
                .unwrap_or("Must be a valid number");
            ValidationResult::failure("float", self.priority, message)
        }
    }

    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }

        let mut has_dot = false;
        let mut has_e = false;

        for (i, ch) in input.char_indices() {
            match ch {
                '+' | '-' => {
                    // Allow sign at start or after 'e'/'E'
                    if i != 0 {
                        if let Some(prev_char) = input.chars().nth(i - 1) {
                            if prev_char != 'e' && prev_char != 'E' {
                                return PartialValidationResult::error_at(i);
                            }
                        } else {
                            return PartialValidationResult::error_at(i);
                        }
                    }
                }
                '.' => {
                    if has_dot || has_e {
                        return PartialValidationResult::error_at(i);
                    }
                    has_dot = true;
                }
                'e' | 'E' => {
                    if has_e {
                        return PartialValidationResult::error_at(i);
                    }
                    has_e = true;
                }
                '0'..='9' => {
                    // Valid digit
                }
                _ => {
                    return PartialValidationResult::error_at(i);
                }
            }
        }

        PartialValidationResult::valid()
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn name(&self) -> &str {
        "float"
    }
}

/// Range validator for numeric values
#[derive(Debug)]
pub struct RangeValidator {
    min: Option<f64>,
    max: Option<f64>,
    priority: Priority,
    custom_message: Option<String>,
}

impl RangeValidator {
    pub fn new(min: Option<f64>, max: Option<f64>) -> Self {
        Self {
            min,
            max,
            priority: Priority::Medium,
            custom_message: None,
        }
    }

    pub fn min_only(min: f64) -> Self {
        Self::new(Some(min), None)
    }

    pub fn max_only(max: f64) -> Self {
        Self::new(None, Some(max))
    }

    pub fn between(min: f64, max: f64) -> Self {
        Self::new(Some(min), Some(max))
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

impl Validator for RangeValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        let value = match input.parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                let message = self
                    .custom_message
                    .as_deref()
                    .unwrap_or("Must be a valid number");
                return ValidationResult::failure("range", self.priority, message);
            }
        };

        let mut errors = Vec::new();

        if let Some(min) = self.min {
            if value < min {
                errors.push(format!("must be at least {}", min));
            }
        }

        if let Some(max) = self.max {
            if value > max {
                errors.push(format!("must be at most {}", max));
            }
        }

        if errors.is_empty() {
            ValidationResult::success("range")
        } else {
            let message = if let Some(msg) = &self.custom_message {
                msg.clone()
            } else {
                match (self.min, self.max) {
                    (Some(min), Some(max)) => format!("Must be between {} and {}", min, max),
                    (Some(min), None) => format!("Must be at least {}", min),
                    (None, Some(max)) => format!("Must be at most {}", max),
                    (None, None) => "Must be a valid number".to_string(),
                }
            };
            ValidationResult::failure("range", self.priority, &message)
        }
    }

    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        // Use float validation for partial validation
        let float_validator = FloatValidator::new();
        float_validator.partial_validate(input, _cursor_pos)
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn name(&self) -> &str {
        "range"
    }
}

/// Positive number validator
#[derive(Debug)]
pub struct PositiveValidator {
    priority: Priority,
    custom_message: Option<String>,
}

impl PositiveValidator {
    pub fn new() -> Self {
        Self {
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

impl Validator for PositiveValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        let value = match input.parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                let message = self
                    .custom_message
                    .as_deref()
                    .unwrap_or("Must be a valid number");
                return ValidationResult::failure("positive", self.priority, message);
            }
        };

        if value > 0.0 {
            ValidationResult::success("positive")
        } else {
            let message = self
                .custom_message
                .as_deref()
                .unwrap_or("Must be a positive number");
            ValidationResult::failure("positive", self.priority, message)
        }
    }

    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }

        // Don't allow negative sign at start
        if input.starts_with('-') {
            return PartialValidationResult::error_at(0);
        }

        // Use float validation for the rest
        let float_validator = FloatValidator::new();
        float_validator.partial_validate(input, _cursor_pos)
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn name(&self) -> &str {
        "positive"
    }
}

/// Negative number validator
#[derive(Debug)]
pub struct NegativeValidator {
    priority: Priority,
    custom_message: Option<String>,
}

impl NegativeValidator {
    pub fn new() -> Self {
        Self {
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

impl Validator for NegativeValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        let value = match input.parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                let message = self
                    .custom_message
                    .as_deref()
                    .unwrap_or("Must be a valid number");
                return ValidationResult::failure("negative", self.priority, message);
            }
        };

        if value < 0.0 {
            ValidationResult::success("negative")
        } else {
            let message = self
                .custom_message
                .as_deref()
                .unwrap_or("Must be a negative number");
            ValidationResult::failure("negative", self.priority, message)
        }
    }

    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }

        // Must start with negative sign for negative numbers
        if !input.starts_with('-') && input.len() > 0 {
            // Allow typing the minus sign first
            if input.chars().all(|c| c.is_ascii_digit() || c == '.') {
                return PartialValidationResult::valid(); // User might type minus later
            }
        }

        // Use float validation for the rest
        let float_validator = FloatValidator::new();
        float_validator.partial_validate(input, _cursor_pos)
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn name(&self) -> &str {
        "negative"
    }
}
