use super::super::{PartialValidationResult, Priority, ValidationResult, Validator};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};

/// Date validator with configurable format
#[derive(Debug)]
pub struct DateValidator {
    format: String,
    priority: Priority,
    custom_message: Option<String>,
}

impl DateValidator {
    pub fn new(format: Option<String>) -> Self {
        Self {
            format: format.unwrap_or_else(|| "%Y-%m-%d".to_string()),
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

impl Validator for DateValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        match NaiveDate::parse_from_str(input, &self.format) {
            Ok(_) => ValidationResult::success("date"),
            Err(_) => {
                let message = if let Some(msg) = &self.custom_message {
                    msg.clone()
                } else {
                    format!("Must be a valid date in format: {}", self.format)
                };
                ValidationResult::failure("date", self.priority, &message)
            }
        }
    }

    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }

        // Basic format checking for common patterns
        if self.format == "%Y-%m-%d" && input.len() <= 10 {
            // YYYY-MM-DD format
            let chars: Vec<char> = input.chars().collect();
            for (i, ch) in chars.iter().enumerate() {
                match i {
                    0..=3 => {
                        // Year digits
                        if !ch.is_ascii_digit() {
                            return PartialValidationResult::error_at(i);
                        }
                    }
                    4 | 7 => {
                        // Separators
                        if *ch != '-' {
                            return PartialValidationResult::error_at(i);
                        }
                    }
                    5..=6 | 8..=9 => {
                        // Month and day digits
                        if !ch.is_ascii_digit() {
                            return PartialValidationResult::error_at(i);
                        }
                    }
                    _ => {}
                }
            }
        }

        PartialValidationResult::valid()
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn name(&self) -> &str {
        "date"
    }
}

/// Time validator with configurable format
#[derive(Debug)]
pub struct TimeValidator {
    format: String,
    priority: Priority,
    custom_message: Option<String>,
}

impl TimeValidator {
    pub fn new(format: Option<String>) -> Self {
        Self {
            format: format.unwrap_or_else(|| "%H:%M:%S".to_string()),
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

impl Validator for TimeValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        match NaiveTime::parse_from_str(input, &self.format) {
            Ok(_) => ValidationResult::success("time"),
            Err(_) => {
                let message = if let Some(msg) = &self.custom_message {
                    msg.clone()
                } else {
                    format!("Must be a valid time in format: {}", self.format)
                };
                ValidationResult::failure("time", self.priority, &message)
            }
        }
    }

    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }

        // Basic format checking for common patterns
        if self.format == "%H:%M:%S" && input.len() <= 8 {
            // HH:MM:SS format
            let chars: Vec<char> = input.chars().collect();
            for (i, ch) in chars.iter().enumerate() {
                match i {
                    0..=1 | 3..=4 | 6..=7 => {
                        // Hour, minute, second digits
                        if !ch.is_ascii_digit() {
                            return PartialValidationResult::error_at(i);
                        }
                    }
                    2 | 5 => {
                        // Separators
                        if *ch != ':' {
                            return PartialValidationResult::error_at(i);
                        }
                    }
                    _ => {}
                }
            }
        } else if self.format == "%H:%M" && input.len() <= 5 {
            // HH:MM format
            let chars: Vec<char> = input.chars().collect();
            for (i, ch) in chars.iter().enumerate() {
                match i {
                    0..=1 | 3..=4 => {
                        // Hour, minute digits
                        if !ch.is_ascii_digit() {
                            return PartialValidationResult::error_at(i);
                        }
                    }
                    2 => {
                        // Separator
                        if *ch != ':' {
                            return PartialValidationResult::error_at(i);
                        }
                    }
                    _ => {}
                }
            }
        }

        PartialValidationResult::valid()
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn name(&self) -> &str {
        "time"
    }
}

/// DateTime validator with configurable format
#[derive(Debug)]
pub struct DateTimeValidator {
    format: String,
    priority: Priority,
    custom_message: Option<String>,
}

impl DateTimeValidator {
    pub fn new(format: Option<String>) -> Self {
        Self {
            format: format.unwrap_or_else(|| "%Y-%m-%d %H:%M:%S".to_string()),
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

impl Validator for DateTimeValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        // Try parsing as NaiveDateTime first
        if let Ok(_) = NaiveDateTime::parse_from_str(input, &self.format) {
            return ValidationResult::success("datetime");
        }

        // Try parsing as DateTime with timezone if format contains timezone info
        if self.format.contains("%z") || self.format.contains("%Z") || self.format.contains("%:z") {
            if let Ok(_) = DateTime::parse_from_str(input, &self.format) {
                return ValidationResult::success("datetime");
            }
        }

        let message = if let Some(msg) = &self.custom_message {
            msg.clone()
        } else {
            format!("Must be a valid datetime in format: {}", self.format)
        };
        ValidationResult::failure("datetime", self.priority, &message)
    }

    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }

        // Basic format checking for common pattern
        if self.format == "%Y-%m-%d %H:%M:%S" && input.len() <= 19 {
            // YYYY-MM-DD HH:MM:SS format
            let chars: Vec<char> = input.chars().collect();
            for (i, ch) in chars.iter().enumerate() {
                match i {
                    0..=3 => {
                        // Year digits
                        if !ch.is_ascii_digit() {
                            return PartialValidationResult::error_at(i);
                        }
                    }
                    4 | 7 => {
                        // Date separators
                        if *ch != '-' {
                            return PartialValidationResult::error_at(i);
                        }
                    }
                    5..=6 | 8..=9 => {
                        // Month and day digits
                        if !ch.is_ascii_digit() {
                            return PartialValidationResult::error_at(i);
                        }
                    }
                    10 => {
                        // Space between date and time
                        if *ch != ' ' {
                            return PartialValidationResult::error_at(i);
                        }
                    }
                    11..=12 | 14..=15 | 17..=18 => {
                        // Hour, minute, second digits
                        if !ch.is_ascii_digit() {
                            return PartialValidationResult::error_at(i);
                        }
                    }
                    13 | 16 => {
                        // Time separators
                        if *ch != ':' {
                            return PartialValidationResult::error_at(i);
                        }
                    }
                    _ => {}
                }
            }
        }

        PartialValidationResult::valid()
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn name(&self) -> &str {
        "datetime"
    }
}
