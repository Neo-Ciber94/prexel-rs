use crossterm::style::Color;
use crate::ColorWriter;
use crate::style::{ColoredText, TextStyling};

pub trait ReplTheme {
    fn write_prompt(&mut self, writer: &mut ColorWriter, prompt: &str);
    fn write_prompt_prefix(&mut self, writer: &mut ColorWriter);
    fn write_error(&mut self, writer: &mut ColorWriter, error: &str);
    fn write_start_text(&mut self, writer: &mut ColorWriter, text: &str);
    fn write_exit_text(&mut self, writer: &mut ColorWriter, text: &str);

    fn writeln_prompt(&mut self, writer: &mut ColorWriter, prompt: &str);
    fn writeln_error(&mut self, writer: &mut ColorWriter, error: &str);
}

pub struct SimpleTheme {
    prompt: Option<TextStyling>,
    prompt_prefix: Option<ColoredText<String>>,
    error: Option<TextStyling>,
    start_text: Option<TextStyling>,
    exit_text: Option<TextStyling>,
}

#[allow(unused)]
impl SimpleTheme {
    pub fn new() -> Self {
        SimpleTheme {
            prompt: None,
            prompt_prefix: None,
            error: None,
            start_text: None,
            exit_text: None,
        }
    }

    pub fn builder() -> SimpleThemeBuilder {
        SimpleThemeBuilder::new()
    }
}

#[allow(unused)]
impl ReplTheme for SimpleTheme {
    fn write_prompt(&mut self, writer: &mut ColorWriter, prompt: &str) {
        if let Some(style) = self.prompt {
            writer.styled(style).write(prompt);
        } else {
            writer.write(prompt);
        }
    }

    fn write_prompt_prefix(&mut self, writer: &mut ColorWriter) {
        if let Some(style) = &self.prompt_prefix {
            writer.fg(style.fg)
                .bg(style.bg)
                .write(&style.content);
        }
    }

    fn write_error(&mut self, writer: &mut ColorWriter, error: &str) {
        if let Some(style) = self.error {
            writer.styled(style).write_err(error);
        } else {
            writer.write_err(error);
        }
    }

    fn write_start_text(&mut self, writer: &mut ColorWriter, text: &str) {
        if let Some(style) = self.start_text {
            writer.styled(style).writeln(text);
        } else {
            writer.writeln(text);
        }
    }

    fn write_exit_text(&mut self, writer: &mut ColorWriter, text: &str) {
        if let Some(style) = self.exit_text {
            writer.styled(style).writeln(text);
        } else {
            writer.writeln(text);
        }
    }

    fn writeln_prompt(&mut self, writer: &mut ColorWriter, prompt: &str) {
        if let Some(style) = self.prompt {
            writer.styled(style).writeln(prompt);
        } else {
            writer.writeln(prompt);
        }
    }

    fn writeln_error(&mut self, writer: &mut ColorWriter, error: &str) {
        if let Some(style) = self.error {
            writer.styled(style).writeln_err(error);
        } else {
            writer.writeln_err(error);
        }
    }
}

pub struct SimpleThemeBuilder {
    prompt: Option<TextStyling>,
    prompt_prefix: Option<ColoredText<String>>,
    error: Option<TextStyling>,
    start_text: Option<TextStyling>,
    exit_text: Option<TextStyling>,
}

#[allow(unused)]
impl SimpleThemeBuilder {
    pub fn new() -> Self {
        SimpleThemeBuilder {
            prompt: None,
            prompt_prefix: None,
            error: None,
            start_text: None,
            exit_text: None,
        }
    }

    pub fn prompt(mut self, prompt: Option<TextStyling>) -> Self {
        self.prompt = prompt;
        self
    }

    pub fn prompt_prefix(mut self, prompt_prefix: Option<ColoredText<String>>) -> Self {
        self.prompt_prefix = prompt_prefix;
        self
    }

    pub fn error(mut self, error: Option<TextStyling>) -> Self {
        self.error = error;
        self
    }

    pub fn start_text(mut self, start_text: Option<TextStyling>) -> Self {
        self.start_text = start_text;
        self
    }

    pub fn exit_text(mut self, exit_text: Option<TextStyling>) -> Self {
        self.exit_text = exit_text;
        self
    }

    pub fn build(self) -> SimpleTheme {
        SimpleTheme {
            prompt: self.prompt,
            prompt_prefix: self.prompt_prefix,
            error: self.error,
            start_text: self.start_text,
            exit_text: self.exit_text,
        }
    }
}

pub struct DefaultTheme;

#[allow(unused)]
impl ReplTheme for DefaultTheme {
    fn write_prompt(&mut self, writer: &mut ColorWriter, prompt: &str) {
        writer.write(prompt);
    }

    fn write_prompt_prefix(&mut self, writer: &mut ColorWriter) {
        writer.fg(Some(Color::Cyan)).write(">>> ");
    }

    fn write_error(&mut self, writer: &mut ColorWriter, error: &str) {
        writer.fg(Some(Color::Red)).write_err(error);
    }

    fn write_start_text(&mut self, writer: &mut ColorWriter, text: &str) {
        writer.fg(Some(Color::Green)).write(text);
    }

    fn write_exit_text(&mut self, writer: &mut ColorWriter, text: &str) {
        writer.fg(Some(Color::Yellow)).write(text);
    }

    fn writeln_prompt(&mut self, writer: &mut ColorWriter, prompt: &str) {
        writer.writeln(prompt);
    }

    fn writeln_error(&mut self, writer: &mut ColorWriter, error: &str) {
        writer.fg(Some(Color::Red)).writeln_err(error);
    }
}