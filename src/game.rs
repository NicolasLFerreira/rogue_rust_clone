use crate::dungeon::dungeon_map::DungeonMap;
use crate::entities::entity::{Entity, EntityType};
use crate::types::point::Point;

pub struct Game {
    pub entities: Vec<Entity>,
    pub player_idx: usize,
    pub dungeon_map: DungeonMap,
}

impl Game {
    pub fn new() -> Self {
        let player: Entity = Entity::new(Point::new(0, 0), EntityType::Player);
        let entities = vec![player];

        Self {
            entities,
            player_idx: 0,
            dungeon_map: DungeonMap::new(25, 25),
        }
    }

    pub fn player(&self) -> &mut Entity {
        &mut self.entities[self.player_idx]
    }
}
