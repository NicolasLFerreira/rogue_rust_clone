use crate::dungeon::dungeon_map::*;
use crate::dungeon::tile::*;
use crate::entities::entity::{Entity, EntityType};
use crate::geometry::direction::Direction;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;
use crate::rendering::cell::Cell;
use crate::rendering::frame::Frame;
use crossterm::style::Color;

pub struct Game {
    pub dungeon_map: DungeonMap,
    pub entities: Vec<Entity>,
    pub player_idx: usize,
}

impl Game {
    pub fn new(rect: Rect) -> Self {
        let player: Entity = Entity::new(Point::new(0, 0), EntityType::Player);
        let entities = vec![player, Entity::new(Point::new(5, 5), EntityType::Enemy)];
        Self {
            dungeon_map: DungeonMap::new(rect),
            entities,
            player_idx: 0,
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
        let rect = self.dungeon_map.rect;
        let player = self.player_mut();

        if let Some(new_point) = player.point.offset(delta) {
            if rect.contains(new_point) {
                player.point = new_point;
            }
        }
    }

    pub(crate) fn compose(&self, frame: &mut Frame) {
        // static background (example)

        for (point, tile) in self.dungeon_map.iter_tiles() {
            let tile_type = tile.tile_type;
            frame.put(
                point,
                Cell {
                    glyph: match tile_type {
                        TileType::Floor => '.',
                        TileType::Wall => '#',
                    },
                    foreground: match tile_type {
                        TileType::Floor => Color::White,
                        TileType::Wall => Color::DarkGrey,
                    },
                    background: Color::Black,
                },
            )
        }

        // entities (z-order: map < entities < ui)
        for entity in self.entities.iter() {
            frame.put(
                entity.point,
                Cell {
                    glyph: match entity.entity_type {
                        EntityType::Player => '@',
                        EntityType::Enemy => '&',
                    },
                    foreground: match entity.entity_type {
                        EntityType::Player => Color::Yellow,
                        EntityType::Enemy => Color::DarkRed,
                    },
                    background: Color::Black,
                },
            );
        }

        // UI overlay
        frame.put_str(
            Point::new(0, self.dungeon_map.rect.height),
            "q to quit",
            Color::Yellow,
            Color::Black,
        );
    }
}
