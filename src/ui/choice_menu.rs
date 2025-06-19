use super::{ColorScheme, Colorizer, LayoutManager, Screen, Terminal};
use crate::error::{PromptError, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io::{self, stderr};
use std::time::Duration;

pub struct ChoiceMenu {
    terminal: Terminal,
    choices: Vec<String>,
    allow_multiple: bool,
    min_choices: usize,
    max_choices: usize,
    selected_choices: Vec<bool>,
    current_index: usize,
    colorizer: Colorizer,
    validation_error: Option<String>,
    last_content_lines: u16, // Track how many content lines were drawn last time
}

impl ChoiceMenu {
    pub fn new(
        mut terminal: Terminal,
        choices: Vec<String>,
        allow_multiple: bool,
        min_choices: usize,
        max_choices: usize,
        no_color: bool,
    ) -> Result<Self> {
        // Only enter raw mode if we have cursor control
        if terminal.capabilities().cursor_control {
            terminal.enter_raw_mode()?;
        }

        let color_scheme = if no_color {
            ColorScheme::no_color()
        } else {
            ColorScheme::default()
        };

        let colorizer = Colorizer::new(color_scheme, no_color);
        let selected_choices = vec![false; choices.len()];

        Ok(Self {
            terminal,
            choices,
            allow_multiple,
            min_choices,
            max_choices,
            selected_choices,
            current_index: 0,
            colorizer,
            validation_error: None,
            last_content_lines: 0,
        })
    }

    pub fn show(&mut self, prompt_text: &str) -> Result<Vec<String>> {
        use crossterm::{terminal::Clear, terminal::ClearType, ExecutableCommand};
        use std::io::stderr;
        
        // Clear any existing content on the current line first
        stderr().execute(Clear(ClearType::CurrentLine))?;
        
        let (width, height) = self.terminal.size()?;
        let layout = LayoutManager::new(width, height);
        let mut screen = Screen::new(stderr(), layout, self.colorizer.clone());

        // Calculate space needed and reserve it
        let reserved_lines = self.calculate_and_reserve_space(width, prompt_text)?;

        // Move cursor to the reserved prompt position
        self.move_to_prompt_position(reserved_lines)?;

        // Validate initial state and draw menu
        self.validate_selections();
        self.draw_menu(&mut screen, prompt_text)?;
        self.update_content_line_count();

        // Input loop
        loop {
            screen.flush()?;

            let timeout = Duration::from_secs(300);
            if event::poll(timeout)? {
                if let Event::Key(key_event) = event::read()? {
                    match self.handle_key_event(key_event)? {
                        MenuAction::Continue => {
                            // Validate selections after any change
                            self.validate_selections();
                            // Clear the area and redraw
                            self.clear_and_redraw(&mut screen, prompt_text)?;
                        }
                        MenuAction::Submit => {
                            // Only submit if constraints are met
                            if self.can_submit() {
                                let selected = self.get_selected_choices();
                                return Ok(selected);
                            } else {
                                // Validation failed, update error and redraw
                                self.validate_selections();
                                self.clear_and_redraw(&mut screen, prompt_text)?;
                            }
                        }
                        MenuAction::Cancel => {
                            return Err(PromptError::Interrupted);
                        }
                    }
                }
            } else {
                return Err(PromptError::Timeout);
            }
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<MenuAction> {
        match key_event {
            // Navigation with arrow keys
            KeyEvent {
                code: KeyCode::Up, ..
            } => {
                if self.current_index > 0 {
                    self.current_index -= 1;
                }
                Ok(MenuAction::Continue)
            }
            KeyEvent {
                code: KeyCode::Down,
                ..
            } => {
                if self.current_index < self.choices.len() - 1 {
                    self.current_index += 1;
                }
                Ok(MenuAction::Continue)
            }

            // Selection
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => {
                if self.allow_multiple {
                    // In multiple choice mode, Enter submits current selections
                    Ok(MenuAction::Submit)
                } else {
                    // In single choice mode, Enter selects current item and submits
                    self.selected_choices.fill(false);
                    self.selected_choices[self.current_index] = true;
                    Ok(MenuAction::Submit)
                }
            }

            // Toggle selection with spacebar (for multiple choice)
            KeyEvent {
                code: KeyCode::Char(' '),
                ..
            } => {
                if self.allow_multiple {
                    self.selected_choices[self.current_index] =
                        !self.selected_choices[self.current_index];
                }
                Ok(MenuAction::Continue)
            }

            // Cancel with Ctrl+C
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => Ok(MenuAction::Cancel),

            // Cancel with Ctrl+D (EOF)
            KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => Ok(MenuAction::Cancel),

            // Ignore other keys
            _ => Ok(MenuAction::Continue),
        }
    }

    fn draw_menu(&self, screen: &mut Screen<io::Stderr>, prompt_text: &str) -> Result<()> {
        use crossterm::{cursor::MoveToNextLine, ExecutableCommand};

        // Draw prompt at current position (which should be the reserved position)
        let _prompt_width = screen.write_prompt(prompt_text)?;

        // Move to next line for instructions and choices
        stderr().execute(MoveToNextLine(1))?;

        // Draw the menu content
        self.draw_menu_content()?;

        Ok(())
    }

    fn clear_and_redraw(&mut self, screen: &mut Screen<io::Stderr>, prompt_text: &str) -> Result<()> {
        use crossterm::{cursor::{MoveUp, MoveToColumn}, terminal::Clear, terminal::ClearType, ExecutableCommand};

        // Move back using the tracked number of content lines from last draw
        // Add 1 to account for the prompt line
        if self.last_content_lines > 0 {
            stderr().execute(MoveUp(self.last_content_lines + 1))?;
        }

        // Move to the beginning of the line to ensure proper cursor positioning
        stderr().execute(MoveToColumn(0))?;

        // Clear from cursor down to remove old menu content
        stderr().execute(Clear(ClearType::FromCursorDown))?;

        // Redraw the full menu (prompt + instruction + choices + error)
        self.draw_menu(screen, prompt_text)?;
        
        // Update the line count for next time
        self.update_content_line_count();

        Ok(())
    }

    /// Calculate and store how many lines the current content occupies
    fn update_content_line_count(&mut self) {
        let mut lines = 1; // instruction line
        lines += self.choices.len() as u16; // choice lines
        if self.validation_error.is_some() {
            lines += 1; // error line
        }
        // Note: prompt line is handled separately in clear_and_redraw
        self.last_content_lines = lines;
    }

    fn draw_menu_content(&self) -> Result<()> {
        use crossterm::{cursor::MoveToNextLine, ExecutableCommand};

        // Add instruction text
        let instruction = if self.allow_multiple {
            if self.min_choices == self.max_choices {
                format!("Select exactly {} choice(s). Use ↑↓ to navigate, SPACE to toggle, ENTER to submit:", self.min_choices)
            } else {
                format!("Select {}-{} choice(s). Use ↑↓ to navigate, SPACE to toggle, ENTER to submit:", self.min_choices, self.max_choices)
            }
        } else {
            "Use ↑↓ to navigate, ENTER to select:".to_string()
        };

        // Write instruction
        let colored_instruction = self.colorizer.help_text(&instruction);
        self.colorizer
            .write_colored(&mut std::io::stderr(), &colored_instruction)?;
        stderr().execute(MoveToNextLine(1))?;

        // Draw choices
        for (i, choice) in self.choices.iter().enumerate() {
            let is_current = i == self.current_index;
            let is_selected = self.selected_choices[i];

            let marker = if self.allow_multiple {
                if is_selected {
                    "[✓]"
                } else {
                    "[ ]"
                }
            } else if is_current {
                ">"
            } else {
                " "
            };

            let choice_text = format!("{} {}", marker, choice);

            if is_current {
                let colored_choice = self.colorizer.highlighted_text(&choice_text);
                self.colorizer
                    .write_colored(&mut std::io::stderr(), &colored_choice)?;
            } else {
                let colored_choice = self.colorizer.valid_text(&choice_text);
                self.colorizer
                    .write_colored(&mut std::io::stderr(), &colored_choice)?;
            }
            stderr().execute(MoveToNextLine(1))?;
        }

        // Display validation error if present
        if let Some(error_message) = &self.validation_error {
            stderr().execute(MoveToNextLine(1))?;
            let colored_error = self.colorizer.error_message(error_message);
            self.colorizer
                .write_colored(&mut std::io::stderr(), &colored_error)?;
        }

        Ok(())
    }

    fn get_selected_choices(&self) -> Vec<String> {
        self.selected_choices
            .iter()
            .enumerate()
            .filter_map(|(i, &selected)| {
                if selected {
                    Some(self.choices[i].clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Validate current selections against min/max constraints
    fn validate_selections(&mut self) {
        let selected_count = self.selected_choices.iter().filter(|&&s| s).count();
        
        self.validation_error = if selected_count < self.min_choices {
            Some(format!("At least {} choice(s) required", self.min_choices))
        } else if selected_count > self.max_choices {
            Some(format!("At most {} choice(s) allowed", self.max_choices))
        } else {
            None
        };
    }

    /// Check if current selections meet the constraints for submission
    fn can_submit(&self) -> bool {
        let selected_count = self.selected_choices.iter().filter(|&&s| s).count();
        selected_count >= self.min_choices && selected_count <= self.max_choices
    }

    fn calculate_and_reserve_space(&self, _width: u16, _prompt_text: &str) -> Result<u16> {
        use std::io::{self, Write};

        let mut total_lines = 0u16;

        // 1. Prompt line
        total_lines += 1;

        // 2. Instruction line
        total_lines += 1;

        // 3. Choice lines
        total_lines += self.choices.len() as u16;

        // 4. Validation error line (if present)
        total_lines += 1;

        // 5. Buffer for spacing
        total_lines += 2;

        // Ensure we don't try to reserve more lines than the terminal height
        let (_, terminal_height) = self.terminal.size()?;
        let max_reservable = terminal_height.saturating_sub(2);
        total_lines = total_lines.min(max_reservable);

        // Print blank lines to reserve the space
        let mut stderr = io::stderr();
        for _ in 0..total_lines {
            writeln!(stderr)?;
        }
        stderr.flush()?;

        Ok(total_lines)
    }

    fn move_to_prompt_position(&mut self, reserved_lines: u16) -> Result<()> {
        use crossterm::{cursor::MoveUp, ExecutableCommand};
        use std::io::stderr;

        // Move cursor back up to where we want to start the prompt
        stderr().execute(MoveUp(reserved_lines))?;

        Ok(())
    }

}

impl Drop for ChoiceMenu {
    fn drop(&mut self) {
        // Clean up terminal state only if we have cursor control
        if self.terminal.capabilities().cursor_control {
            let _ = self.terminal.leave_raw_mode();
        }
    }
}

#[derive(Debug)]
enum MenuAction {
    Continue,
    Submit,
    Cancel,
}
