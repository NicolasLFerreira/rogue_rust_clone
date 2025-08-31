use crate::dungeon::dungeon_map::DungeonMap;
use crate::entities::entity::{Entity, EntityType};
use crate::types::point::Point;

pub struct Game {
    pub player: Entity,
    pub entities: Vec<Entity>,
    pub dungeon_map: DungeonMap,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Entity::new(Point::new(5, 5), EntityType::Player),
            entities: Vec::new(),
            dungeon_map: DungeonMap::new(25, 25),
        }
    }
}
