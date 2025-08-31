use crate::geometry::point::Point;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Rect {
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }

    pub fn contains(&self, point: Point) -> bool {
        (point.x >= 0 && point.x < self.width) && (point.y >= 0 && point.y < self.height)
    }
}
