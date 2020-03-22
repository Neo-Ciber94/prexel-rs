
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Yellow = 6,
    White = 7,
    Gray = 8,
    BrightBlue = 9,
    BrightGreen = 10,
    BrightCyan = 11,
    BrightRed = 12,
    BrightMagenta = 13,
    BrightYellow = 14,
    BrightWhite = 15,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConsoleColor{
    foreground: Option<Color>,
    background: Option<Color>
}

impl ConsoleColor{
    #[inline]
    pub const fn new(foreground: Color, background: Color) -> Self{
        ConsoleColor{
            foreground: Some(foreground),
            background: Some(background)
        }
    }

    #[inline]
    pub const fn with_foreground(color: Color) -> Self{
        ConsoleColor{
            foreground: Some(color),
            background: None
        }
    }

    #[inline]
    pub const fn with_background(color: Color) -> Self{
        ConsoleColor{
            foreground: None,
            background: Some(color)
        }
    }

    #[inline]
    pub const fn foreground(&self) -> Option<Color>{
        self.foreground
    }

    #[inline]
    pub const fn background(&self) -> Option<Color>{
        self.background
    }
}