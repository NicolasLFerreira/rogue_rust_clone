use crate::entities::entity::{Entity, EntityType};
use crate::game_map::generation::generators::dungeon_map_generator::DungeonMapGenerator;
use crate::game_map::generation::map_generator::MapGenerator;
use crate::game_map::tile_map::*;
use crate::geometry::direction::Direction;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;
use crate::graphics::graphics::Graphics;
use crate::graphics::rendering::frame::Frame;
use crate::graphics::theme::{AsciiTheme, Theme};
use crossterm::style::Color;

pub struct Game {
    pub map: TileMap,
    pub entities: Vec<Entity>,
    pub player_idx: usize,
    theme: Box<dyn Theme>,
}

impl Game {
    pub fn new(rect: Rect) -> Self {
        // Map
        let mut tile_map = TileMap::new(rect);

        // Generation
        let mut generator: Box<dyn MapGenerator> = Box::new(DungeonMapGenerator::new(rect));
        generator.generate_map(&mut tile_map);

        // Entities
        let player: Entity = Entity::new(tile_map.rnd_floor_point(), EntityType::Player);
        let entities = vec![player];
        // entities.push(Entity::new(Point::new(5, 5), EntityType::Enemy));

        Self {
            map: tile_map,
            entities,
            player_idx: 0,
            theme: Box::new(AsciiTheme {}),
        }
    }

    pub fn player(&self) -> &Entity {
        &self.entities[self.player_idx]
    }

    pub fn player_mut(&mut self) -> &mut Entity {
        &mut self.entities[self.player_idx]
    }

    pub(crate) fn move_player(&mut self, key: Direction) {
        let delta = key.to_delta();
        if let Some(new_point) = self.player().point.offset(delta) {
            if self.can_move_to_tile(new_point) {
                self.player_mut().point = new_point;
            }
        }
    }

    fn can_move_to_tile(&self, point: Point) -> bool {
        self.map
            .safe_get(point)
            .map(|tile| tile.is_walkable())
            .unwrap_or(false)
    }

    pub(crate) fn compose(&self, frame: &mut Frame) {
        // Map
        for (point, tile) in self.map.iter_tiles() {
            frame.put(point, self.theme.tile_theme(tile))
        }

        // Entities
        for entity in self.entities.iter() {
            if self.map.get(entity.point).visible {
                frame.put(entity.point, self.theme.entity_theme(entity));
            }
        }

        // UI
        frame.put_str(
            Point::new(0, self.map.rect.height),
            "q to quit",
            Color::Yellow,
            Color::Black,
        );
    }
}
