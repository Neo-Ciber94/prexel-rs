
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    Black = 0,
    DarkBlue = 1,
    DarkGreen = 2,
    DarkCyan = 3,
    DarkRed = 4,
    DarkMagenta = 5,
    DarkYellow = 6,
    Gray = 7,
    DarkGray = 8,
    Blue = 9,
    Green = 10,
    Cyan = 11,
    Red = 12,
    Magenta = 13,
    Yellow = 14,
    White = 15,
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