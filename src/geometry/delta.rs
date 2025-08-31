use crate::geometry::direction::Direction;
use crate::geometry::point::Point;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Delta {
    pub dx: i32,
    pub dy: i32,
}

impl Delta {
    pub fn new(dx: i32, dy: i32) -> Self {
        Self { dx, dy }
    }

    pub fn is_zero(&self) -> bool {
        self.dx == 0 && self.dy == 0
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

impl From<Delta> for Point {
    fn from(value: Delta) -> Self {
        Point {
            x: value.dx as usize,
            y: value.dy as usize,
        }
    }
}
