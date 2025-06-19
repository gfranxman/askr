use super::{PartialValidationResult, Priority, ValidationResult, ValidationSummary, Validator};
use dashmap::DashMap;
use std::time::Instant;

/// Main validation engine that orchestrates multiple validators
pub struct ValidationEngine {
    validators: Vec<Box<dyn Validator>>,
    cache: Option<DashMap<String, Vec<ValidationResult>>>,
    cache_enabled: bool,
}

impl ValidationEngine {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
            cache: Some(DashMap::new()),
            cache_enabled: true,
        }
    }

    pub fn without_cache() -> Self {
        Self {
            validators: Vec::new(),
            cache: None,
            cache_enabled: false,
        }
    }

    pub fn add_validator(&mut self, validator: Box<dyn Validator>) {
        self.validators.push(validator);
        // Clear cache when validators change
        if let Some(cache) = &self.cache {
            cache.clear();
        }
    }

    pub fn validate(&self, input: &str) -> ValidationSummary {
        let start_time = Instant::now();

        // Check cache first
        if let Some(cached) = self.get_cached_results(input) {
            let mut summary = ValidationSummary::new(input.to_string(), cached);
            summary.metadata.validation_time_ms = start_time.elapsed().as_millis() as u64;
            return summary;
        }

        // Run all validators
        let results: Vec<ValidationResult> = self
            .validators
            .iter()
            .map(|validator| validator.validate(input))
            .collect();

        // Cache results
        self.cache_results(input, &results);

        let mut summary = ValidationSummary::new(input.to_string(), results);
        summary.metadata.validation_time_ms = start_time.elapsed().as_millis() as u64;
        summary
    }

    pub fn partial_validate(&self, input: &str, cursor_pos: usize) -> PartialValidationResult {
        // For partial validation, we find the first error position across all validators
        let mut first_error_pos: Option<usize> = None;
        let mut can_continue = true;
        let mut suggestions = Vec::new();

        for validator in &self.validators {
            let result = validator.partial_validate(input, cursor_pos);

            // Track the earliest error position
            if let Some(pos) = result.first_error_pos {
                first_error_pos = Some(match first_error_pos {
                    Some(existing) => existing.min(pos),
                    None => pos,
                });
            }

            // If any validator blocks continuation, block overall
            if !result.can_continue {
                can_continue = false;
            }

            // Collect suggestions
            if let Some(suggestion) = result.suggestion {
                suggestions.push(suggestion);
            }
        }

        PartialValidationResult {
            first_error_pos,
            can_continue,
            suggestion: if suggestions.is_empty() {
                None
            } else {
                Some(suggestions.join("; "))
            },
        }
    }

    pub fn get_display_errors(
        &self,
        input: &str,
        max_errors: Option<usize>,
    ) -> Vec<ValidationResult> {
        let summary = self.validate(input);
        self.filter_display_errors(summary.validation_results, max_errors)
    }

    fn filter_display_errors(
        &self,
        mut results: Vec<ValidationResult>,
        max_errors: Option<usize>,
    ) -> Vec<ValidationResult> {
        // Sort by priority (critical first) then by rule order
        results.sort_by(|a, b| {
            a.priority
                .cmp(&b.priority)
                .then_with(|| a.rule_name.cmp(&b.rule_name))
        });

        // Filter failed validations and apply display rules
        let failed: Vec<_> = results.into_iter().filter(|r| !r.passed).collect();

        let mut display_errors = Vec::new();
        let mut critical_high_count = 0;
        let mut medium_count = 0;
        let mut low_count = 0;

        for error in failed {
            let should_include = match error.priority {
                Priority::Critical | Priority::High => {
                    critical_high_count += 1;
                    true // Always show critical and high priority
                }
                Priority::Medium => {
                    medium_count += 1;
                    medium_count <= 3 // Show up to 3 medium priority errors
                }
                Priority::Low => {
                    low_count += 1;
                    // Only show low priority if no critical/high errors and max 2
                    critical_high_count == 0 && low_count <= 2
                }
            };

            if should_include {
                display_errors.push(error);

                // Apply overall max_errors limit if specified
                if let Some(max) = max_errors {
                    if display_errors.len() >= max {
                        break;
                    }
                }
            }
        }

        display_errors
    }

    fn get_cached_results(&self, input: &str) -> Option<Vec<ValidationResult>> {
        if !self.cache_enabled {
            return None;
        }

        self.cache.as_ref()?.get(input).map(|entry| entry.clone())
    }

    fn cache_results(&self, input: &str, results: &[ValidationResult]) {
        if let Some(cache) = &self.cache {
            cache.insert(input.to_string(), results.to_vec());
        }
    }

    pub fn clear_cache(&self) {
        if let Some(cache) = &self.cache {
            cache.clear();
        }
    }

    pub fn validator_count(&self) -> usize {
        self.validators.len()
    }

    /// Get all potential error messages to calculate display space needed
    pub fn get_potential_error_messages(&self) -> Vec<String> {
        let mut messages = Vec::new();

        // Get default error messages from each validator by testing with invalid input
        for validator in &self.validators {
            // Try various test inputs to trigger different error conditions
            let test_inputs = vec![
                "", // Empty input
                "x", // Minimal input  
                "this is a very long input string that will likely fail most validators and show their error messages",
                "invalid-format-123!@#", // Invalid format
            ];

            for test_input in test_inputs {
                let result = validator.validate(test_input);
                if !result.passed {
                    if let Some(message) = &result.message {
                        if !messages.contains(message) {
                            messages.push(message.clone());
                        }
                    }
                }
            }
        }

        // Add some buffer for dynamic messages
        if !messages.is_empty() {
            // Add a few generic buffer messages for dynamic content
            messages.push("Additional validation context may appear here".to_string());
        }

        messages
    }
}

impl Default for ValidationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::rules::basic::{
        MaxLengthValidator, MinLengthValidator, RequiredValidator,
    };

    #[test]
    fn test_empty_validation_engine() {
        let engine = ValidationEngine::new();
        let summary = engine.validate("test");

        assert!(summary.valid);
        assert!(summary.validation_results.is_empty());
        assert!(summary.error.is_none());
    }

    #[test]
    fn test_single_validator() {
        let mut engine = ValidationEngine::new();
        engine.add_validator(Box::new(RequiredValidator::new()));

        // Test valid input
        let summary = engine.validate("hello");
        assert!(summary.valid);
        assert_eq!(summary.validation_results.len(), 1);
        assert!(summary.validation_results[0].passed);

        // Test invalid input
        let summary = engine.validate("");
        assert!(!summary.valid);
        assert_eq!(summary.validation_results.len(), 1);
        assert!(!summary.validation_results[0].passed);
        assert!(summary.error.is_some());
    }

    #[test]
    fn test_multiple_validators() {
        let mut engine = ValidationEngine::new();
        engine.add_validator(Box::new(RequiredValidator::new()));
        engine.add_validator(Box::new(MinLengthValidator::new(5)));
        engine.add_validator(Box::new(MaxLengthValidator::new(10)));

        // Test all validators pass
        let summary = engine.validate("hello");
        assert!(summary.valid);
        assert_eq!(summary.validation_results.len(), 3);
        assert!(summary.validation_results.iter().all(|r| r.passed));

        // Test some validators fail
        let summary = engine.validate("hi");
        assert!(!summary.valid);
        assert_eq!(summary.validation_results.len(), 3);

        // After sorting: required (Critical), max_length (Medium), min_length (Medium)
        assert!(summary.validation_results[0].passed); // required
        assert!(summary.validation_results[1].passed); // max_length (passes for "hi")
        assert!(!summary.validation_results[2].passed); // min_length (fails for "hi")
    }

    #[test]
    fn test_priority_sorting() {
        let mut engine = ValidationEngine::new();

        // Add validators in non-priority order
        engine.add_validator(Box::new(MinLengthValidator::new(10))); // Medium priority
        engine.add_validator(Box::new(RequiredValidator::new())); // Critical priority

        let summary = engine.validate("");
        assert!(!summary.valid);

        // Results should be sorted by priority (Critical first)
        assert_eq!(summary.validation_results[0].rule_name, "required");
        assert_eq!(summary.validation_results[1].rule_name, "min_length");
    }

    #[test]
    fn test_error_display_filtering() {
        let mut engine = ValidationEngine::new();
        engine.add_validator(Box::new(RequiredValidator::new()));
        engine.add_validator(Box::new(MinLengthValidator::new(5)));

        let errors = engine.get_display_errors("", Some(1));

        // Should only show 1 error (the most critical)
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].rule_name, "required");
        assert_eq!(errors[0].priority, Priority::Critical);
    }

    #[test]
    fn test_partial_validation() {
        let mut engine = ValidationEngine::new();
        engine.add_validator(Box::new(MaxLengthValidator::new(5)));

        // Test valid partial input
        let result = engine.partial_validate("hello", 5);
        assert!(result.first_error_pos.is_none());
        assert!(result.can_continue);

        // Test invalid partial input
        let result = engine.partial_validate("hello world", 8);
        assert!(result.first_error_pos.is_some());
        assert_eq!(result.first_error_pos.unwrap(), 5);
    }

    #[test]
    fn test_validation_performance_timing() {
        let mut engine = ValidationEngine::new();
        engine.add_validator(Box::new(RequiredValidator::new()));

        let summary = engine.validate("test");

        // Should record timing metadata
        assert!(summary.metadata.validation_time_ms < 1000); // Should be very fast
    }

    #[test]
    fn test_get_potential_error_messages() {
        let mut engine = ValidationEngine::new();
        engine.add_validator(Box::new(RequiredValidator::new()));
        engine.add_validator(Box::new(MinLengthValidator::new(5)));

        let messages = engine.get_potential_error_messages();

        // Should contain error messages from all validators
        assert!(!messages.is_empty());
        assert!(messages.iter().any(|msg| msg.contains("required")));
        assert!(messages.iter().any(|msg| msg.contains("Minimum length")));
    }

    #[test]
    fn test_validation_engine_without_cache() {
        let engine = ValidationEngine::without_cache();

        // Should still work without cache
        let summary = engine.validate("test");
        assert!(summary.valid);
    }

    #[test]
    fn test_validation_caching() {
        let mut engine = ValidationEngine::new();
        engine.add_validator(Box::new(RequiredValidator::new()));

        // First validation
        let summary1 = engine.validate("test");
        let time1 = summary1.metadata.validation_time_ms;

        // Second validation of same input (should use cache)
        let summary2 = engine.validate("test");
        let time2 = summary2.metadata.validation_time_ms;

        // Results should be identical
        assert_eq!(summary1.valid, summary2.valid);
        assert_eq!(
            summary1.validation_results.len(),
            summary2.validation_results.len()
        );

        // Second call might be faster due to caching
        assert!(time2 <= time1 + 1); // Allow for timing variations
    }

    #[test]
    fn test_cache_invalidation() {
        let mut engine = ValidationEngine::new();
        engine.add_validator(Box::new(RequiredValidator::new()));

        // Cache some results
        let _summary1 = engine.validate("test");

        // Add another validator (should clear cache)
        engine.add_validator(Box::new(MinLengthValidator::new(5)));

        // Validation should still work correctly
        let summary2 = engine.validate("testing");
        assert!(summary2.valid);
        assert_eq!(summary2.validation_results.len(), 2);
    }

    #[test]
    fn test_clear_cache() {
        let mut engine = ValidationEngine::new();
        engine.add_validator(Box::new(RequiredValidator::new()));

        // Cache some results
        let _summary = engine.validate("test");

        // Clear cache manually
        engine.clear_cache();

        // Should still work
        let summary = engine.validate("test");
        assert!(summary.valid);
    }

    #[test]
    fn test_validator_count() {
        let mut engine = ValidationEngine::new();
        assert_eq!(engine.validator_count(), 0);

        engine.add_validator(Box::new(RequiredValidator::new()));
        assert_eq!(engine.validator_count(), 1);

        engine.add_validator(Box::new(MinLengthValidator::new(5)));
        assert_eq!(engine.validator_count(), 2);
    }
}
