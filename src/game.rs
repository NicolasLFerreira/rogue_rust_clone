use crate::dungeon::dungeon_map::DungeonMap;
use crate::entities::entity::{Entity, EntityType};
use crate::input::action::*;
use crate::rendering::cell::Cell;
use crate::rendering::frame::Frame;
use crate::types::point::Point;
use crossterm::style::Color;

pub struct Game {
    pub dungeon_map: DungeonMap,
    pub entities: Vec<Entity>,
    pub player_idx: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let player: Entity = Entity::new(Point::new(0, 0), EntityType::Player);
        let entities = vec![player, Entity::new(Point::new(5, 5), EntityType::Enemy)];
        Self {
            dungeon_map: DungeonMap::new(width, height),
            entities,
            player_idx: 0,
        }
    }

    pub fn player(&mut self) -> &mut Entity {
        &mut self.entities[self.player_idx]
    }

    pub(crate) fn move_player(&mut self, key: MoveAction) {
        match key {
            MoveAction::Up if self.player().point.y > 0 => self.player().point.y -= 1,
            MoveAction::Down if self.player().point.y < (self.dungeon_map.height - 1) as i32 => {
                self.player().point.y += 1
            }
            MoveAction::Left if self.player().point.x > 0 => self.player().point.x -= 1,
            MoveAction::Right if self.player().point.x < (self.dungeon_map.width - 1) as i32 => {
                self.player().point.x += 1
            }
            MoveAction::UpLeft if self.player().point.x > 0 && self.player().point.y > 0 => {
                self.player().point.x -= 1;
                self.player().point.y -= 1;
            }
            MoveAction::UpRight
                if self.player().point.x < (self.dungeon_map.width - 1) as i32
                    && self.player().point.y > 0 =>
            {
                self.player().point.x += 1;
                self.player().point.y -= 1;
            }
            MoveAction::DownLeft
                if self.player().point.x > 0
                    && self.player().point.y < (self.dungeon_map.height - 1) as i32 =>
            {
                self.player().point.x -= 1;
                self.player().point.y += 1;
            }
            MoveAction::DownRight
                if self.player().point.x < (self.dungeon_map.width - 1) as i32
                    && self.player().point.y < (self.dungeon_map.height - 1) as i32 =>
            {
                self.player().point.x += 1;
                self.player().point.y += 1;
            }
            _ => {}
        }
    }

    pub(crate) fn compose(&self, frame: &mut Frame) {
        // static background (example)
        frame.clear(Cell {
            glyph: '.',
            foreground: Color::Grey,
            background: Color::Black,
        });

        // entities (z-order: map < entities < ui)
        for (i, entity) in self.entities.iter().enumerate() {
            frame.put(
                entity.point.x as usize,
                entity.point.y as usize,
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
            0,
            self.dungeon_map.height.saturating_sub(1),
            "q to quit",
            Color::Yellow,
            Color::Black,
        );
    }
}
