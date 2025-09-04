use crate::entities::entity::{Entity, EntityKind};
use crate::entities::entity_manager::EntityManager;
use crate::game_map::generation::generators::dungeon_map_generator::DungeonMapGenerator;
use crate::game_map::generation::map_generator::MapGenerator;
use crate::game_map::tile_map::*;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;

pub struct Game {
    pub tile_map: TileMap,
    pub entity_manager: EntityManager,
}

impl Game {
    pub fn new(rect: Rect) -> Self {
        // Map
        let mut tile_map = TileMap::new(rect);

        // Generation
        let mut generator: Box<dyn MapGenerator> = Box::new(DungeonMapGenerator::new(rect));
        generator.generate_map(&mut tile_map);

        // Entities
        let player = Entity::new(tile_map.rnd_floor_point(), EntityKind::Player);
        let mut entity_manager = EntityManager::new(player);
        entity_manager.add(Entity::new(Point::new(5, 5), EntityKind::Enemy));

        Self {
            tile_map,
            entity_manager,
        }
    }
}
