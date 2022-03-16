#![allow(unused)]
use std::fmt::Display;
use crossterm::style::Color;
use crate::ColorWriter;
use crate::style::{ColoredText, TextStyling};

pub trait ReplTheme {
    fn write_prompt_prefix(&mut self, writer: &mut ColorWriter);
    fn write_prompt<D: Display>(&mut self, writer: &mut ColorWriter, prompt: D);
    fn write_error<D: Display>(&mut self, writer: &mut ColorWriter, error: D);
    fn write_start_text<D: Display>(&mut self, writer: &mut ColorWriter, text: D);
    fn write_exit_text<D: Display>(&mut self, writer: &mut ColorWriter, text: D);

    fn writeln_prompt<D: Display>(&mut self, writer: &mut ColorWriter, prompt: D);
    fn writeln_error<D: Display>(&mut self, writer: &mut ColorWriter, error: D);
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
    fn write_prompt_prefix(&mut self, writer: &mut ColorWriter) {
        if let Some(style) = &self.prompt_prefix {
            writer.fg(style.fg)
                .bg(style.bg)
                .write(&style.content);
        }
    }

    fn write_prompt<D: Display>(&mut self, writer: &mut ColorWriter, prompt: D) {
        if let Some(style) = self.prompt {
            writer.styled(style).write(prompt);
        } else {
            writer.write(prompt);
        }
    }

    fn write_error<D: Display>(&mut self, writer: &mut ColorWriter, error: D) {
        if let Some(style) = self.error {
            writer.styled(style).write_err(error);
        } else {
            writer.write_err(error);
        }
    }

    fn write_start_text<D: Display>(&mut self, writer: &mut ColorWriter, text: D) {
        if let Some(style) = self.start_text {
            writer.styled(style).writeln(text);
        } else {
            writer.writeln(text);
        }
    }

    fn write_exit_text<D: Display>(&mut self, writer: &mut ColorWriter, text: D) {
        if let Some(style) = self.exit_text {
            writer.styled(style).writeln(text);
        } else {
            writer.writeln(text);
        }
    }

    fn writeln_prompt<D: Display>(&mut self, writer: &mut ColorWriter, prompt: D) {
        if let Some(style) = self.prompt {
            writer.styled(style).writeln(prompt);
        } else {
            writer.writeln(prompt);
        }
    }

    fn writeln_error<D: Display>(&mut self, writer: &mut ColorWriter, error: D) {
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
impl ReplTheme for DefaultTheme {
    fn write_prompt_prefix(&mut self, writer: &mut ColorWriter) {
        writer.fg(Some(Color::Cyan)).write(">>> ");
    }

    fn write_prompt<D: Display>(&mut self, writer: &mut ColorWriter, prompt: D) {
        writer.write(prompt);
    }

    fn write_error<D: Display>(&mut self, writer: &mut ColorWriter, error: D) {
        writer.fg(Some(Color::Red)).write_err(error);
    }

    fn write_start_text<D: Display>(&mut self, writer: &mut ColorWriter, text: D) {
        writer.fg(Some(Color::Green)).writeln(text);
    }

    fn write_exit_text<D: Display>(&mut self, writer: &mut ColorWriter, text: D) {
        writer.fg(Some(Color::Yellow)).writeln(text);
    }

    fn writeln_prompt<D: Display>(&mut self, writer: &mut ColorWriter, prompt: D) {
        writer.writeln(prompt);
    }

    fn writeln_error<D: Display>(&mut self, writer: &mut ColorWriter, error: D) {
        writer.fg(Some(Color::Red)).writeln_err(error);
    }
}

pub struct NoTheme;
impl ReplTheme for NoTheme {
    fn write_prompt_prefix(&mut self, writer: &mut ColorWriter) {}

    #[inline]
    fn write_prompt<D: Display>(&mut self, writer: &mut ColorWriter, prompt: D) {
        writer.write(prompt);
    }

    #[inline]
    fn write_error<D: Display>(&mut self, writer: &mut ColorWriter, error: D) {
        writer.write_err(error);
    }

    #[inline]
    fn write_start_text<D: Display>(&mut self, writer: &mut ColorWriter, text: D) {
        writer.writeln(text);
    }

    #[inline]
    fn write_exit_text<D: Display>(&mut self, writer: &mut ColorWriter, text: D) {
        writer.writeln(text);
    }

    #[inline]
    fn writeln_prompt<D: Display>(&mut self, writer: &mut ColorWriter, prompt: D) {
        writer.writeln(prompt);
    }

    #[inline]
    fn writeln_error<D: Display>(&mut self, writer: &mut ColorWriter, error: D) {
        writer.writeln_err(error);
    }
}