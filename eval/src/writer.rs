use std::fmt::Display;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use crossterm::style::{Color, StyledContent, Stylize};
use crate::style::TextStyling;

#[macro_export]
macro_rules! impl_colored_methods {
    ($($source_method:tt $method_name:tt $color:ident),* $(,)?) => {
        $(
            pub fn $method_name(&mut self) -> &mut Self {
                self.$source_method(Some(Color::$color))
            }
        )*
    }
}

/// Whether if use or not colored output.
static USE_COLORS : AtomicBool = AtomicBool::new(true);

pub fn use_colors() -> bool {
    USE_COLORS.load(Ordering::SeqCst)
}

pub fn set_use_colors(value: bool) {
    USE_COLORS.store(value, Ordering::SeqCst);
}

#[derive(Debug, Clone)]
pub struct ColorWriter {
    pub(crate) fg: Option<Color>,
    pub(crate) bg: Option<Color>
}

#[allow(unused)]
impl ColorWriter {
    pub fn new() -> Self {
        ColorWriter {
            fg: None,
            bg: None
        }
    }

    pub fn flush(&self) {
        std::io::stdout().flush().unwrap();
        std::io::stderr().flush().unwrap();
    }

    pub fn fg(&mut self, color: Option<Color>) -> &mut Self {
        self.fg = color;
        self
    }

    pub fn bg(&mut self, color: Option<Color>) -> &mut Self {
        self.bg = color;
        self
    }

    pub fn styled(&mut self, styling: TextStyling) -> &mut Self {
        self.fg(styling.fg);
        self.bg(styling.bg);
        self
    }

    pub fn write<D: Display>(&mut self, data: D) {
        if !use_colors() {
            print!("{}", data);
        } else {
            let mut styled = StyledContent::new(Default::default(), data);

            if let Some(fg) = self.fg {
                styled = styled.with(fg);
            }

            if let Some(bg) = self.bg {
                styled = styled.on(bg);
            }

            print!("{}", styled);
            self.reset();
        }
    }

    pub fn writeln<D: Display>(&mut self, data: D) {
        self.write(data);
        self.write("\n");
    }

    pub fn write_err<D: Display>(&mut self, data: D) {
        if !use_colors() {
            eprint!("{}", data);
        } else {
            let mut styled = StyledContent::new(Default::default(), data);

            if let Some(fg) = self.fg {
                styled = styled.with(fg);
            }

            if let Some(bg) = self.bg {
                styled = styled.on(bg);
            }

            eprint!("{}", styled);
            self.reset();
        }
    }

    pub fn writeln_err<D: Display>(&mut self, data: D) {
        self.write_err(data);
        self.write_err("\n");
    }

    pub fn reset(&mut self) -> &mut Self {
        self.fg = None;
        self.bg = None;
        self
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