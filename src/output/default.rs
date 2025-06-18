use super::OutputFormatter;
use crate::validation::ValidationSummary;
use crate::error::Result;

/// Default output formatter - just the value to stdout
pub struct DefaultFormatter;

impl OutputFormatter for DefaultFormatter {
    fn format(&self, summary: &ValidationSummary) -> Result<String> {
        if summary.valid {
            Ok(summary.value.clone())
        } else {
            // For default format, we don't output anything on validation failure
            // The exit code will indicate the failure
            Ok(String::new())
        }
    }
}