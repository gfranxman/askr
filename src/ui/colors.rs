use crossterm::style::{
    Attribute, Color, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
};
use crossterm::ExecutableCommand;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub valid_text: Color,
    pub invalid_text: Color,
    pub prompt_text: Color,
    pub help_text: Color,
    pub error_icon: Color,
    pub warning_icon: Color,
    pub info_icon: Color,
    pub success_icon: Color,
    pub background: Option<Color>,
}

impl ColorScheme {
    pub fn default() -> Self {
        Self {
            valid_text: Color::Reset,
            invalid_text: Color::Red,
            prompt_text: Color::Reset,
            help_text: Color::DarkGrey,
            error_icon: Color::Red,
            warning_icon: Color::Yellow,
            info_icon: Color::Blue,
            success_icon: Color::Green,
            background: None,
        }
    }

    pub fn no_color() -> Self {
        Self {
            valid_text: Color::Reset,
            invalid_text: Color::Reset,
            prompt_text: Color::Reset,
            help_text: Color::Reset,
            error_icon: Color::Reset,
            warning_icon: Color::Reset,
            info_icon: Color::Reset,
            success_icon: Color::Reset,
            background: None,
        }
    }

    pub fn high_contrast() -> Self {
        Self {
            valid_text: Color::White,
            invalid_text: Color::Red,
            prompt_text: Color::White,
            help_text: Color::Grey,
            error_icon: Color::Red,
            warning_icon: Color::Yellow,
            info_icon: Color::Cyan,
            success_icon: Color::Green,
            background: Some(Color::Black),
        }
    }
}

pub struct ColoredText {
    pub text: String,
    pub color: Color,
    pub background: Option<Color>,
    pub attributes: Vec<Attribute>,
}

impl ColoredText {
    pub fn new(text: impl Into<String>, color: Color) -> Self {
        Self {
            text: text.into(),
            color,
            background: None,
            attributes: Vec::new(),
        }
    }

    pub fn with_background(mut self, background: Color) -> Self {
        self.background = Some(background);
        self
    }

    pub fn bold(mut self) -> Self {
        self.attributes.push(Attribute::Bold);
        self
    }

    pub fn italic(mut self) -> Self {
        self.attributes.push(Attribute::Italic);
        self
    }

    pub fn underlined(mut self) -> Self {
        self.attributes.push(Attribute::Underlined);
        self
    }
}

#[derive(Clone)]
pub struct Colorizer {
    scheme: ColorScheme,
    no_color: bool,
}

impl Colorizer {
    pub fn new(scheme: ColorScheme, no_color: bool) -> Self {
        Self {
            scheme: if no_color {
                ColorScheme::no_color()
            } else {
                scheme
            },
            no_color,
        }
    }

    pub fn write_colored<W: Write + ExecutableCommand>(
        &self,
        writer: &mut W,
        colored_text: &ColoredText,
    ) -> io::Result<()> {
        if !self.no_color {
            writer.execute(SetForegroundColor(colored_text.color))?;

            if let Some(bg) = colored_text.background {
                writer.execute(SetBackgroundColor(bg))?;
            }

            for attr in &colored_text.attributes {
                writer.execute(SetAttribute(*attr))?;
            }
        }

        write!(writer, "{}", colored_text.text)?;

        if !self.no_color {
            writer.execute(ResetColor)?;
        }

        Ok(())
    }

    pub fn prompt_text(&self, text: impl Into<String>) -> ColoredText {
        ColoredText::new(text, self.scheme.prompt_text).bold()
    }

    pub fn valid_text(&self, text: impl Into<String>) -> ColoredText {
        ColoredText::new(text, self.scheme.valid_text)
    }

    pub fn invalid_text(&self, text: impl Into<String>) -> ColoredText {
        ColoredText::new(text, self.scheme.invalid_text)
    }

    pub fn help_text(&self, text: impl Into<String>) -> ColoredText {
        ColoredText::new(text, self.scheme.help_text)
    }

    pub fn highlighted_text(&self, text: impl Into<String>) -> ColoredText {
        ColoredText::new(text, Color::Black)
            .with_background(Color::White)
            .bold()
    }

    pub fn error_message(&self, text: impl Into<String>) -> ColoredText {
        ColoredText::new(format!("❌ {}", text.into()), self.scheme.error_icon)
    }

    pub fn warning_message(&self, text: impl Into<String>) -> ColoredText {
        ColoredText::new(format!("⚠️ {}", text.into()), self.scheme.warning_icon)
    }

    pub fn info_message(&self, text: impl Into<String>) -> ColoredText {
        ColoredText::new(format!("💡 {}", text.into()), self.scheme.info_icon)
    }

    pub fn success_message(&self, text: impl Into<String>) -> ColoredText {
        ColoredText::new(format!("✅ {}", text.into()), self.scheme.success_icon)
    }

    pub fn no_color_error(&self, text: impl Into<String>) -> String {
        format!("[ERROR] {}", text.into())
    }

    pub fn no_color_warning(&self, text: impl Into<String>) -> String {
        format!("[WARN] {}", text.into())
    }

    pub fn no_color_info(&self, text: impl Into<String>) -> String {
        format!("[INFO] {}", text.into())
    }

    pub fn no_color_success(&self, text: impl Into<String>) -> String {
        format!("[OK] {}", text.into())
    }
}
