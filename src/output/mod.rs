pub mod default;
pub mod json;
pub mod raw;

pub use default::*;
pub use json::*;
pub use raw::*;

use crate::error::Result;
use crate::validation::ValidationSummary;

pub trait OutputFormatter {
    fn format(&self, summary: &ValidationSummary) -> Result<String>;
}
