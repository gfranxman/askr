pub mod cli;
pub mod error;
pub mod input;
pub mod output;
pub mod ui;
pub mod validation;

pub use cli::{Args, PromptConfig};
pub use error::{PromptError, Result};
pub use validation::{Priority, ValidationEngine, ValidationResult, ValidationSummary};

// Re-export commonly used validators for testing
pub use validation::rules::basic::{
    MaxLengthValidator, MinLengthValidator, PatternValidator, RequiredValidator,
};
pub use validation::rules::choice::ChoiceValidator;
pub use validation::rules::datetime::{DateTimeValidator, DateValidator, TimeValidator};
pub use validation::rules::filesystem::{
    DirExistsValidator, ExecutableValidator, FileExistsValidator, PathExistsValidator,
    ReadableValidator, WritableValidator,
};
pub use validation::rules::format::{
    EmailValidator, HostnameValidator, Ipv4Validator, Ipv6Validator, UrlValidator,
};
pub use validation::rules::numeric::{
    FloatValidator, IntegerValidator, NegativeValidator, PositiveValidator, RangeValidator,
};
