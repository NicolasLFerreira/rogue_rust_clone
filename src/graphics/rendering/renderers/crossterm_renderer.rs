use crate::geometry::rect::Rect;
use crate::graphics::rendering::cell::Cell;
use crate::graphics::rendering::frame::Frame;
use crate::graphics::rendering::renderer::Renderer;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
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
                        SetForegroundColor(current.foreground),
                        SetBackgroundColor(current.background),
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
