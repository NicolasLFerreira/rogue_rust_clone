use crate::geometry::delta::Delta;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

// Constructor
impl Point {
    pub const ZERO: Point = Point { x: 0, y: 0 };

    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

// Queries
impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn manhattan_distance(&self, other: &Point) -> usize {
        (self.x as isize - other.x as isize).abs() as usize
            + (self.y as isize - other.y as isize).abs() as usize
    }

    pub fn neighbors(&self) -> [Point; 4] {
        [
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x.saturating_sub(1),
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y.saturating_sub(1),
            },
        ]
    }
}

// Transformations
impl Point {
    pub fn offset(self, d: Delta) -> Option<Point> {
        let nx = self.x as i32 + d.dx;
        let ny = self.y as i32 + d.dy;

        if nx < 0 || ny < 0 {
            None
        } else {
            Some(Point {
                x: nx as usize,
                y: ny as usize,
            })
        }
    }

    pub fn saturating_offset(self, d: Delta) -> Point {
        let nx = self.x as i32 + d.dx;
        let ny = self.y as i32 + d.dy;

        Point {
            x: nx.max(0) as usize,
            y: ny.max(0) as usize,
        }
    }
}

impl Add<Delta> for Point {
    type Output = Point;

    fn add(self, rhs: Delta) -> Self::Output {
        Point {
            x: (self.x as i32 + rhs.dx).max(0) as usize,
            y: (self.y as i32 + rhs.dy).max(0) as usize,
        }
    }
}

impl Add<(usize, usize)> for Point {
    type Output = Point;

    fn add(self, rhs: (usize, usize)) -> Self::Output {
        Point {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl Sub for Point {
    type Output = Delta;

    fn sub(self, rhs: Self) -> Self::Output {
        Delta {
            dx: (self.x as i32) - (rhs.x as i32),
            dy: (self.y as i32) - (rhs.y as i32),
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from(t: (usize, usize)) -> Point {
        Point { x: t.0, y: t.1 }
    }
}

impl From<Point> for (usize, usize) {
    fn from(p: Point) -> (usize, usize) {
        (p.x, p.y)
    }
}
