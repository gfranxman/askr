use super::colors::{ColoredText, Colorizer};
use crate::validation::{Priority, ValidationResult};
use crossterm::{
    cursor::{MoveTo, MoveToColumn, MoveToNextLine, RestorePosition, SavePosition},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
pub struct LayoutManager {
    width: u16,
    height: u16,
    prompt_line: u16,
    input_line: u16,
    error_area_start: u16,
    error_area_height: u16,
    help_line: Option<u16>,
}

impl LayoutManager {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            prompt_line: 0,
            input_line: 0,
            error_area_start: 1,
            error_area_height: 0,
            help_line: None,
        }
    }

    pub fn calculate_layout(&mut self, has_help: bool) {
        // Simple layout: prompt+input on line 0, errors below, help at bottom
        self.prompt_line = 0;
        self.input_line = 0;
        self.error_area_start = 1;

        if has_help {
            self.help_line = Some(self.height.saturating_sub(1));
        }
    }

    pub fn update_error_area_height(&mut self, error_count: usize) {
        self.error_area_height = error_count.min(10) as u16; // Max 10 lines of errors
    }

    pub fn prompt_position(&self) -> (u16, u16) {
        (0, self.prompt_line)
    }

    pub fn input_position(&self, prompt_width: u16) -> (u16, u16) {
        (prompt_width, self.input_line)
    }

    pub fn error_area_bounds(&self) -> (u16, u16, u16) {
        (self.error_area_start, self.error_area_height, self.width)
    }

    pub fn help_position(&self) -> Option<(u16, u16)> {
        self.help_line.map(|line| (0, line))
    }

    pub fn wrap_text(&self, text: &str, max_width: u16) -> Vec<String> {
        let max_width = max_width as usize;
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            let word_width = word.width();
            let current_width = current_line.width();

            if current_width + word_width < max_width || current_line.is_empty() {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            } else {
                if !current_line.is_empty() {
                    lines.push(current_line);
                }
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }
}

pub struct Screen<W: Write + ExecutableCommand> {
    writer: W,
    layout: LayoutManager,
    colorizer: Colorizer,
    cursor_saved: bool,
}

impl<W: Write + ExecutableCommand> Screen<W> {
    pub fn new(writer: W, layout: LayoutManager, colorizer: Colorizer) -> Self {
        Self {
            writer,
            layout,
            colorizer,
            cursor_saved: false,
        }
    }

    pub fn save_cursor(&mut self) -> io::Result<()> {
        self.writer.execute(SavePosition)?;
        self.cursor_saved = true;
        Ok(())
    }

    pub fn restore_cursor(&mut self) -> io::Result<()> {
        if self.cursor_saved {
            self.writer.execute(RestorePosition)?;
        }
        Ok(())
    }

    pub fn move_to(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.writer.execute(MoveTo(x, y))?;
        Ok(())
    }

    pub fn clear_line(&mut self) -> io::Result<()> {
        self.writer.execute(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn clear_from_cursor(&mut self) -> io::Result<()> {
        self.writer.execute(Clear(ClearType::FromCursorDown))?;
        Ok(())
    }

    pub fn write_at(&mut self, x: u16, y: u16, text: &ColoredText) -> io::Result<()> {
        self.move_to(x, y)?;
        self.colorizer.write_colored(&mut self.writer, text)?;
        Ok(())
    }

    pub fn write_prompt(&mut self, prompt_text: &str) -> io::Result<u16> {
        let colored_prompt = self.colorizer.prompt_text(format!("{} ", prompt_text));

        // Write at current cursor position instead of using layout position
        self.colorizer
            .write_colored(&mut self.writer, &colored_prompt)?;

        // Return the width of the prompt for cursor positioning
        Ok((prompt_text.len() + 1) as u16)
    }

    pub fn write_prompt_at(&mut self, prompt_text: &str, x: u16, y: u16) -> io::Result<u16> {
        let colored_prompt = self.colorizer.prompt_text(format!("{} ", prompt_text));

        self.write_at(x, y, &colored_prompt)?;

        // Return the width of the prompt for cursor positioning
        Ok((prompt_text.len() + 1) as u16)
    }

    pub fn write_input(
        &mut self,
        input: &str,
        prompt_width: u16,
        error_pos: Option<usize>,
    ) -> io::Result<()> {
        // Clear from current position to end of line, then write input
        self.writer
            .execute(crossterm::cursor::MoveToColumn(prompt_width))?;
        self.clear_from_cursor()?;

        if let Some(error_pos) = error_pos {
            // Split input at error position and color accordingly
            let (valid_part, invalid_part) = input.split_at(error_pos.min(input.len()));

            if !valid_part.is_empty() {
                let valid_text = self.colorizer.valid_text(valid_part);
                self.colorizer
                    .write_colored(&mut self.writer, &valid_text)?;
            }

            if !invalid_part.is_empty() {
                let invalid_text = self.colorizer.invalid_text(invalid_part);
                self.colorizer
                    .write_colored(&mut self.writer, &invalid_text)?;
            }
        } else {
            // All text is valid
            let valid_text = self.colorizer.valid_text(input);
            self.colorizer
                .write_colored(&mut self.writer, &valid_text)?;
        }

        Ok(())
    }

    pub fn write_errors(&mut self, errors: &[ValidationResult]) -> io::Result<()> {
        // Store current cursor position to return to later
        self.writer.execute(crossterm::cursor::SavePosition)?;

        // Move to the line after the prompt/input for error display
        self.writer.execute(crossterm::cursor::MoveToNextLine(1))?;

        // Clear from cursor down to clear any old errors
        self.clear_from_cursor()?;

        // Write each error on the following lines
        for error in errors.iter().take(10) {
            // Limit to 10 errors max
            if let Some(message) = &error.message {
                let colored_error = match error.priority {
                    Priority::Critical | Priority::High => self.colorizer.error_message(message),
                    Priority::Medium => self.colorizer.warning_message(message),
                    Priority::Low => self.colorizer.info_message(message),
                };

                // Handle text wrapping
                let wrapped_lines = self
                    .layout
                    .wrap_text(&colored_error.text, self.layout.width);

                for wrapped_line in wrapped_lines {
                    let wrapped_colored = ColoredText::new(wrapped_line, colored_error.color);
                    self.colorizer
                        .write_colored(&mut self.writer, &wrapped_colored)?;
                    self.writer.execute(crossterm::cursor::MoveToNextLine(1))?;
                }
            }
        }

        // Restore cursor to original position (the input line)
        self.writer.execute(crossterm::cursor::RestorePosition)?;

        Ok(())
    }

    pub fn write_help(&mut self, help_text: &str) -> io::Result<()> {
        if let Some((x, y)) = self.layout.help_position() {
            let colored_help = self.colorizer.help_text(help_text);
            self.write_at(x, y, &colored_help)?;
        }
        Ok(())
    }

    pub fn position_cursor_after_input(
        &mut self,
        input: &str,
        prompt_width: u16,
    ) -> io::Result<()> {
        let input_width = input.width() as u16;
        let (_, y) = self.layout.input_position(prompt_width);
        self.move_to(prompt_width + input_width, y)?;
        Ok(())
    }

    pub fn position_cursor_at_input_pos(
        &mut self,
        input: &str,
        cursor_pos: usize,
        prompt_width: u16,
    ) -> io::Result<()> {
        let chars: Vec<char> = input.chars().collect();
        let text_before_cursor: String = chars.iter().take(cursor_pos).collect();
        let cursor_x = prompt_width + text_before_cursor.width() as u16;
        // Move to the correct column on the current line
        self.writer.execute(MoveToColumn(cursor_x))?;
        Ok(())
    }

    pub fn write_choice(&mut self, choice_text: &str) -> io::Result<()> {
        let colored_choice = self.colorizer.valid_text(choice_text);
        self.colorizer
            .write_colored(&mut self.writer, &colored_choice)?;
        self.writer.execute(MoveToNextLine(1))?;
        Ok(())
    }

    pub fn write_highlighted_choice(&mut self, choice_text: &str) -> io::Result<()> {
        let colored_choice = self.colorizer.highlighted_text(choice_text);
        self.colorizer
            .write_colored(&mut self.writer, &colored_choice)?;
        self.writer.execute(MoveToNextLine(1))?;
        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }

    pub fn layout_mut(&mut self) -> &mut LayoutManager {
        &mut self.layout
    }
}
