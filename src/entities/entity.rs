use crate::geometry::point::Point;

pub enum EntityType {
    Player,
    Enemy,
}

pub struct Entity {
    pub point: Point,
    pub entity_type: EntityType,
}

impl Entity {
    pub fn new(point: Point, entity_type: EntityType) -> Self {
        Self { point, entity_type }
    }
}
