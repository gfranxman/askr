use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::Priority;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub rule_name: String,
    pub passed: bool,
    pub priority: Priority,
    pub message: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ValidationResult {
    pub fn success(rule_name: impl Into<String>) -> Self {
        Self {
            rule_name: rule_name.into(),
            passed: true,
            priority: Priority::Medium,
            message: None,
            metadata: HashMap::new(),
        }
    }
    
    pub fn success_with_priority(rule_name: impl Into<String>, priority: Priority) -> Self {
        Self {
            rule_name: rule_name.into(),
            passed: true,
            priority,
            message: None,
            metadata: HashMap::new(),
        }
    }
    
    pub fn failure(rule_name: impl Into<String>, priority: Priority, message: impl Into<String>) -> Self {
        Self {
            rule_name: rule_name.into(),
            passed: false,
            priority,
            message: Some(message.into()),
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PartialValidationResult {
    pub first_error_pos: Option<usize>,
    pub can_continue: bool,
    pub suggestion: Option<String>,
}

impl PartialValidationResult {
    pub fn valid() -> Self {
        Self {
            first_error_pos: None,
            can_continue: true,
            suggestion: None,
        }
    }
    
    pub fn error_at(pos: usize) -> Self {
        Self {
            first_error_pos: Some(pos),
            can_continue: true,
            suggestion: None,
        }
    }
    
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
    
    pub fn blocking(mut self) -> Self {
        self.can_continue = false;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub value: String,
    pub valid: bool,
    pub error: Option<String>,
    pub metadata: ValidationMetadata,
    pub validation_results: Vec<ValidationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationMetadata {
    pub validation_time_ms: u64,
    pub rules_checked: usize,
    pub rules_passed: usize,
    pub input_length: usize,
    pub attempts: Option<u32>,
}

impl ValidationSummary {
    pub fn new(value: String, mut results: Vec<ValidationResult>) -> Self {
        // Sort results by priority (Critical first)
        results.sort_by(|a, b| {
            a.priority.cmp(&b.priority)
                .then_with(|| a.rule_name.cmp(&b.rule_name))
        });
        
        let valid = results.iter().all(|r| r.passed);
        let error = if valid {
            None
        } else {
            results.iter()
                .filter(|r| !r.passed)
                .min_by_key(|r| r.priority)
                .and_then(|r| r.message.as_ref())
                .cloned()
        };
        
        let rules_passed = results.iter().filter(|r| r.passed).count();
        
        Self {
            value: value.clone(),
            valid,
            error,
            metadata: ValidationMetadata {
                validation_time_ms: 0, // Will be set by validation engine
                rules_checked: results.len(),
                rules_passed,
                input_length: value.len(),
                attempts: None,
            },
            validation_results: results,
        }
    }
}