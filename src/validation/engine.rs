use std::collections::HashMap;
use std::time::Instant;
use dashmap::DashMap;
use super::{Validator, ValidationResult, PartialValidationResult, ValidationSummary, Priority};
use crate::error::Result;

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
        let results: Vec<ValidationResult> = self.validators
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
    
    pub fn get_display_errors(&self, input: &str, max_errors: Option<usize>) -> Vec<ValidationResult> {
        let summary = self.validate(input);
        self.filter_display_errors(summary.validation_results, max_errors)
    }
    
    fn filter_display_errors(&self, mut results: Vec<ValidationResult>, max_errors: Option<usize>) -> Vec<ValidationResult> {
        // Sort by priority (critical first) then by rule order
        results.sort_by(|a, b| {
            a.priority.cmp(&b.priority)
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
}

impl Default for ValidationEngine {
    fn default() -> Self {
        Self::new()
    }
}