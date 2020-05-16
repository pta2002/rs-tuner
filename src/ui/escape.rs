use std::fmt;

const ESC: char = 27u8 as char;

/// Defines some escape codes that can be used for formatting terminal output
pub enum EscapeCode {
    FgColor(Color),
    BgColor(Color),
    HideCursor,
    ShowCursor,
    Reset,
}

impl fmt::Display for EscapeCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EscapeCode::FgColor(c) => write!(f, "{}[3{}m", ESC, c),
            EscapeCode::BgColor(c) => write!(f, "{}[4{}m", ESC, c),
            EscapeCode::Reset => write!(f, "{}[0m", ESC),
            EscapeCode::HideCursor => write!(f, "{}[?25l", ESC),
            EscapeCode::ShowCursor => write!(f, "{}[?25h", ESC),
        }
    }
}

/// Defines colors to be used in terminal output
#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Other(u8),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = match self {
            Color::Other(i) => *i,
            Color::Black => 0,
            Color::Red => 1,
            Color::Green => 2,
            Color::Yellow => 3,
            Color::Blue => 4,
            Color::Magenta => 5,
            Color::Cyan => 6,
            Color::White => 7,
        };

        write!(f, "{}", n)
    }
}

impl Color {
    pub fn fg(self) -> EscapeCode {
        EscapeCode::FgColor(self)
    }

    pub fn bg(self) -> EscapeCode {
        EscapeCode::BgColor(self)
    }
}
