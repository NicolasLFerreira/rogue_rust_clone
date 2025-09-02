use crate::geometry::delta::Delta;

pub enum Direction {
    Center,
    North,
    South,
    West,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Direction {
    pub const ALL: [Direction; 9] = [
        Direction::Center,
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
        Direction::NorthWest,
        Direction::NorthEast,
        Direction::SouthWest,
        Direction::SouthEast,
    ];

    pub fn to_delta(self) -> Delta {
        match self {
            Direction::Center => Delta::new(0, 0),
            Direction::North => Delta::new(0, -1),
            Direction::South => Delta::new(0, 1),
            Direction::West => Delta::new(-1, 0),
            Direction::East => Delta::new(1, 0),
            Direction::NorthWest => Delta::new(-1, -1),
            Direction::NorthEast => Delta::new(1, -1),
            Direction::SouthWest => Delta::new(-1, 1),
            Direction::SouthEast => Delta::new(1, 1),
        }
    }
}
