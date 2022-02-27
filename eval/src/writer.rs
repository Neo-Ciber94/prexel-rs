use std::fmt::Display;
use std::io::Write;
use termcolor::{ColorSpec, StandardStream, WriteColor};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    White,
    Black,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Intense {
    Yes,
    No,
}

impl From<Intense> for bool {
    fn from(intense: Intense) -> Self {
        match intense {
            Intense::Yes => true,
            Intense::No => false,
        }
    }
}

impl Color {
    pub fn into_color(self) -> termcolor::Color {
        match self {
            Color::Red => termcolor::Color::Red,
            Color::Green => termcolor::Color::Green,
            Color::Blue => termcolor::Color::Blue,
            Color::Yellow => termcolor::Color::Yellow,
            Color::Cyan => termcolor::Color::Cyan,
            Color::Magenta => termcolor::Color::Magenta,
            Color::White => termcolor::Color::White,
            Color::Black => termcolor::Color::Black,
        }
    }
}

pub struct ColorWriter {
    stdout: StandardStream,
    stderr: StandardStream,
}

#[allow(dead_code)]
impl ColorWriter {
    pub fn new(use_color: bool) -> Self {
        let color_choice = if use_color {
            termcolor::ColorChoice::Auto
        } else {
            termcolor::ColorChoice::Never
        };
        let stdout = StandardStream::stdout(color_choice);
        let stderr = StandardStream::stderr(color_choice);

        ColorWriter { stdout, stderr }
    }

    pub fn new_colored() -> Self {
        Self::new(true)
    }

    pub fn writeln(&mut self) {
        writeln!(self.stdout).unwrap();
    }

    pub fn write<S: Display>(&mut self, color: Color, intense: Intense, s: S) {
        self.stdout
            .set_color(
                ColorSpec::new()
                    .set_fg(Some(color.into_color()))
                    .set_intense(intense.into()),
            )
            .unwrap();

        write!(self.stdout, "{}", s).unwrap();
        self.stdout.flush().unwrap();
        self.stdout.reset().unwrap();
    }

    pub fn write_err<S: Display>(&mut self, color: Color, intense: Intense, s: S) {
        self.stderr
            .set_color(
                ColorSpec::new()
                    .set_fg(Some(color.into_color()))
                    .set_intense(intense.into()),
            )
            .unwrap();

        write!(self.stderr, "{}", s).unwrap();
        self.stderr.flush().unwrap();
        self.stderr.reset().unwrap();
    }

    pub fn write_red<S: Display>(&mut self, intense: Intense, s: S) {
        self.write(Color::Red, intense, s);
    }

    pub fn write_green<S: Display>(&mut self, intense: Intense, s: S) {
        self.write(Color::Green, intense, s);
    }

    pub fn write_blue<S: Display>(&mut self, intense: Intense, s: S) {
        self.write(Color::Blue, intense, s);
    }

    pub fn write_yellow<S: Display>(&mut self, intense: Intense, s: S) {
        self.write(Color::Yellow, intense, s);
    }

    pub fn write_cyan<S: Display>(&mut self, intense: Intense, s: S) {
        self.write(Color::Cyan, intense, s);
    }

    pub fn write_magenta<S: Display>(&mut self, intense: Intense, s: S) {
        self.write(Color::Magenta, intense, s);
    }

    pub fn write_white<S: Display>(&mut self, intense: Intense, s: S) {
        self.write(Color::White, intense, s);
    }

    pub fn write_black<S: Display>(&mut self, intense: Intense, s: S) {
        self.write(Color::Black, intense, s);
    }

    pub fn write_err_red<S: Display>(&mut self, intense: Intense, s: S) {
        self.write_err(Color::Red, intense, s);
    }

    pub fn write_err_green<S: Display>(&mut self, intense: Intense, s: S) {
        self.write_err(Color::Green, intense, s);
    }

    pub fn write_err_blue<S: Display>(&mut self, intense: Intense, s: S) {
        self.write_err(Color::Blue, intense, s);
    }

    pub fn write_err_yellow<S: Display>(&mut self, intense: Intense, s: S) {
        self.write_err(Color::Yellow, intense, s);
    }

    pub fn write_err_cyan<S: Display>(&mut self, intense: Intense, s: S) {
        self.write_err(Color::Cyan, intense, s);
    }

    pub fn write_err_magenta<S: Display>(&mut self, intense: Intense, s: S) {
        self.write_err(Color::Magenta, intense, s);
    }

    pub fn write_err_white<S: Display>(&mut self, intense: Intense, s: S) {
        self.write_err(Color::White, intense, s);
    }

    pub fn write_err_black<S: Display>(&mut self, intense: Intense, s: S) {
        self.write_err(Color::Black, intense, s);
    }
}

#[macro_export]
macro_rules! readln {
    () => {{
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf
    }};
}
