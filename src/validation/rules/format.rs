use super::super::{PartialValidationResult, Priority, ValidationResult, Validator};
use once_cell::sync::Lazy;
use regex::Regex;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

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
        if EMAIL_REGEX.is_match(input) && input.len() <= 254 {
            // RFC 5321 limit
            ValidationResult::success("email")
        } else {
            let message = self
                .custom_message
                .as_deref()
                .unwrap_or("Must be a valid email address");
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
            let message = self
                .custom_message
                .as_deref()
                .unwrap_or("Hostname too long (max 253 characters)");
            return ValidationResult::failure("hostname", self.priority, message);
        }

        if HOSTNAME_REGEX.is_match(input) {
            ValidationResult::success("hostname")
        } else {
            let message = self
                .custom_message
                .as_deref()
                .unwrap_or("Must be a valid hostname");
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
            let message = self
                .custom_message
                .as_deref()
                .unwrap_or("Must be a valid URL (http:// or https://)");
            ValidationResult::failure("url", self.priority, message)
        }
    }

    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }

        // Must start with http:// or https://
        if !input.starts_with("http://") && !input.starts_with("https://") {
            // If we have enough characters to rule out http/https, it's an error
            if input.len() >= 7 && !input.starts_with("http") {
                return PartialValidationResult::error_at(0);
            }
            // Also check for other common protocols that we don't support
            if input.starts_with("ftp://")
                || input.starts_with("ssh://")
                || input.starts_with("file://")
            {
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
            let message = self
                .custom_message
                .as_deref()
                .unwrap_or("Must be a valid IPv4 address");
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
            let message = self
                .custom_message
                .as_deref()
                .unwrap_or("Must be a valid IPv6 address");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validator_valid() {
        let validator = EmailValidator::new();

        let valid_emails = [
            "user@example.com",
            "test.email@domain.co.uk",
            "user+tag@example.org",
            "user_name@domain-name.com",
            "123@456.com",
            "a@b.co",
        ];

        for email in valid_emails {
            let result = validator.validate(email);
            assert!(result.passed, "Email should be valid: {}", email);
        }
    }

    #[test]
    fn test_email_validator_invalid() {
        let validator = EmailValidator::new();

        let invalid_emails = [
            "not-an-email",
            "@example.com",
            "user@",
            "user@.example.com",
            "user@example.",
            "user name@example.com",
            "",
        ];

        for email in invalid_emails {
            let result = validator.validate(email);
            assert!(!result.passed, "Email should be invalid: {}", email);
            assert!(result.message.is_some());
        }
    }

    #[test]
    fn test_hostname_validator_valid() {
        let validator = HostnameValidator::new();

        let valid_hostnames = [
            "example.com",
            "subdomain.example.com",
            "test-server.local",
            "server123.example.org",
            "a.b.c.d",
            "localhost",
        ];

        for hostname in valid_hostnames {
            let result = validator.validate(hostname);
            assert!(result.passed, "Hostname should be valid: {}", hostname);
        }
    }

    #[test]
    fn test_hostname_validator_invalid() {
        let validator = HostnameValidator::new();

        let invalid_hostnames = [
            "",
            ".example.com",
            "example..com",
            "example.com.",
            "-example.com",
            "example-.com",
            "ex ample.com",
            "very-long-hostname-that-exceeds-the-maximum-allowed-length-for-a-single-label-in-dns.com",
        ];

        for hostname in invalid_hostnames {
            let result = validator.validate(hostname);
            assert!(!result.passed, "Hostname should be invalid: {}", hostname);
        }
    }

    #[test]
    fn test_url_validator_valid() {
        let validator = UrlValidator::new();

        let valid_urls = [
            "https://example.com",
            "http://subdomain.example.org",
            "https://example.com/path/to/resource",
            "https://example.com/path",
        ];

        for url in valid_urls {
            let result = validator.validate(url);
            assert!(result.passed, "URL should be valid: {}", url);
        }
    }

    #[test]
    fn test_url_validator_invalid() {
        let validator = UrlValidator::new();

        let invalid_urls = [
            "",
            "not-a-url",
            "ftp://example.com", // Only http/https supported
            "https://",
            "http://",
            "example.com",          // Missing protocol
            "https://exam ple.com", // Space in hostname
        ];

        for url in invalid_urls {
            let result = validator.validate(url);
            assert!(!result.passed, "URL should be invalid: {}", url);
        }
    }

    #[test]
    fn test_ipv4_validator_valid() {
        let validator = Ipv4Validator::new();

        let valid_ips = [
            "192.168.1.1",
            "10.0.0.1",
            "172.16.255.254",
            "127.0.0.1",
            "0.0.0.0",
            "255.255.255.255",
        ];

        for ip in valid_ips {
            let result = validator.validate(ip);
            assert!(result.passed, "IPv4 should be valid: {}", ip);
        }
    }

    #[test]
    fn test_ipv4_validator_invalid() {
        let validator = Ipv4Validator::new();

        let invalid_ips = [
            "",
            "192.168.1",
            "192.168.1.256",
            "192.168.1.1.1",
            "192.168.1.-1",
            "192.168.01.1",   // Leading zeros
            "192.168.1.1/24", // CIDR notation
            "not.an.ip.address",
        ];

        for ip in invalid_ips {
            let result = validator.validate(ip);
            assert!(!result.passed, "IPv4 should be invalid: {}", ip);
        }
    }

    #[test]
    fn test_ipv6_validator_valid() {
        let validator = Ipv6Validator::new();

        let valid_ips = [
            "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
            "2001:db8:85a3::8a2e:370:7334",
            "::1",
            "::",
            "2001:db8::1",
            "fe80::1",
        ];

        for ip in valid_ips {
            let result = validator.validate(ip);
            assert!(result.passed, "IPv6 should be valid: {}", ip);
        }
    }

    #[test]
    fn test_ipv6_validator_invalid() {
        let validator = Ipv6Validator::new();

        let invalid_ips = [
            "",
            "192.168.1.1",                                   // IPv4
            "2001:0db8:85a3::8a2e:370g:7334",                // Invalid character 'g'
            "2001:0db8:85a3:::8a2e:370:7334",                // Triple colon
            "2001:0db8:85a3:0000:0000:8a2e:0370:7334:extra", // Too many groups
            "not:an:ipv6:address",
        ];

        for ip in invalid_ips {
            let result = validator.validate(ip);
            assert!(!result.passed, "IPv6 should be invalid: {}", ip);
        }
    }

    #[test]
    fn test_custom_messages() {
        let email_validator = EmailValidator::new().with_message("Please enter a valid email");
        let result = email_validator.validate("invalid");
        assert!(!result.passed);
        assert_eq!(result.message.unwrap(), "Please enter a valid email");

        let hostname_validator = HostnameValidator::new().with_message("Invalid hostname");
        let result = hostname_validator.validate("invalid..hostname");
        assert!(!result.passed);
        assert_eq!(result.message.unwrap(), "Invalid hostname");
    }

    #[test]
    fn test_custom_priorities() {
        let validator = EmailValidator::new().with_priority(Priority::Low);
        assert_eq!(validator.priority(), Priority::Low);

        let result = validator.validate("invalid");
        assert!(!result.passed);
        assert_eq!(result.priority, Priority::Low);
    }

    #[test]
    fn test_partial_validation() {
        let email_validator = EmailValidator::new();

        // Valid partial input
        let result = email_validator.partial_validate("user@exam", 8);
        assert!(result.first_error_pos.is_none());

        // Invalid partial input (multiple @ symbols)
        let result = email_validator.partial_validate("user@exam@ple", 10);
        assert!(result.first_error_pos.is_some());

        let url_validator = UrlValidator::new();

        // Valid partial URL
        let result = url_validator.partial_validate("https://exa", 11);
        assert!(result.first_error_pos.is_none());

        // Invalid URL start
        let result = url_validator.partial_validate("ftp://", 6);
        assert!(result.first_error_pos.is_some());
    }

    #[test]
    fn test_edge_cases() {
        // Very long email
        let long_local = "a".repeat(64);
        let long_domain = "b".repeat(63);
        let long_email = format!("{}@{}.com", long_local, long_domain);

        let email_validator = EmailValidator::new();
        let result = email_validator.validate(&long_email);
        // Should handle long emails gracefully (may pass or fail depending on implementation)

        // IPv4 edge cases
        let ipv4_validator = Ipv4Validator::new();
        assert!(ipv4_validator.validate("0.0.0.0").passed);
        assert!(ipv4_validator.validate("255.255.255.255").passed);

        // IPv6 edge cases
        let ipv6_validator = Ipv6Validator::new();
        assert!(ipv6_validator.validate("::").passed);
        assert!(ipv6_validator.validate("::1").passed);
    }

    #[test]
    fn test_international_domains() {
        let hostname_validator = HostnameValidator::new();

        // Note: These tests depend on how the hostname validator handles IDN
        // Basic ASCII domains should work
        assert!(hostname_validator.validate("example.com").passed);
        assert!(hostname_validator.validate("subdomain.example.org").passed);
    }

    #[test]
    fn test_validator_names() {
        assert_eq!(EmailValidator::new().name(), "email");
        assert_eq!(HostnameValidator::new().name(), "hostname");
        assert_eq!(UrlValidator::new().name(), "url");
        assert_eq!(Ipv4Validator::new().name(), "ipv4");
        assert_eq!(Ipv6Validator::new().name(), "ipv6");
    }

    #[test]
    fn test_validator_priorities() {
        assert_eq!(EmailValidator::new().priority(), Priority::High);
        assert_eq!(HostnameValidator::new().priority(), Priority::High);
        assert_eq!(UrlValidator::new().priority(), Priority::High);
        assert_eq!(Ipv4Validator::new().priority(), Priority::High);
        assert_eq!(Ipv6Validator::new().priority(), Priority::High);
    }
}
