use crate::geometry::point::Point;

pub enum EntityKind {
    Player,
    Enemy,
}

#[derive(Clone)]
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
    id: usize,
    pub point: Point,
    pub kind: EntityKind,
    pub stats: Stats,
}

// Constructor
impl Entity {
    pub fn new(id: usize, point: Point, kind: EntityKind) -> Self {
        Self {
            id,
            point,
            kind,
            stats: Stats {
                max_hp: 20,
                cur_hp: 20,
                atk: 5,
                def: 5,
            },
        }
    }
}

// Queries
impl Entity {
    // Behind a getter since changing the id is a big no-no
    pub fn id(&self) -> usize {
        self.id
    }
}
