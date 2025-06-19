pub mod engine;
pub mod priority;
pub mod result;
pub mod rules;

pub use engine::ValidationEngine;
pub use priority::Priority;
pub use result::{PartialValidationResult, ValidationResult, ValidationSummary};

use crate::error::Result;

/// Core trait for all validators
pub trait Validator: Send + Sync {
    /// Validate complete input
    fn validate(&self, input: &str) -> ValidationResult;

    /// Validate partial input during typing
    fn partial_validate(&self, input: &str, cursor_pos: usize) -> PartialValidationResult;

    /// Get the priority of this validator
    fn priority(&self) -> Priority;

    /// Get the name/identifier of this validator
    fn name(&self) -> &str;

    /// Get human-readable description of this validator
    fn description(&self) -> &str {
        self.name()
    }
}

/// Configuration for a validation rule
#[derive(Debug, Clone)]
pub struct ValidationRuleConfig {
    pub validator_type: ValidatorType,
    pub priority: Option<Priority>,
    pub custom_message: Option<String>,
    pub parameters: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum ValidatorType {
    Required,
    MinLength(usize),
    MaxLength(usize),
    Pattern(String),
    Email,
    Hostname,
    Url,
    Ipv4,
    Ipv6,
    Integer,
    Float,
    Range(f64, f64),
    Positive,
    Negative,
    Date(Option<String>),
    Time(Option<String>),
    DateTime(Option<String>),
    Choices(Vec<String>),
    FileExists,
    DirExists,
    PathExists,
    Readable,
    Writable,
    Executable,
}
