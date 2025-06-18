use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io::{self, stderr};
use std::time::Duration;
use super::{Terminal, ColorScheme, Colorizer, LayoutManager, Screen};
use crate::validation::ValidationEngine;
use crate::cli::config::{PromptConfig, InteractionConfig};
use crate::error::{PromptError, Result};

pub struct InteractivePrompt {
    terminal: Terminal,
    validation_engine: ValidationEngine,
    config: PromptConfig,
}

impl InteractivePrompt {
    pub fn new(mut terminal: Terminal, validation_engine: ValidationEngine, config: PromptConfig) -> Result<Self> {
        // Only enter raw mode if we have cursor control (i.e., we're in a TTY)
        if terminal.capabilities().cursor_control {
            terminal.enter_raw_mode()?;
        }
        
        Ok(Self {
            terminal,
            validation_engine,
            config,
        })
    }
    
    pub fn prompt(&mut self) -> Result<String> {
        let prompt_text = self.config.prompt_text.as_deref().unwrap_or("Enter input:");
        
        // Set up UI components
        let capabilities = self.terminal.capabilities().clone();
        let (width, height) = self.terminal.size()?;
        
        let color_scheme = if self.config.ui_config.no_color {
            ColorScheme::no_color()
        } else {
            ColorScheme::default()
        };
        
        let colorizer = Colorizer::new(color_scheme, self.config.ui_config.no_color);
        let layout = LayoutManager::new(width, height);
        let mut screen = Screen::new(stderr(), layout, colorizer);
        
        // Calculate layout
        screen.layout_mut().calculate_layout(self.config.ui_config.help_text.is_some());
        
        // Draw initial screen
        let prompt_width = screen.write_prompt(prompt_text)?;
        
        if let Some(help_text) = &self.config.ui_config.help_text {
            screen.write_help(help_text)?;
        }
        
        screen.flush()?;
        
        // Input loop
        let mut input = String::new();
        let mut attempts = 0;
        let max_attempts = self.config.interaction_config.max_attempts.unwrap_or(u32::MAX);
        
        loop {
            // Position cursor after current input
            screen.position_cursor_after_input(&input, prompt_width)?;
            screen.flush()?;
            
            // Handle timeout
            let timeout = self.config.interaction_config.timeout.unwrap_or(Duration::from_secs(300));
            
            // Read input event
            if event::poll(timeout)? {
                if let Event::Key(key_event) = event::read()? {
                    match self.handle_key_event(key_event, &mut input, &mut screen, prompt_width)? {
                        InputAction::Continue => {
                            // Validate and update display
                            self.update_validation_display(&input, &mut screen)?;
                        }
                        InputAction::Submit => {
                            // Final validation
                            let summary = self.validation_engine.validate(&input);
                            if summary.valid {
                                return Ok(input);
                            } else {
                                attempts += 1;
                                if attempts >= max_attempts {
                                    return Err(PromptError::MaxAttemptsExceeded);
                                }
                                // Show errors and continue
                                self.update_validation_display(&input, &mut screen)?;
                            }
                        }
                        InputAction::Cancel => {
                            return Err(PromptError::Interrupted);
                        }
                    }
                }
            } else {
                return Err(PromptError::Timeout);
            }
        }
    }
    
    fn handle_key_event(&self, key_event: KeyEvent, input: &mut String, screen: &mut Screen<io::Stderr>, prompt_width: u16) -> Result<InputAction> {
        match key_event {
            // Submit on Enter
            KeyEvent { code: KeyCode::Enter, .. } => {
                // Check if we have a default value and input is empty
                if input.is_empty() && self.config.interaction_config.default_value.is_some() {
                    *input = self.config.interaction_config.default_value.as_ref().unwrap().clone();
                }
                Ok(InputAction::Submit)
            }
            
            // Cancel on Ctrl+C
            KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, .. } => {
                Ok(InputAction::Cancel)
            }
            
            // Cancel on Ctrl+D (EOF)
            KeyEvent { code: KeyCode::Char('d'), modifiers: KeyModifiers::CONTROL, .. } => {
                Ok(InputAction::Cancel)
            }
            
            // Backspace
            KeyEvent { code: KeyCode::Backspace, .. } => {
                if !input.is_empty() {
                    input.pop();
                    screen.write_input(input, prompt_width, None)?;
                }
                Ok(InputAction::Continue)
            }
            
            // Regular character input
            KeyEvent { code: KeyCode::Char(c), .. } => {
                // Handle masking for passwords
                if self.config.interaction_config.mask_input {
                    input.push(c);
                    // Display asterisks instead of actual characters
                    let masked = "*".repeat(input.len());
                    screen.write_input(&masked, prompt_width, None)?;
                } else {
                    input.push(c);
                    // For now, write input without validation coloring - we'll add this in next step
                    screen.write_input(input, prompt_width, None)?;
                }
                Ok(InputAction::Continue)
            }
            
            // Ignore other keys for now
            _ => Ok(InputAction::Continue),
        }
    }
    
    fn update_validation_display(&self, input: &str, screen: &mut Screen<io::Stderr>) -> Result<()> {
        if !self.config.interaction_config.mask_input {
            // Get validation results
            let errors = self.validation_engine.get_display_errors(input, Some(10));
            
            // Get first error position for text coloring
            let partial_result = self.validation_engine.partial_validate(input, input.len());
            
            // Update screen
            screen.write_errors(&errors)?;
            screen.flush()?;
        }
        
        Ok(())
    }
}

impl Drop for InteractivePrompt {
    fn drop(&mut self) {
        // Clean up terminal state only if we have cursor control
        if self.terminal.capabilities().cursor_control {
            let _ = self.terminal.leave_raw_mode();
        }
    }
}

#[derive(Debug)]
enum InputAction {
    Continue,
    Submit,
    Cancel,
}