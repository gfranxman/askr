use super::OutputFormatter;
use crate::error::Result;
use crate::validation::ValidationSummary;

/// Raw output formatter - outputs the raw value without any validation
pub struct RawFormatter;

impl OutputFormatter for RawFormatter {
    fn format(&self, summary: &ValidationSummary) -> Result<String> {
        // For raw format, always output the value regardless of validation
        Ok(summary.value.clone())
    }
}
