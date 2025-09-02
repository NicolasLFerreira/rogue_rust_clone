use crate::dungeon::dungeon_map::*;
use crate::dungeon::dungeon_map_generator::DungeonMapGenerator;
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
        // Map
        let mut dungeon_map = DungeonMap::new(rect);
        let generator = DungeonMapGenerator::new(rect);
        generator.generate_map(&mut dungeon_map);

        // Entities
        let player: Entity = Entity::new(dungeon_map.rnd_floor_point(), EntityType::Player);
        let mut entities = vec![player];
        // entities.push(Entity::new(Point::new(5, 5), EntityType::Enemy));

        Self {
            dungeon_map,
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
        if let Some(new_point) = self.player().point.offset(delta) {
            if self.open_tile(new_point) {
                self.player_mut().point = new_point;
            }
        }
    }

    fn open_tile(&self, point: Point) -> bool {
        self.dungeon_map
            .get(point)
            .map(|tile| tile.tile_type == TileType::Floor)
            .unwrap_or(false)
    }

    pub(crate) fn compose(&self, frame: &mut Frame) {
        // Map
        for (point, tile) in self.dungeon_map.iter_tiles() {
            let tile_type = tile.tile_type;

            // Renders only what's visible
            if tile.visible {
                frame.put(
                    point,
                    Cell {
                        glyph: match tile_type {
                            TileType::Floor => '.',
                            TileType::Wall => '#',
                            TileType::Void => ' ',
                        },
                        foreground: match tile_type {
                            TileType::Floor => Color::White,
                            TileType::Wall => Color::White,
                            TileType::Void => Color::Black,
                        },
                        background: Color::Black,
                    },
                )
            }
            // Undiscovered tiles
            else {
                frame.put(
                    point,
                    Cell {
                        glyph: ' ',
                        foreground: Color::Black,
                        background: Color::Black,
                    },
                )
            }
        }

        // Entities
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

        // UI
        frame.put_str(
            Point::new(0, self.dungeon_map.rect.height),
            "q to quit",
            Color::Yellow,
            Color::Black,
        );
    }
}
