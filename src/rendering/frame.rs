use crate::geometry::rect::Rect;
use crate::rendering::cell::Cell;
use crossterm::style::Color;

pub struct Frame {
    pub rect: Rect,
    pub cells: Vec<Cell>,
}

impl Frame {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            cells: vec![Cell::default(); rect.area()],
        }
    }

    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.rect.width + x
    }

    pub(crate) fn clear(&mut self, fill: Cell) {
        self.cells.fill(fill);
    }

    pub(crate) fn put(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.rect.width && y < self.rect.height {
            let idx = self.idx(x, y);
            self.cells[idx] = cell;
        }
    }

    pub(crate) fn put_str(
        &mut self,
        x: usize,
        y: usize,
        string: &str,
        foreground: Color,
        background: Color,
    ) {
        for (i, glyph) in string.chars().enumerate() {
            self.put(
                x + i,
                y,
                Cell {
                    glyph,
                    foreground,
                    background,
                },
            );
        }
    }
}
