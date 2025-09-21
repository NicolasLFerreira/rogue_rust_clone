use crate::entities::entity::{Entity, EntityKind};
use crate::entities::entity_manager::EntityManager;
use crate::game_map::generation::generators::dungeon_map_generator::DungeonMapGenerator;
use crate::game_map::generation::map_generator::MapGenerator;
use crate::game_map::tile_map::*;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;
use crate::systems::movement::{MoveEvent, MovementSystem};

pub struct State {
    pub tile_map: TileMap,
    pub entity_manager: EntityManager,
}

impl State {
    pub fn new(rect: Rect) -> Self {
        // Map generation
        let mut tile_map = TileMap::new(rect);
        let mut generator: Box<dyn MapGenerator> = Box::new(DungeonMapGenerator::new(rect));
        generator.generate_map(&mut tile_map);

        // Entities
        let player = Entity::new(tile_map.rnd_floor_point(), EntityKind::Player);
        let mut entity_manager = EntityManager::new(player);

        for _ in 0..8 {
            let pos: Point = loop {
                let pos = tile_map.rnd_floor_point();
                if entity_manager.iter().all(|e: &Entity| e.point != pos) {
                    break pos;
                }
            };
            entity_manager
                .spawn(Entity::new(pos, EntityKind::Enemy));
        }

        Self {
            tile_map,
            entity_manager,
        }
    }

    pub fn move_entities(&mut self) -> Vec<MoveEvent> {
        let player_id = self.entity_manager.player_id();

        let player_pos = if let Some(player) = self.entity_manager.player() {
            player.point
        } else {
            return vec![MoveEvent::Invalid];
        };

        let entity_ids: Vec<_> = self
            .entity_manager
            .iter()
            .map(|e| e.id())
            .filter(|&id| id != player_id)
            .collect();

        let mut results: Vec<MoveEvent> = vec![];
        for id in entity_ids {
            results.push(MovementSystem::try_move_npc(
                &mut self.entity_manager,
                &self.tile_map,
                id,
                player_pos,
            ));
        }

        results
    }
}
