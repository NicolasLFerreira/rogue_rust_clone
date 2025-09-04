use crate::geometry::point::Point;
use crate::geometry::rect::Rect;
use crate::graphics::rendering::cell::Cell;
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
    fn idx(&self, point: Point) -> usize {
        point.y * self.rect.width + point.x
    }

    pub fn clear(&mut self) {
        self.fill(Cell {
            glyph: ' ',
            foreground: Color::Black,
            background: Color::Black,
        })
    }

    pub(crate) fn fill(&mut self, fill: Cell) {
        self.cells.fill(fill);
    }

    pub(crate) fn put(&mut self, point: Point, cell: Cell) {
        if self.rect.contains(point) {
            let idx = self.idx(point);
            self.cells[idx] = cell;
        }
    }

    pub(crate) fn put_str(
        &mut self,
        point: Point,
        string: &str,
        foreground: Color,
        background: Color,
    ) {
        for (i, glyph) in string.chars().enumerate() {
            self.put(
                point + (i, 0),
                Cell {
                    glyph,
                    foreground,
                    background,
                },
            );
        }
    }
}
