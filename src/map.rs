// X -> Column -> Width
// Y -> Row -> Height

use std::fmt::{Display, Formatter};

pub struct TileMap {
    pub width: usize,  // X, Column
    pub height: usize, // Y, Row
    internal_map: Vec<Tile>,
}

impl TileMap {
    // Basic map generation
    pub fn create_new(width: usize, height: usize) -> Self {
        TileMap {
            width,
            height,
            internal_map: vec![Tile { icon: '.' }; width * height],
        }
    }

    fn in_bound(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    // row major
    fn index(&self, x: usize, y: usize) -> Option<usize> {
        self.in_bound(x, y).then(|| y * self.width + x)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        match self.index(x, y) {
            Some(index) => Some(&self.internal_map[index]),
            None => None,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, tile: Tile) -> bool {
        match self.index(x, y) {
            Some(index) => {
                self.internal_map[index] = tile;
                true
            }
            None => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub icon: char,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.icon)
    }
}

pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }
}
