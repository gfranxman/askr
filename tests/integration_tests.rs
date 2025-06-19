use std::process::{Command, Stdio};
use std::io::Write;

/// Helper function to run the prompt binary with given arguments and input
fn run_prompt_with_input(args: &[&str], input: &str) -> (i32, String, String) {
    let mut cmd = Command::new("./target/debug/prompt")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start prompt process");

    // Write input to stdin
    if let Some(stdin) = cmd.stdin.as_mut() {
        stdin.write_all(input.as_bytes()).expect("Failed to write to stdin");
    }

    let output = cmd.wait_with_output().expect("Failed to wait for process");
    
    let exit_code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    
    (exit_code, stdout, stderr)
}

/// Helper function to run prompt binary with just arguments (no input)
fn run_prompt(args: &[&str]) -> (i32, String, String) {
    run_prompt_with_input(args, "")
}

#[test]
fn test_help_output() {
    let (exit_code, stdout, _stderr) = run_prompt(&["--help"]);
    
    assert_eq!(exit_code, 0);
    assert!(stdout.contains("Interactive CLI prompt tool"));
    assert!(stdout.contains("--required"));
    assert!(stdout.contains("--min-length"));
}

#[test]
fn test_version_output() {
    let (exit_code, stdout, _stderr) = run_prompt(&["--version"]);
    
    assert_eq!(exit_code, 0);
    assert!(stdout.contains("prompt"));
}

#[test]
fn test_basic_required_validation() {
    // Test valid input
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--required", "Enter text:"],
        "hello world"
    );
    
    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "hello world");
    
    // Test empty input (should fail)
    let (exit_code, _stdout, stderr) = run_prompt_with_input(
        &["--required", "Enter text:"],
        ""
    );
    
    assert_ne!(exit_code, 0);
    assert!(stderr.contains("required") || stderr.contains("Validation failed"));
}

#[test]
fn test_length_validation() {
    // Test min length validation
    let (exit_code, _stdout, stderr) = run_prompt_with_input(
        &["--min-length", "5", "Enter text:"],
        "hi"
    );
    
    assert_ne!(exit_code, 0);
    assert!(stderr.contains("Minimum length") || stderr.contains("Validation failed"));
    
    // Test max length validation
    let (exit_code, _stdout, stderr) = run_prompt_with_input(
        &["--max-length", "5", "Enter text:"],
        "hello world"
    );
    
    assert_ne!(exit_code, 0);
    assert!(stderr.contains("Maximum length") || stderr.contains("Validation failed"));
    
    // Test valid length
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--min-length", "3", "--max-length", "10", "Enter text:"],
        "hello"
    );
    
    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "hello");
}

#[test]
fn test_pattern_validation() {
    // Test valid pattern
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--pattern", r"^\d{3}-\d{3}-\d{4}$", "Enter phone:"],
        "123-456-7890"
    );
    
    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "123-456-7890");
    
    // Test invalid pattern
    let (exit_code, _stdout, stderr) = run_prompt_with_input(
        &["--pattern", r"^\d{3}-\d{3}-\d{4}$", "Enter phone:"],
        "not-a-phone"
    );
    
    assert_ne!(exit_code, 0);
    assert!(stderr.contains("pattern") || stderr.contains("Validation failed"));
}

#[test]
fn test_email_validation() {
    // Test valid email
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--validate-email", "Enter email:"],
        "user@example.com"
    );
    
    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "user@example.com");
    
    // Test invalid email
    let (exit_code, _stdout, stderr) = run_prompt_with_input(
        &["--validate-email", "Enter email:"],
        "not-an-email"
    );
    
    assert_ne!(exit_code, 0);
    assert!(stderr.contains("email") || stderr.contains("Validation failed"));
}

#[test]
fn test_number_validation() {
    // Test integer validation
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--integer", "Enter number:"],
        "42"
    );
    
    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "42");
    
    // Test invalid integer
    let (exit_code, _stdout, stderr) = run_prompt_with_input(
        &["--integer", "Enter number:"],
        "not-a-number"
    );
    
    assert_ne!(exit_code, 0);
    assert!(stderr.contains("number") || stderr.contains("Validation failed"));
    
    // Test float validation
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--float", "Enter number:"],
        "3.14"
    );
    
    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "3.14");
}

#[test]
fn test_range_validation() {
    // Test valid range
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--range", "1-10", "Enter number:"],
        "5"
    );
    
    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "5");
    
    // Test out of range
    let (exit_code, _stdout, stderr) = run_prompt_with_input(
        &["--range", "1-10", "Enter number:"],
        "15"
    );
    
    assert_ne!(exit_code, 0);
    assert!(stderr.contains("range") || stderr.contains("between") || stderr.contains("Validation failed"));
}

#[test]
fn test_choice_validation() {
    // Test valid choice
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--choices", "red,green,blue", "Pick a color:"],
        "red"
    );
    
    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "red");
    
    // Test invalid choice
    let (exit_code, _stdout, stderr) = run_prompt_with_input(
        &["--choices", "red,green,blue", "Pick a color:"],
        "yellow"
    );
    
    assert_ne!(exit_code, 0);
    assert!(stderr.contains("choice") || stderr.contains("Valid options") || stderr.contains("Validation failed"));
}

#[test]
fn test_date_validation() {
    // Test valid date
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--date", "Enter date:"],
        "2025-01-15"
    );
    
    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "2025-01-15");
    
    // Test invalid date
    let (exit_code, _stdout, stderr) = run_prompt_with_input(
        &["--date", "Enter date:"],
        "not-a-date"
    );
    
    assert_ne!(exit_code, 0);
    assert!(stderr.contains("date") || stderr.contains("Validation failed"));
}

#[test]
fn test_file_validation() {
    // Test with existing file (Cargo.toml should exist)
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--file-exists", "Enter file:"],
        "Cargo.toml"
    );
    
    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "Cargo.toml");
    
    // Test with non-existent file
    let (exit_code, _stdout, stderr) = run_prompt_with_input(
        &["--file-exists", "Enter file:"],
        "nonexistent-file.txt"
    );
    
    assert_ne!(exit_code, 0);
    assert!(stderr.contains("exist") || stderr.contains("Validation failed"));
}

#[test]
fn test_json_output() {
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--output", "json", "--required", "Enter text:"],
        "hello"
    );
    
    assert_eq!(exit_code, 0);
    
    // Parse JSON output
    let json: serde_json::Value = serde_json::from_str(&stdout)
        .expect("Output should be valid JSON");
    
    assert_eq!(json["value"], "hello");
    assert_eq!(json["valid"], true);
    assert!(json["validation_results"].is_array());
}

#[test]
fn test_quiet_mode() {
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--quiet", "--required"],
        "hello"
    );
    
    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "hello");
}

#[test]
fn test_multiple_validators() {
    // Test all validators pass
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--required", "--min-length", "5", "--validate-email", "Enter email:"],
        "user@example.com"
    );
    
    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "user@example.com");
    
    // Test some validators fail
    let (exit_code, _stdout, stderr) = run_prompt_with_input(
        &["--required", "--min-length", "20", "--validate-email", "Enter email:"],
        "user@example.com"
    );
    
    assert_ne!(exit_code, 0);
    assert!(stderr.contains("Validation failed"));
}

#[test]
fn test_custom_prompt_text() {
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--required", "Please enter your name:"],
        "John Doe"
    );
    
    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "John Doe");
}

#[test]
fn test_invalid_arguments() {
    // Test invalid range format
    let (exit_code, _stdout, stderr) = run_prompt(&["--range", "invalid", "Enter number:"]);
    
    assert_ne!(exit_code, 0);
    assert!(stderr.contains("Invalid") || stderr.contains("error"));
    
    // Test invalid pattern
    let (exit_code, _stdout, stderr) = run_prompt(&["--pattern", "[", "Enter text:"]);
    
    assert_ne!(exit_code, 0);
    assert!(stderr.contains("Invalid") || stderr.contains("error"));
}

#[test]
fn test_no_color_output() {
    let (exit_code, _stdout, _stderr) = run_prompt_with_input(
        &["--no-color", "--required", "Enter text:"],
        "hello"
    );
    
    assert_eq!(exit_code, 0);
    // Note: Hard to test no-color in automated tests, but should not crash
}

#[test]
fn test_raw_output_format() {
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--output", "raw", "--required", "Enter text:"],
        "hello"
    );
    
    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "hello");
}

#[test]
fn test_unicode_input() {
    // Test with emoji
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--required", "--min-length", "3", "Enter text:"],
        "👋🌍🚀"
    );
    
    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "👋🌍🚀");
    
    // Test with various scripts
    let (exit_code, stdout, _stderr) = run_prompt_with_input(
        &["--required", "Enter text:"],
        "مرحبا こんにちは"
    );
    
    assert_eq!(exit_code, 0);
    assert_eq!(stdout.trim(), "مرحبا こんにちは");
}

#[test]
fn test_shell_completion_generation() {
    // Test bash completion generation
    let (exit_code, stdout, _stderr) = run_prompt(&["completion", "bash"]);
    
    assert_eq!(exit_code, 0);
    assert!(stdout.contains("_prompt()"));
    assert!(stdout.contains("--required"));
    assert!(stdout.contains("--validate-email"));
    assert!(stdout.contains("default json raw"));
    
    // Test zsh completion generation
    let (exit_code, stdout, _stderr) = run_prompt(&["completion", "zsh"]);
    
    assert_eq!(exit_code, 0);
    assert!(stdout.contains("#compdef prompt"));
    assert!(stdout.contains("--output"));
    
    // Test fish completion generation
    let (exit_code, stdout, _stderr) = run_prompt(&["completion", "fish"]);
    
    assert_eq!(exit_code, 0);
    assert!(stdout.contains("__fish_prompt"));
    
    // Test PowerShell completion generation
    let (exit_code, stdout, _stderr) = run_prompt(&["completion", "power-shell"]);
    
    assert_eq!(exit_code, 0);
    assert!(stdout.len() > 0); // Should generate some completion script
}

#[test]
fn test_completion_help() {
    let (exit_code, stdout, _stderr) = run_prompt(&["completion", "--help"]);
    
    assert_eq!(exit_code, 0);
    assert!(stdout.contains("Generate shell completion scripts"));
    assert!(stdout.contains("bash, zsh, fish, power-shell"));
}