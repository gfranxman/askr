use crossterm::{
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    tty::IsTty,
    ExecutableCommand,
};
use std::io::{self, stdout, Stdout};

#[derive(Debug, Clone)]
pub struct TerminalCapabilities {
    pub colors_supported: bool,
    pub cursor_control: bool,
    pub unicode_support: bool,
    pub width: u16,
    pub height: u16,
    pub supports_alternate_screen: bool,
}

impl TerminalCapabilities {
    pub fn detect() -> io::Result<Self> {
        // Check if we're in a TTY
        if !io::stdout().is_tty() {
            return Ok(Self {
                colors_supported: false,
                cursor_control: false,
                unicode_support: true,
                width: 80,  // Default width
                height: 24, // Default height
                supports_alternate_screen: false,
            });
        }

        let (width, height) = terminal::size()?;

        Ok(Self {
            colors_supported: true, // Most modern terminals support colors
            cursor_control: true,   // crossterm handles this
            unicode_support: true,  // Assume unicode support
            width,
            height,
            supports_alternate_screen: false, // We don't use alternate screen mode
        })
    }


    pub fn fallback_ui(&self) -> UIMode {
        if !self.cursor_control {
            UIMode::Simple
        } else if !self.colors_supported {
            UIMode::NoColor
        } else {
            UIMode::Full
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UIMode {
    Full,    // Full UI with colors and cursor control
    NoColor, // No colors but cursor control
    Simple,  // Basic text only
}

pub struct Terminal {
    stdout: Stdout,
    capabilities: TerminalCapabilities,
    original_hook: Option<Box<dyn Fn() + Send + Sync>>,
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
        let capabilities = TerminalCapabilities::detect()?;

        Ok(Self {
            stdout: stdout(),
            capabilities,
            original_hook: None,
        })
    }


    pub fn enter_raw_mode(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        Ok(())
    }

    pub fn leave_raw_mode(&mut self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn enter_alternate_screen(&mut self) -> io::Result<()> {
        if self.capabilities.supports_alternate_screen {
            self.stdout.execute(EnterAlternateScreen)?;
        }
        Ok(())
    }

    pub fn leave_alternate_screen(&mut self) -> io::Result<()> {
        if self.capabilities.supports_alternate_screen {
            self.stdout.execute(LeaveAlternateScreen)?;
        }
        Ok(())
    }

    pub fn capabilities(&self) -> &TerminalCapabilities {
        &self.capabilities
    }

    pub fn size(&self) -> io::Result<(u16, u16)> {
        terminal::size()
    }

    pub fn clear_screen(&mut self) -> io::Result<()> {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        use std::io::Write;
        self.stdout.flush()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        // Ensure we clean up properly
        let _ = self.leave_alternate_screen();
        let _ = self.leave_raw_mode();
    }
}
