use std::fmt::Display;
use crossterm::style::Color;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextStyling {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

#[allow(unused)]
impl TextStyling {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn fg(mut self, color: Option<Color>) -> Self {
        self.fg = color;
        self
    }

    pub fn bg(mut self, color: Option<Color>) -> Self {
        self.bg = color;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColoredText<D> {
    pub content: D,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

#[allow(unused)]
impl<D: Display> ColoredText<D> {
    pub fn new(content: D) -> Self {
        ColoredText {
            content,
            fg: None,
            bg: None,
        }
    }

    pub fn fg(mut self, color: Option<Color>) -> Self {
        self.fg = color;
        self
    }

    pub fn bg(mut self, color: Option<Color>) -> Self {
        self.bg = color;
        self
    }
}