use crate::geometry::point::Point;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }

    pub fn contains(&self, point: Point) -> bool {
        (point.x >= 0 && point.x < self.width) && (point.y >= 0 && point.y < self.height)
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Point> {
        let x_start = self.x;
        let y_start = self.y;
        let width = self.width;
        let height = self.height;

        (0..height).flat_map(move |dy| {
            let y = y_start + dy;
            (0..width).map(move |dx| Point { x: x_start + dx, y })
        })
    }
}
