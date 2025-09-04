use crate::geometry::point::Point;

pub enum EntityKind {
    Player,
    Enemy,
}

pub struct Entity {
    pub point: Point,
    pub kind: EntityKind,
}

impl Entity {
    pub fn new(point: Point, kind: EntityKind) -> Self {
        Self { point, kind }
    }
}
