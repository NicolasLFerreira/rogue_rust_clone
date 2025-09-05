use crate::geometry::point::Point;
use crate::types::Id;

pub enum EntityKind {
    Player,
    Enemy,
}

#[derive(Clone, Debug)]
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
    id: Id,
    pub point: Point,
    pub kind: EntityKind,
    pub stats: Stats,
}

// Constructor
impl Entity {
    pub fn new(point: Point, kind: EntityKind) -> Self {
        Self {
            id: 0,
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

    pub(crate) fn assign_id(&mut self, id: Id) {
        self.id = id;
    }
}
