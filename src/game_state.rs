use crate::entities::entity::{Entity, EntityKind};
use crate::entities::entity_manager::EntityManager;
use crate::game_map::generation::generators::dungeon_map_generator::DungeonMapGenerator;
use crate::game_map::generation::map_generator::MapGenerator;
use crate::game_map::tile_map::*;
use crate::geometry::delta::Delta;
use crate::geometry::rect::Rect;
use crate::systems::movement::MovementSystem;

pub struct GameState {
    pub tile_map: TileMap,
    pub entity_manager: EntityManager,
}

impl GameState {
    pub fn new(rect: Rect) -> Self {
        // Map
        let mut tile_map = TileMap::new(rect);

        // Generation
        let mut generator: Box<dyn MapGenerator> = Box::new(DungeonMapGenerator::new(rect));
        generator.generate_map(&mut tile_map);

        // Entities
        let player = Entity::new(tile_map.rnd_floor_point(), EntityKind::Player);
        let mut entity_manager = EntityManager::new(player);
        entity_manager.spawn(Entity::new(tile_map.rnd_floor_point(), EntityKind::Enemy));

        Self {
            tile_map,
            entity_manager,
        }
    }

    pub fn move_entities(&mut self) {
        let player_id = self.entity_manager.player_id();
        let entity_ids: Vec<_> = self
            .entity_manager
            .iter()
            .map(|e| e.id())
            .filter(|&id| id != player_id)
            .collect();

        for id in entity_ids {
            MovementSystem::try_move(
                self,
                id,
                Delta::new(rand::random_range(-1..=1), rand::random_range(-1..=1)).to_direction(),
            );
        }
    }
}
