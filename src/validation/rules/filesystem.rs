use super::super::{PartialValidationResult, Priority, ValidationResult, Validator};
use std::fs;
use std::path::Path;

/// File exists validator
#[derive(Debug)]
pub struct FileExistsValidator {
    priority: Priority,
    custom_message: Option<String>,
}

impl FileExistsValidator {
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

impl Validator for FileExistsValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        let path = Path::new(input);

        if path.exists() && path.is_file() {
            ValidationResult::success("file_exists")
        } else {
            let message = if let Some(msg) = &self.custom_message {
                msg.clone()
            } else if !path.exists() {
                "File does not exist".to_string()
            } else {
                "Path exists but is not a file".to_string()
            };
            ValidationResult::failure("file_exists", self.priority, &message)
        }
    }

    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }

        // Basic path validation - check for invalid characters
        if cfg!(windows) {
            let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
            for (i, ch) in input.char_indices() {
                if invalid_chars.contains(&ch) {
                    return PartialValidationResult::error_at(i);
                }
            }
        } else {
            // On Unix, only null character is truly invalid
            for (i, ch) in input.char_indices() {
                if ch == '\0' {
                    return PartialValidationResult::error_at(i);
                }
            }
        }

        PartialValidationResult::valid()
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn name(&self) -> &str {
        "file_exists"
    }
}

/// Directory exists validator
#[derive(Debug)]
pub struct DirExistsValidator {
    priority: Priority,
    custom_message: Option<String>,
}

impl DirExistsValidator {
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

impl Validator for DirExistsValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        let path = Path::new(input);

        if path.exists() && path.is_dir() {
            ValidationResult::success("dir_exists")
        } else {
            let message = if let Some(msg) = &self.custom_message {
                msg.clone()
            } else if !path.exists() {
                "Directory does not exist".to_string()
            } else {
                "Path exists but is not a directory".to_string()
            };
            ValidationResult::failure("dir_exists", self.priority, &message)
        }
    }

    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }

        // Basic path validation - check for invalid characters
        if cfg!(windows) {
            let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
            for (i, ch) in input.char_indices() {
                if invalid_chars.contains(&ch) {
                    return PartialValidationResult::error_at(i);
                }
            }
        } else {
            // On Unix, only null character is truly invalid
            for (i, ch) in input.char_indices() {
                if ch == '\0' {
                    return PartialValidationResult::error_at(i);
                }
            }
        }

        PartialValidationResult::valid()
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn name(&self) -> &str {
        "dir_exists"
    }
}

/// Path exists validator (file or directory)
#[derive(Debug)]
pub struct PathExistsValidator {
    priority: Priority,
    custom_message: Option<String>,
}

impl PathExistsValidator {
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

impl Validator for PathExistsValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        let path = Path::new(input);

        if path.exists() {
            ValidationResult::success("path_exists")
        } else {
            let message = if let Some(msg) = &self.custom_message {
                msg.clone()
            } else {
                "Path does not exist".to_string()
            };
            ValidationResult::failure("path_exists", self.priority, &message)
        }
    }

    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }

        // Basic path validation - check for invalid characters
        if cfg!(windows) {
            let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
            for (i, ch) in input.char_indices() {
                if invalid_chars.contains(&ch) {
                    return PartialValidationResult::error_at(i);
                }
            }
        } else {
            // On Unix, only null character is truly invalid
            for (i, ch) in input.char_indices() {
                if ch == '\0' {
                    return PartialValidationResult::error_at(i);
                }
            }
        }

        PartialValidationResult::valid()
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn name(&self) -> &str {
        "path_exists"
    }
}

/// Readable validator
#[derive(Debug)]
pub struct ReadableValidator {
    priority: Priority,
    custom_message: Option<String>,
}

impl ReadableValidator {
    pub fn new() -> Self {
        Self {
            priority: Priority::Medium,
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

impl Validator for ReadableValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        let path = Path::new(input);

        if !path.exists() {
            let message = if let Some(msg) = &self.custom_message {
                msg.clone()
            } else {
                "Path does not exist".to_string()
            };
            return ValidationResult::failure("readable", self.priority, &message);
        }

        // Try to read the file/directory to check permissions
        let readable = if path.is_file() {
            fs::File::open(path).is_ok()
        } else if path.is_dir() {
            fs::read_dir(path).is_ok()
        } else {
            false
        };

        if readable {
            ValidationResult::success("readable")
        } else {
            let message = if let Some(msg) = &self.custom_message {
                msg.clone()
            } else {
                "Path is not readable (permission denied)".to_string()
            };
            ValidationResult::failure("readable", self.priority, &message)
        }
    }

    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }

        // Basic path validation - check for invalid characters
        if cfg!(windows) {
            let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
            for (i, ch) in input.char_indices() {
                if invalid_chars.contains(&ch) {
                    return PartialValidationResult::error_at(i);
                }
            }
        } else {
            // On Unix, only null character is truly invalid
            for (i, ch) in input.char_indices() {
                if ch == '\0' {
                    return PartialValidationResult::error_at(i);
                }
            }
        }

        PartialValidationResult::valid()
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn name(&self) -> &str {
        "readable"
    }
}

/// Writable validator
#[derive(Debug)]
pub struct WritableValidator {
    priority: Priority,
    custom_message: Option<String>,
}

impl WritableValidator {
    pub fn new() -> Self {
        Self {
            priority: Priority::Medium,
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

impl Validator for WritableValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        let path = Path::new(input);

        if !path.exists() {
            let message = if let Some(msg) = &self.custom_message {
                msg.clone()
            } else {
                "Path does not exist".to_string()
            };
            return ValidationResult::failure("writable", self.priority, &message);
        }

        // Check if we can write to the path
        let writable = if path.is_file() {
            fs::OpenOptions::new().write(true).open(path).is_ok()
        } else if path.is_dir() {
            // For directories, try to create a temporary file
            let temp_file = path.join(".tmp_write_test");
            if let Ok(mut file) = fs::File::create(&temp_file) {
                use std::io::Write;
                let can_write = file.write_all(b"test").is_ok();
                let _ = fs::remove_file(temp_file); // Clean up
                can_write
            } else {
                false
            }
        } else {
            false
        };

        if writable {
            ValidationResult::success("writable")
        } else {
            let message = if let Some(msg) = &self.custom_message {
                msg.clone()
            } else {
                "Path is not writable (permission denied)".to_string()
            };
            ValidationResult::failure("writable", self.priority, &message)
        }
    }

    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }

        // Basic path validation - check for invalid characters
        if cfg!(windows) {
            let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
            for (i, ch) in input.char_indices() {
                if invalid_chars.contains(&ch) {
                    return PartialValidationResult::error_at(i);
                }
            }
        } else {
            // On Unix, only null character is truly invalid
            for (i, ch) in input.char_indices() {
                if ch == '\0' {
                    return PartialValidationResult::error_at(i);
                }
            }
        }

        PartialValidationResult::valid()
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn name(&self) -> &str {
        "writable"
    }
}

/// Executable validator
#[derive(Debug)]
pub struct ExecutableValidator {
    priority: Priority,
    custom_message: Option<String>,
}

impl ExecutableValidator {
    pub fn new() -> Self {
        Self {
            priority: Priority::Medium,
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

impl Validator for ExecutableValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        let path = Path::new(input);

        if !path.exists() {
            let message = if let Some(msg) = &self.custom_message {
                msg.clone()
            } else {
                "Path does not exist".to_string()
            };
            return ValidationResult::failure("executable", self.priority, &message);
        }

        if !path.is_file() {
            let message = if let Some(msg) = &self.custom_message {
                msg.clone()
            } else {
                "Path is not a file".to_string()
            };
            return ValidationResult::failure("executable", self.priority, &message);
        }

        // Check if the file is executable
        #[cfg(unix)]
        let executable = {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = fs::metadata(path) {
                let permissions = metadata.permissions();
                permissions.mode() & 0o111 != 0 // Check if any execute bit is set
            } else {
                false
            }
        };

        #[cfg(windows)]
        let executable = {
            // On Windows, check if it has an executable extension
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                matches!(
                    ext_str.as_str(),
                    "exe" | "bat" | "cmd" | "com" | "scr" | "msi"
                )
            } else {
                false
            }
        };

        if executable {
            ValidationResult::success("executable")
        } else {
            let message = if let Some(msg) = &self.custom_message {
                msg.clone()
            } else {
                "File is not executable".to_string()
            };
            ValidationResult::failure("executable", self.priority, &message)
        }
    }

    fn partial_validate(&self, input: &str, _cursor_pos: usize) -> PartialValidationResult {
        if input.is_empty() {
            return PartialValidationResult::valid();
        }

        // Basic path validation - check for invalid characters
        if cfg!(windows) {
            let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
            for (i, ch) in input.char_indices() {
                if invalid_chars.contains(&ch) {
                    return PartialValidationResult::error_at(i);
                }
            }
        } else {
            // On Unix, only null character is truly invalid
            for (i, ch) in input.char_indices() {
                if ch == '\0' {
                    return PartialValidationResult::error_at(i);
                }
            }
        }

        PartialValidationResult::valid()
    }

    fn priority(&self) -> Priority {
        self.priority
    }

    fn name(&self) -> &str {
        "executable"
    }
}
