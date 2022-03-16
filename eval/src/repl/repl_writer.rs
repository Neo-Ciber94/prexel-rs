use crate::style::TextStyling;
use crate::{impl_colored_methods, ColorWriter};
use crossterm::cursor::MoveLeft;
use crossterm::execute;
use crossterm::style::Color;
use crossterm::terminal::{Clear, ClearType};
use std::fmt::Display;

pub struct ReplWriter {
    writer: ColorWriter,
    prompt_prefix: Option<String>
}

#[allow(unused)]
impl ReplWriter {
    pub fn new() -> Self {
        let writer = ColorWriter::new();

        ReplWriter {
            writer,
            prompt_prefix: None,
        }
    }

    pub fn with_prompt_prefix<S: Into<String>>(prefix: S) -> Self {
        let writer = ColorWriter::new();

        ReplWriter {
            writer,
            prompt_prefix: Some(prefix.into()),
        }
    }

    pub fn fg(&mut self, color: Option<Color>) -> &mut Self {
        self.writer.fg(color);
        self
    }

    pub fn bg(&mut self, color: Option<Color>) -> &mut Self {
        self.writer.bg(color);
        self
    }

    pub fn styled(&mut self, styling: TextStyling) -> &mut Self {
        self.writer.styled(styling);
        self
    }

    pub fn flush(&self) {
        self.writer.flush();
    }

    pub fn write_prompt_prefix(&mut self) {
        if let Some(prefix) = self.prompt_prefix.clone() {
            self.cyan().write(prefix);
        }
    }

    pub fn write<D: Display>(&mut self, data: D) {
        self.writer.write(data);
    }

    pub fn writeln<D: Display>(&mut self, data: D) {
        self.writer.writeln(data);
        self.write_prompt_prefix();
    }

    pub fn write_err<D: Display>(&mut self, data: D) {
        self.writer.write_err(data);
    }

    pub fn writeln_err<D: Display>(&mut self, data: D) {
        self.writer.writeln_err(data);
        self.write_prompt_prefix();
    }

    pub fn rewrite<D: Display>(&mut self, data: D) {
        let fg = self.writer.fg.clone();
        let bg = self.writer.bg.clone();

        execute!(
            std::io::stdout(),
            Clear(ClearType::CurrentLine),
            MoveLeft(u16::MAX)
        )
        .unwrap();

        self.write_prompt_prefix();
        self.fg(fg).bg(bg).write(data);
    }

    pub fn rewrite_err<D: Display>(&mut self, data: D) {
        let fg = self.writer.fg.clone();
        let bg = self.writer.bg.clone();

        execute!(
            std::io::stderr(),
            Clear(ClearType::CurrentLine),
            MoveLeft(u16::MAX)
        )
        .unwrap();

        self.write_prompt_prefix();
        self.fg(fg).bg(bg).write(data);
    }

    impl_colored_methods! {
        // Foreground Colors
        fg red Red,
        fg green Green,
        fg yellow Yellow,
        fg blue Blue,
        fg magenta Magenta,
        fg cyan Cyan,
        fg white White,
        fg gray Grey,
        fg black Black,
        fg dark_red DarkRed,
        fg dark_green DarkGreen,
        fg dark_yellow DarkYellow,
        fg dark_blue DarkBlue,
        fg dark_magenta DarkMagenta,
        fg dark_cyan DarkCyan,
        fg dark_gray DarkGrey,

        // Foreground Colors
        bg on_red Red,
        bg on_green Green,
        bg on_yellow Yellow,
        bg on_blue Blue,
        bg on_magenta Magenta,
        bg on_cyan Cyan,
        bg on_white White,
        bg on_gray Grey,
        bg on_black Black,
        bg on_dark_red DarkRed,
        bg on_dark_green DarkGreen,
        bg on_dark_yellow DarkYellow,
        bg on_dark_blue DarkBlue,
        bg on_dark_magenta DarkMagenta,
        bg on_dark_cyan DarkCyan,
        bg on_dark_gray DarkGrey,
    }
}
