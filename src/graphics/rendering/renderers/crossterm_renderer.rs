use crate::geometry::rect::Rect;
use crate::graphics::color::Color;
use crate::graphics::rendering::cell::Cell;
use crate::graphics::rendering::frame::Frame;
use crate::graphics::rendering::renderer::Renderer;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Color as CtColor;
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use crossterm::{execute, queue};
use std::io::{Stdout, Write, stdout};

pub struct CrosstermRenderer {
    rect: Rect,
    out: Stdout,
    prev: Frame,
}

impl CrosstermRenderer {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            out: stdout(),
            prev: Frame::new(rect),
        }
    }
}

impl Renderer for CrosstermRenderer {
    fn present(&mut self, frame: &Frame) -> std::io::Result<()> {
        // Diff current frame against prev, Update only changed cells
        let width = frame.rect.width;
        let height = frame.rect.height;

        for y in 0..height {
            let row_off = y * width;
            for x in 0..width {
                let i = row_off + x;
                let current = frame.cells[i];
                let old = self.prev.cells[i];
                if current != old {
                    queue!(
                        self.out,
                        MoveTo(x as u16, y as u16),
                        SetForegroundColor(to_crossterm(current.foreground)),
                        SetBackgroundColor(to_crossterm(current.background)),
                        Print(current.glyph)
                    )?;
                    // Update shadow buffer
                    self.prev.cells[i] = current;
                }
            }
        }
        self.out.flush()
    }

    fn begin(&mut self) -> std::io::Result<()> {
        self.prev.fill(Cell::default());
        self.clear()?;
        enable_raw_mode()?;
        execute!(self.out, Hide, Clear(ClearType::All))
    }

    fn end(&mut self) -> std::io::Result<()> {
        self.clear()?;
        execute!(self.out, Show)?;
        disable_raw_mode()
    }

    fn clear(&mut self) -> std::io::Result<()> {
        execute!(self.out, Clear(ClearType::All))?;
        execute!(self.out, ResetColor)?;
        execute!(self.out, MoveTo(0, 0))
    }
}

// Color decoupling so I can switch the renderers later
pub fn to_crossterm(color: Color) -> CtColor {
    match color {
        Color::Black => CtColor::Black,
        Color::White => CtColor::White,
        Color::Grey => CtColor::Grey,
        Color::DarkGrey => CtColor::DarkGrey,
        Color::Red => CtColor::Red,
        Color::DarkRed => CtColor::DarkRed,
        Color::Green => CtColor::Green,
        Color::DarkGreen => CtColor::DarkGreen,
        Color::Blue => CtColor::Blue,
        Color::DarkBlue => CtColor::DarkBlue,
        Color::Yellow => CtColor::Yellow,
        Color::DarkYellow => CtColor::DarkYellow,
        Color::Magenta => CtColor::Magenta,
        Color::DarkMagenta => CtColor::DarkMagenta,
        Color::Cyan => CtColor::Cyan,
        Color::DarkCyan => CtColor::DarkCyan,
        Color::Rgb(r, g, b) => CtColor::Rgb { r, g, b },
    }
}
