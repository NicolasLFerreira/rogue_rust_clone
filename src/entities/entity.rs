use crate::geometry::point::Point;

pub enum EntityKind {
    Player,
    Enemy,
}

pub struct Stats {
    pub max_hp: i32,
    pub cur_hp: i32,
    pub atk: i32,
    pub def: i32,
}

impl Stats {
    pub const EMPTY: Self = Stats {
        max_hp: 0,
        cur_hp: 0,
        atk: 0,
        def: 0,
    };
}

pub struct Entity {
    pub point: Point,
    pub kind: EntityKind,
    pub stats: Stats,
}

// Constructor
impl Entity {
    pub const EMPTY: Self = Entity {
        point: Point::ZERO,
        stats: Stats::EMPTY,
        kind: EntityKind::Enemy,
    };

    pub fn new(point: Point, kind: EntityKind) -> Self {
        Self {
            point,
            kind,
            stats: Stats::EMPTY,
        }
    }
}
