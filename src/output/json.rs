use super::OutputFormatter;
use crate::error::Result;
use crate::validation::ValidationSummary;

/// JSON output formatter
pub struct JsonFormatter;

impl OutputFormatter for JsonFormatter {
    fn format(&self, summary: &ValidationSummary) -> Result<String> {
        let json = serde_json::to_string_pretty(summary)?;
        Ok(json)
    }
}
