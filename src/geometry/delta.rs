use crate::geometry::direction::Direction;
use crate::geometry::point::Point;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Delta {
    pub dx: i32,
    pub dy: i32,
}

// Constructor
impl Delta {
    pub const ZERO: Delta = Delta { dx: 0, dy: 0 };

    pub fn new(dx: i32, dy: i32) -> Self {
        Self { dx, dy }
    }
}

// Queries
impl Delta {
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.dx == 0 && self.dy == 0
    }

    #[inline]
    pub fn to_direction(&self) -> Direction {
        let normalized = self.normalize();
        match (normalized.dx, normalized.dy) {
            (0, -1) => Direction::North,
            (0, 1) => Direction::South,
            (-1, 0) => Direction::West,
            (1, 0) => Direction::East,
            (-1, -1) => Direction::NorthWest,
            (1, -1) => Direction::NorthEast,
            (-1, 1) => Direction::SouthWest,
            (1, 1) => Direction::SouthEast,
            (0, 0) | _ => Direction::Center,
        }
    }
}

// Transformation
impl Delta {
    #[inline]
    pub fn scale(self, factor: f64) -> Delta {
        Delta {
            dx: (self.dx as f64 * factor).round() as i32,
            dy: (self.dy as f64 * factor).round() as i32,
        }
    }

    #[inline]
    pub fn normalize(self) -> Delta {
        let dx = if self.dx == 0 {
            0
        } else {
            self.dx / self.dx.abs()
        };
        let dy = if self.dy == 0 {
            0
        } else {
            self.dy / self.dy.abs()
        };
        Delta { dx, dy }
    }
}

impl Add for Delta {
    type Output = Delta;
    fn add(self, rhs: Delta) -> Delta {
        Delta {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
        }
    }
}

impl Sub for Delta {
    type Output = Delta;
    fn sub(self, rhs: Delta) -> Delta {
        Delta {
            dx: self.dx - rhs.dx,
            dy: self.dy - rhs.dy,
        }
    }
}

impl Neg for Delta {
    type Output = Delta;
    fn neg(self) -> Delta {
        Delta {
            dx: -self.dx,
            dy: -self.dy,
        }
    }
}

impl Mul<i32> for Delta {
    type Output = Delta;
    fn mul(self, rhs: i32) -> Delta {
        Delta {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
        }
    }
}

impl From<Point> for Delta {
    fn from(value: Point) -> Self {
        Delta {
            dx: value.x as i32,
            dy: value.y as i32,
        }
    }
}
