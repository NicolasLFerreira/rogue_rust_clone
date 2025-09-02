use crate::geometry::delta::Delta;
use crate::geometry::point::Point;
use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

// Constructors
impl Rect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn new_dimensions(width: usize, height: usize) -> Self {
        Rect {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    pub fn empty() -> Self {
        Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}

// Queries
impl Rect {
    pub fn area(&self) -> usize {
        self.width * self.height
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x < self.x + self.width
            && point.y >= self.y
            && point.y < self.y + self.height
    }

    pub fn intersect(&self, other: Rect) -> bool {
        self.x < other.x + other.width
            && other.x < self.x + self.width
            && self.y < other.y + other.height
            && other.y < self.y + self.height
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

// Transformations
impl Rect {
    pub fn resize(self, dw: i32, dh: i32) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            width: Self::apply_delta_1d(self.width, dw),
            height: Self::apply_delta_1d(self.height, dh),
        }
    }

    pub fn translate(self, delta: Delta) -> Rect {
        Rect {
            x: Self::apply_delta_1d(self.x, delta.dx),
            y: Self::apply_delta_1d(self.y, delta.dy),
            width: self.width,
            height: self.height,
        }
    }
}

// Private functions
impl Rect {
    #[inline]
    fn apply_delta_1d(u: usize, d: i32) -> usize {
        if d.is_negative() {
            u.saturating_sub((-d) as usize)
        } else {
            u.saturating_add(d as usize)
        }
    }
}
