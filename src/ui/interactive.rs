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
        let mut cursor_pos = 0; // Track cursor position within the input
        let mut attempts = 0;
        let max_attempts = self.config.interaction_config.max_attempts.unwrap_or(u32::MAX);
        
        loop {
            // Position cursor at the correct position within the input
            screen.position_cursor_at_input_pos(&input, cursor_pos, prompt_width)?;
            screen.flush()?;
            
            // Handle timeout
            let timeout = self.config.interaction_config.timeout.unwrap_or(Duration::from_secs(300));
            
            // Read input event
            if event::poll(timeout)? {
                if let Event::Key(key_event) = event::read()? {
                    match self.handle_key_event(key_event, &mut input, &mut cursor_pos, &mut screen, prompt_width)? {
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
    
    fn handle_key_event(&self, key_event: KeyEvent, input: &mut String, cursor_pos: &mut usize, screen: &mut Screen<io::Stderr>, prompt_width: u16) -> Result<InputAction> {
        match key_event {
            // Submit on Enter
            KeyEvent { code: KeyCode::Enter, .. } => {
                // Check if we have a default value and input is empty
                if input.is_empty() && self.config.interaction_config.default_value.is_some() {
                    *input = self.config.interaction_config.default_value.as_ref().unwrap().clone();
                    *cursor_pos = input.chars().count();
                }
                Ok(InputAction::Submit)
            }
            
            // Cancel on Ctrl+C
            KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, .. } => {
                Ok(InputAction::Cancel)
            }
            
            // Cancel on Ctrl+D (EOF) - only if input is empty
            KeyEvent { code: KeyCode::Char('d'), modifiers: KeyModifiers::CONTROL, .. } => {
                if input.is_empty() {
                    Ok(InputAction::Cancel)
                } else {
                    Ok(InputAction::Continue)
                }
            }
            
            // Ctrl+A - Jump to beginning of line
            KeyEvent { code: KeyCode::Char('a'), modifiers: KeyModifiers::CONTROL, .. } => {
                *cursor_pos = 0;
                Ok(InputAction::Continue)
            }
            
            // Ctrl+E - Jump to end of line
            KeyEvent { code: KeyCode::Char('e'), modifiers: KeyModifiers::CONTROL, .. } => {
                *cursor_pos = input.chars().count();
                Ok(InputAction::Continue)
            }
            
            // Ctrl+K - Kill from cursor to end of line
            KeyEvent { code: KeyCode::Char('k'), modifiers: KeyModifiers::CONTROL, .. } => {
                let chars: Vec<char> = input.chars().collect();
                if *cursor_pos < chars.len() {
                    let new_input: String = chars[..*cursor_pos].iter().collect();
                    *input = new_input;
                    self.redraw_input(input, cursor_pos, screen, prompt_width)?;
                }
                Ok(InputAction::Continue)
            }
            
            // Ctrl+U - Kill from beginning of line to cursor
            KeyEvent { code: KeyCode::Char('u'), modifiers: KeyModifiers::CONTROL, .. } => {
                let chars: Vec<char> = input.chars().collect();
                if *cursor_pos > 0 {
                    let new_input: String = chars[*cursor_pos..].iter().collect();
                    *input = new_input;
                    *cursor_pos = 0;
                    self.redraw_input(input, cursor_pos, screen, prompt_width)?;
                }
                Ok(InputAction::Continue)
            }
            
            // Ctrl+W - Delete word before cursor
            KeyEvent { code: KeyCode::Char('w'), modifiers: KeyModifiers::CONTROL, .. } => {
                if *cursor_pos > 0 {
                    let chars: Vec<char> = input.chars().collect();
                    let mut new_cursor = *cursor_pos;
                    
                    // Skip whitespace before cursor
                    while new_cursor > 0 && chars[new_cursor - 1].is_whitespace() {
                        new_cursor -= 1;
                    }
                    
                    // Delete word characters
                    while new_cursor > 0 && !chars[new_cursor - 1].is_whitespace() {
                        new_cursor -= 1;
                    }
                    
                    let mut new_chars = chars;
                    new_chars.drain(new_cursor..*cursor_pos);
                    *input = new_chars.iter().collect();
                    *cursor_pos = new_cursor;
                    self.redraw_input(input, cursor_pos, screen, prompt_width)?;
                }
                Ok(InputAction::Continue)
            }
            
            // Arrow keys for cursor movement
            KeyEvent { code: KeyCode::Left, .. } => {
                if *cursor_pos > 0 {
                    *cursor_pos -= 1;
                }
                Ok(InputAction::Continue)
            }
            
            KeyEvent { code: KeyCode::Right, .. } => {
                if *cursor_pos < input.chars().count() {
                    *cursor_pos += 1;
                }
                Ok(InputAction::Continue)
            }
            
            // Home key - jump to beginning
            KeyEvent { code: KeyCode::Home, .. } => {
                *cursor_pos = 0;
                Ok(InputAction::Continue)
            }
            
            // End key - jump to end
            KeyEvent { code: KeyCode::End, .. } => {
                *cursor_pos = input.chars().count();
                Ok(InputAction::Continue)
            }
            
            // Backspace - delete character before cursor
            KeyEvent { code: KeyCode::Backspace, .. } => {
                if *cursor_pos > 0 {
                    let mut chars: Vec<char> = input.chars().collect();
                    chars.remove(*cursor_pos - 1);
                    *input = chars.iter().collect();
                    *cursor_pos -= 1;
                    self.redraw_input(input, cursor_pos, screen, prompt_width)?;
                }
                Ok(InputAction::Continue)
            }
            
            // Delete key - delete character at cursor
            KeyEvent { code: KeyCode::Delete, .. } => {
                let mut chars: Vec<char> = input.chars().collect();
                if *cursor_pos < chars.len() {
                    chars.remove(*cursor_pos);
                    *input = chars.iter().collect();
                    self.redraw_input(input, cursor_pos, screen, prompt_width)?;
                }
                Ok(InputAction::Continue)
            }
            
            // Regular character input
            KeyEvent { code: KeyCode::Char(c), modifiers, .. } => {
                // Skip if this is a control character we already handled
                if modifiers.contains(KeyModifiers::CONTROL) {
                    return Ok(InputAction::Continue);
                }
                
                // Insert character at cursor position
                let mut chars: Vec<char> = input.chars().collect();
                chars.insert(*cursor_pos, c);
                *input = chars.iter().collect();
                *cursor_pos += 1;
                
                // Handle masking for passwords
                if self.config.interaction_config.mask_input {
                    let masked = "*".repeat(input.chars().count());
                    self.redraw_input(&masked, cursor_pos, screen, prompt_width)?;
                } else {
                    self.redraw_input(input, cursor_pos, screen, prompt_width)?;
                }
                Ok(InputAction::Continue)
            }
            
            // Ignore other keys
            _ => Ok(InputAction::Continue),
        }
    }
    
    fn redraw_input(&self, input: &str, _cursor_pos: &usize, screen: &mut Screen<io::Stderr>, prompt_width: u16) -> Result<()> {
        screen.write_input(input, prompt_width, None)?;
        Ok(())
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