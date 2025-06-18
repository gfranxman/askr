use thiserror::Error;

#[derive(Debug, Error)]
pub enum PromptError {
    #[error("Invalid CLI arguments: {0}")]
    InvalidArguments(String),
    
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Terminal error: {0}")]
    Terminal(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Timeout exceeded")]
    Timeout,
    
    #[error("User interrupted")]
    Interrupted,
    
    #[error("Maximum attempts exceeded")]
    MaxAttemptsExceeded,
    
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Date/time parsing error: {0}")]
    DateTime(#[from] chrono::ParseError),
}

pub type Result<T> = std::result::Result<T, PromptError>;

impl PromptError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::ValidationFailed(_) => 1,
            Self::InvalidArguments(_) => 2,
            Self::MaxAttemptsExceeded => 3,
            Self::Timeout => 124,
            Self::Interrupted => 130,
            _ => 1,
        }
    }
}