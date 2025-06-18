use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io::{self, stderr, Write};
use std::time::Duration;
use super::{Terminal, ColorScheme, Colorizer, LayoutManager, Screen};
use crate::error::{PromptError, Result};

pub struct ChoiceMenu {
    terminal: Terminal,
    choices: Vec<String>,
    allow_multiple: bool,
    selected_choices: Vec<bool>,
    current_index: usize,
    colorizer: Colorizer,
}

impl ChoiceMenu {
    pub fn new(
        mut terminal: Terminal,
        choices: Vec<String>,
        allow_multiple: bool,
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
            selected_choices,
            current_index: 0,
            colorizer,
        })
    }

    pub fn show(&mut self, prompt_text: &str) -> Result<Vec<String>> {
        let (width, height) = self.terminal.size()?;
        let layout = LayoutManager::new(width, height);
        let mut screen = Screen::new(stderr(), layout, self.colorizer.clone());

        // Calculate space needed and reserve it
        let reserved_lines = self.calculate_and_reserve_space(width, prompt_text)?;

        // Move cursor to the reserved prompt position
        self.move_to_prompt_position(reserved_lines)?;

        // Draw initial menu
        self.draw_menu(&mut screen, prompt_text)?;

        // Input loop
        loop {
            screen.flush()?;

            let timeout = Duration::from_secs(300);
            if event::poll(timeout)? {
                if let Event::Key(key_event) = event::read()? {
                    match self.handle_key_event(key_event)? {
                        MenuAction::Continue => {
                            // Clear the area and redraw
                            self.clear_and_redraw(&mut screen, prompt_text)?;
                        }
                        MenuAction::Submit => {
                            let selected = self.get_selected_choices();
                            if self.allow_multiple || selected.len() == 1 {
                                return Ok(selected);
                            }
                            // If single choice mode and no selection, continue
                            if selected.is_empty() {
                                self.draw_menu(&mut screen, prompt_text)?;
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
            KeyEvent { code: KeyCode::Up, .. } => {
                if self.current_index > 0 {
                    self.current_index -= 1;
                }
                Ok(MenuAction::Continue)
            }
            KeyEvent { code: KeyCode::Down, .. } => {
                if self.current_index < self.choices.len() - 1 {
                    self.current_index += 1;
                }
                Ok(MenuAction::Continue)
            }

            // Selection
            KeyEvent { code: KeyCode::Enter, .. } => {
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
            KeyEvent { code: KeyCode::Char(' '), .. } => {
                if self.allow_multiple {
                    self.selected_choices[self.current_index] = !self.selected_choices[self.current_index];
                }
                Ok(MenuAction::Continue)
            }

            // Cancel with Ctrl+C
            KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, .. } => {
                Ok(MenuAction::Cancel)
            }

            // Cancel with Ctrl+D (EOF)
            KeyEvent { code: KeyCode::Char('d'), modifiers: KeyModifiers::CONTROL, .. } => {
                Ok(MenuAction::Cancel)
            }

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

    fn clear_and_redraw(&self, screen: &mut Screen<io::Stderr>, prompt_text: &str) -> Result<()> {
        use crossterm::{cursor::{MoveTo, MoveUp}, terminal::Clear, terminal::ClearType, ExecutableCommand};

        // Move back to the start of the menu area (after the prompt)
        let total_menu_lines = 1 + self.choices.len() as u16; // instruction + choices
        stderr().execute(MoveUp(total_menu_lines))?;
        
        // Clear from cursor down to remove old menu
        stderr().execute(Clear(ClearType::FromCursorDown))?;
        
        // Redraw the menu
        self.draw_menu_content()?;
        
        Ok(())
    }

    fn draw_menu_content(&self) -> Result<()> {
        use crossterm::{cursor::MoveToNextLine, ExecutableCommand};

        // Add instruction text
        let instruction = if self.allow_multiple {
            "Use ↑↓ to navigate, SPACE to toggle, ENTER to submit:"
        } else {
            "Use ↑↓ to navigate, ENTER to select:"
        };
        
        // Write instruction
        let colored_instruction = self.colorizer.help_text(instruction);
        self.colorizer.write_colored(&mut std::io::stderr(), &colored_instruction)?;
        stderr().execute(MoveToNextLine(1))?;

        // Draw choices
        for (i, choice) in self.choices.iter().enumerate() {
            let is_current = i == self.current_index;
            let is_selected = self.selected_choices[i];

            let marker = if self.allow_multiple {
                if is_selected { "[✓]" } else { "[ ]" }
            } else {
                if is_current { ">" } else { " " }
            };

            let choice_text = format!("{} {}", marker, choice);
            
            if is_current {
                let colored_choice = self.colorizer.highlighted_text(&choice_text);
                self.colorizer.write_colored(&mut std::io::stderr(), &colored_choice)?;
            } else {
                let colored_choice = self.colorizer.valid_text(&choice_text);
                self.colorizer.write_colored(&mut std::io::stderr(), &colored_choice)?;
            }
            stderr().execute(MoveToNextLine(1))?;
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

    fn calculate_and_reserve_space(&self, _width: u16, _prompt_text: &str) -> Result<u16> {
        use std::io::{self, Write};

        let mut total_lines = 0u16;

        // 1. Prompt line
        total_lines += 1;

        // 2. Instruction line
        total_lines += 1;

        // 3. Choice lines
        total_lines += self.choices.len() as u16;

        // 4. Buffer for spacing
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