// X -> Column -> Width
// Y -> Row -> Height

use std::fmt::{Display, Formatter};

// The map structure covering the whole screen
pub struct Level {
    pub width: usize,
    pub height: usize,
    pub sections: u8,
    rooms: Vec<Room>,
    corridors: Vec<Corridor>,
}

impl Level {
    pub fn create_new(width: usize, height: usize, sections: u8) -> Level {
        let mut rooms: Vec<Room> = (0..sections * sections)
            .map(|i| {
                Room::create_new(
                    RoomId(i as usize),
                    width / sections as usize,
                    height / sections as usize,
                )
            })
            .collect();

        let corridor = Corridor {
            connections: rooms.iter().map(|a| a.id).collect(),
        };

        Level {
            width,
            height,
            sections,
            rooms,
            corridors: vec![corridor],
        }
    }
}

// Walkable map connecting rooms
pub struct Corridor {
    pub connections: Vec<RoomId>,
}

// Playable area
pub struct Room {
    pub id: RoomId,
    pub width: usize,  // X, Column
    pub height: usize, // Y, Row
    internal_map: Vec<Tile>,
}

impl Room {
    // Basic map generation
    pub fn create_new(id: RoomId, width: usize, height: usize) -> Self {
        Room {
            id,
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
        self.index(x, y).map(|i| &self.internal_map[i])
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

// Wrapper ID structs
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RoomId(pub usize);
