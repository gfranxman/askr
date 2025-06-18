use super::super::{Validator, ValidationResult, PartialValidationResult, Priority};
use std::collections::HashSet;

/// Choice validator for selecting from predefined options
#[derive(Debug)]
pub struct ChoiceValidator {
    choices: Vec<String>,
    case_sensitive: bool,
    min_choices: usize,
    max_choices: usize,
    priority: Priority,
    custom_message: Option<String>,
}

impl ChoiceValidator {
    pub fn new(choices: Vec<String>) -> Self {
        Self {
            choices,
            case_sensitive: false,
            min_choices: 1,
            max_choices: 1,
            priority: Priority::High,
            custom_message: None,
        }
    }
    
    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = case_sensitive;
        self
    }
    
    pub fn min_choices(mut self, min: usize) -> Self {
        self.min_choices = min;
        self
    }
    
    pub fn max_choices(mut self, max: usize) -> Self {
        self.max_choices = max;
        self
    }
    
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.custom_message = Some(message.into());
        self
    }
    
    /// Parse input for multiple choices (comma-separated)
    fn parse_input(&self, input: &str) -> Vec<String> {
        if self.max_choices == 1 {
            vec![input.trim().to_string()]
        } else {
            input
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        }
    }
    
    /// Check if a choice matches any of the valid options
    fn is_valid_choice(&self, choice: &str) -> bool {
        if self.case_sensitive {
            self.choices.contains(&choice.to_string())
        } else {
            let choice_lower = choice.to_lowercase();
            self.choices.iter().any(|c| c.to_lowercase() == choice_lower)
        }
    }
    
    /// Get the canonical form of a choice (with correct case)
    fn get_canonical_choice(&self, choice: &str) -> Option<String> {
        if self.case_sensitive {
            if self.choices.contains(&choice.to_string()) {
                Some(choice.to_string())
            } else {
                None
            }
        } else {
            let choice_lower = choice.to_lowercase();
            self.choices
                .iter()
                .find(|c| c.to_lowercase() == choice_lower)
                .cloned()
        }
    }
}

impl Validator for ChoiceValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        let parsed_choices = self.parse_input(input);
        
        // Check choice count
        if parsed_choices.len() < self.min_choices {
            let message = if let Some(msg) = &self.custom_message {
                msg.clone()
            } else {
                format!("At least {} choice(s) required", self.min_choices)
            };
            return ValidationResult::failure("choice", self.priority, &message);
        }
        
        if parsed_choices.len() > self.max_choices {
            let message = if let Some(msg) = &self.custom_message {
                msg.clone()
            } else {
                format!("At most {} choice(s) allowed", self.max_choices)
            };
            return ValidationResult::failure("choice", self.priority, &message);
        }
        
        // Check for duplicates
        let mut seen = HashSet::new();
        let mut duplicates = Vec::new();
        
        for choice in &parsed_choices {
            if let Some(canonical) = self.get_canonical_choice(choice) {
                if !seen.insert(canonical.clone()) {
                    duplicates.push(canonical);
                }
            }
        }
        
        if !duplicates.is_empty() {
            let message = if let Some(msg) = &self.custom_message {
                msg.clone()
            } else {
                format!("Duplicate choices not allowed: {}", duplicates.join(", "))
            };
            return ValidationResult::failure("choice", self.priority, &message);
        }
        
        // Check each choice validity
        let mut invalid_choices = Vec::new();
        for choice in &parsed_choices {
            if !self.is_valid_choice(choice) {
                invalid_choices.push(choice.clone());
            }
        }
        
        if !invalid_choices.is_empty() {
            let message = if let Some(msg) = &self.custom_message {
                msg.clone()
            } else {
                let valid_choices_str = self.choices.join(", ");
                format!(
                    "Invalid choice(s): {}. Valid options: {}",
                    invalid_choices.join(", "),
                    valid_choices_str
                )
            };
            return ValidationResult::failure("choice", self.priority, &message);
        }
        
        ValidationResult::success("choice")
    }
    
    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }
        
        // For single choice, check partial matching
        if self.max_choices == 1 {
            // Check if any choice starts with the current input
            let has_partial_match = self.choices.iter().any(|choice| {
                if self.case_sensitive {
                    choice.starts_with(input)
                } else {
                    choice.to_lowercase().starts_with(&input.to_lowercase())
                }
            });
            
            if !has_partial_match {
                // Find the first character where it diverges
                for (i, _ch) in input.char_indices() {
                    let partial = &input[..=i];
                    
                    let has_match = self.choices.iter().any(|choice| {
                        if self.case_sensitive {
                            choice.starts_with(partial)
                        } else {
                            choice.to_lowercase().starts_with(&partial.to_lowercase())
                        }
                    });
                    
                    if !has_match {
                        return PartialValidationResult::error_at(i);
                    }
                }
            }
        } else {
            // For multiple choices, validate the current choice being typed
            let parts: Vec<&str> = input.split(',').collect();
            if let Some(current_choice) = parts.last() {
                let current_choice = current_choice.trim();
                if !current_choice.is_empty() {
                    let has_partial_match = self.choices.iter().any(|choice| {
                        if self.case_sensitive {
                            choice.starts_with(current_choice)
                        } else {
                            choice.to_lowercase().starts_with(&current_choice.to_lowercase())
                        }
                    });
                    
                    if !has_partial_match {
                        // Calculate position of error in the current choice
                        let prefix_len: usize = parts[..parts.len()-1]
                            .iter()
                            .map(|p| p.len() + 1) // +1 for comma
                            .sum();
                        
                        for (i, _ch) in current_choice.char_indices() {
                            let partial = current_choice[..=i].trim();
                            
                            let has_match = self.choices.iter().any(|choice| {
                                if self.case_sensitive {
                                    choice.starts_with(partial)
                                } else {
                                    choice.to_lowercase().starts_with(&partial.to_lowercase())
                                }
                            });
                            
                            if !has_match {
                                return PartialValidationResult::error_at(prefix_len + i);
                            }
                        }
                    }
                }
            }
        }
        
        PartialValidationResult::valid()
    }
    
    fn priority(&self) -> Priority {
        self.priority
    }
    
    fn name(&self) -> &str {
        "choice"
    }
}