use crate::graphics::color::Color;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Cell {
    pub glyph: char,
    pub foreground: Color,
    pub background: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            glyph: ' ',
            foreground: Color::Black,
            background: Color::Black,
        }
    }
}
