use regex::Regex;
use once_cell::sync::Lazy;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use super::super::{Validator, ValidationResult, PartialValidationResult, Priority};

// Email validation regex - comprehensive RFC-compliant pattern
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap()
});

// Hostname validation regex - RFC-compliant
static HOSTNAME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap()
});

// URL validation regex - basic but comprehensive
static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^https?://[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*(/[^\s]*)?$").unwrap()
});

/// Email address validator
#[derive(Debug)]
pub struct EmailValidator {
    priority: Priority,
    custom_message: Option<String>,
}

impl EmailValidator {
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

impl Validator for EmailValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        if EMAIL_REGEX.is_match(input) && input.len() <= 254 { // RFC 5321 limit
            ValidationResult::success("email")
        } else {
            let message = self.custom_message.as_deref().unwrap_or("Must be a valid email address");
            ValidationResult::failure("email", self.priority, message)
        }
    }
    
    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }
        
        // Basic checks during typing
        if input.contains('@') {
            let parts: Vec<&str> = input.split('@').collect();
            if parts.len() > 2 {
                // Multiple @ symbols
                if let Some(pos) = input.rfind('@') {
                    return PartialValidationResult::error_at(pos);
                }
            }
        }
        
        PartialValidationResult::valid()
    }
    
    fn priority(&self) -> Priority {
        self.priority
    }
    
    fn name(&self) -> &str {
        "email"
    }
}

/// Hostname/domain validator
#[derive(Debug)]
pub struct HostnameValidator {
    priority: Priority,
    custom_message: Option<String>,
}

impl HostnameValidator {
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

impl Validator for HostnameValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        if input.len() > 253 {
            // RFC 1035 limit
            let message = self.custom_message.as_deref().unwrap_or("Hostname too long (max 253 characters)");
            return ValidationResult::failure("hostname", self.priority, message);
        }
        
        if HOSTNAME_REGEX.is_match(input) {
            ValidationResult::success("hostname")
        } else {
            let message = self.custom_message.as_deref().unwrap_or("Must be a valid hostname");
            ValidationResult::failure("hostname", self.priority, message)
        }
    }
    
    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }
        
        // Check for invalid characters
        for (i, ch) in input.char_indices() {
            if !ch.is_ascii_alphanumeric() && ch != '.' && ch != '-' {
                return PartialValidationResult::error_at(i);
            }
        }
        
        // Check for consecutive dots
        if input.contains("..") {
            if let Some(pos) = input.find("..") {
                return PartialValidationResult::error_at(pos);
            }
        }
        
        PartialValidationResult::valid()
    }
    
    fn priority(&self) -> Priority {
        self.priority
    }
    
    fn name(&self) -> &str {
        "hostname"
    }
}

/// URL validator
#[derive(Debug)]
pub struct UrlValidator {
    priority: Priority,
    custom_message: Option<String>,
}

impl UrlValidator {
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

impl Validator for UrlValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        if URL_REGEX.is_match(input) {
            ValidationResult::success("url")
        } else {
            let message = self.custom_message.as_deref().unwrap_or("Must be a valid URL (http:// or https://)");
            ValidationResult::failure("url", self.priority, message)
        }
    }
    
    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }
        
        // Must start with http:// or https://
        if !input.starts_with("http://") && !input.starts_with("https://") {
            if input.len() >= 8 && !input.starts_with("http") {
                return PartialValidationResult::error_at(0);
            }
        }
        
        PartialValidationResult::valid()
    }
    
    fn priority(&self) -> Priority {
        self.priority
    }
    
    fn name(&self) -> &str {
        "url"
    }
}

/// IPv4 address validator
#[derive(Debug)]
pub struct Ipv4Validator {
    priority: Priority,
    custom_message: Option<String>,
}

impl Ipv4Validator {
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

impl Validator for Ipv4Validator {
    fn validate(&self, input: &str) -> ValidationResult {
        if input.parse::<Ipv4Addr>().is_ok() {
            ValidationResult::success("ipv4")
        } else {
            let message = self.custom_message.as_deref().unwrap_or("Must be a valid IPv4 address");
            ValidationResult::failure("ipv4", self.priority, message)
        }
    }
    
    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }
        
        // Check for invalid characters
        for (i, ch) in input.char_indices() {
            if !ch.is_ascii_digit() && ch != '.' {
                return PartialValidationResult::error_at(i);
            }
        }
        
        // Check for too many dots
        let dot_count = input.chars().filter(|&c| c == '.').count();
        if dot_count > 3 {
            if let Some(pos) = input.rfind('.') {
                return PartialValidationResult::error_at(pos);
            }
        }
        
        PartialValidationResult::valid()
    }
    
    fn priority(&self) -> Priority {
        self.priority
    }
    
    fn name(&self) -> &str {
        "ipv4"
    }
}

/// IPv6 address validator
#[derive(Debug)]
pub struct Ipv6Validator {
    priority: Priority,
    custom_message: Option<String>,
}

impl Ipv6Validator {
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

impl Validator for Ipv6Validator {
    fn validate(&self, input: &str) -> ValidationResult {
        if input.parse::<Ipv6Addr>().is_ok() {
            ValidationResult::success("ipv6")
        } else {
            let message = self.custom_message.as_deref().unwrap_or("Must be a valid IPv6 address");
            ValidationResult::failure("ipv6", self.priority, message)
        }
    }
    
    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }
        
        // Check for invalid characters (IPv6 uses hex digits and colons)
        for (i, ch) in input.char_indices() {
            if !ch.is_ascii_hexdigit() && ch != ':' {
                return PartialValidationResult::error_at(i);
            }
        }
        
        PartialValidationResult::valid()
    }
    
    fn priority(&self) -> Priority {
        self.priority
    }
    
    fn name(&self) -> &str {
        "ipv6"
    }
}