use super::OutputFormatter;
use crate::validation::ValidationSummary;
use crate::error::Result;

/// JSON output formatter
pub struct JsonFormatter;

impl OutputFormatter for JsonFormatter {
    fn format(&self, summary: &ValidationSummary) -> Result<String> {
        let json = serde_json::to_string_pretty(summary)?;
        Ok(json)
    }
}