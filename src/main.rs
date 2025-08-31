mod dungeon;
mod entities;
mod input;
mod render;
mod types;

use crate::dungeon::{map::*, tile::*};
use crate::entities::entity::{Entity, EntityType};
use crate::input::{Action, get_actions};
use crate::render::Terminal;
use crate::types::point::*;
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::fmt::Formatter;
use std::io::{Write, stdout};
use std::thread::sleep;
use std::time::{Duration, Instant};

const FPS: u64 = 10;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / FPS);

fn main() {
    let _terminal: Terminal = Terminal::new();
    let mut stdout = stdout();

    // Player
    let mut player: Entity = Entity::new(Point::new(5, 5), EntityType::Player);

    // Entities
    let mut entities: Vec<Entity> = Vec::new();

    entities.push(Entity::new(Point::new(2, 2), EntityType::Enemy));
    entities.push(Entity::new(Point::new(7, 3), EntityType::Enemy));
    entities.push(Entity::new(Point::new(13, 8), EntityType::Enemy));

    // Map
    let map = DungeonMap::new(20, 20);

    'game_loop: loop {
        let frame_start = Instant::now();

        // --- INPUT ---
        let actions = get_actions();

        // --- UPDATE ---
        for action in actions {
            match action {
                Action::MoveUp => {
                    if player.point.y > 0 {
                        player.point.y -= 1
                    }
                }
                Action::MoveDown => {
                    if player.point.y < (map.height - 1) as i32 {
                        player.point.y += 1
                    }
                }
                Action::MoveLeft => {
                    if player.point.x > 0 {
                        player.point.x -= 1
                    }
                }
                Action::MoveRight => {
                    if player.point.x < (map.width - 1) as i32 {
                        player.point.x += 1
                    }
                }
                Action::Quit => break 'game_loop,
                Action::Wait => {}
            }
        }

        // --- RENDER ---
        Terminal::clear();

        for x in 0..map.width {
            for y in 0..map.height {
                queue!(
                    stdout,
                    MoveTo(x as u16, y as u16),
                    Print(match map.get(y, x).unwrap().tile_type {
                        TileType::Floor => '.',
                        TileType::Wall => '#',
                    })
                )
                .unwrap();
            }
        }

        for entity in std::iter::once(&player).chain(entities.iter()) {
            queue!(
                stdout,
                MoveTo(entity.point.x as u16, entity.point.y as u16),
                SetForegroundColor(match entity.entity_type {
                    EntityType::Player => Color::Yellow,
                    EntityType::Enemy => Color::Red,
                }),
                Print(match entity.entity_type {
                    EntityType::Player => '@',
                    EntityType::Enemy => '&',
                }),
                ResetColor
            )
            .unwrap();
        }

        queue!(
            stdout,
            MoveTo(0, map.height as u16 + 2),
            SetForegroundColor(Color::Green),
            Print(format!("Player pos: {} {}", player.point.y, player.point.x)),
            ResetColor
        )
        .unwrap();

        stdout.flush().unwrap();

        // --- FRAME LIMIT ---
        let elapsed = frame_start.elapsed();
        if elapsed < FRAME_DURATION {
            sleep(FRAME_DURATION - elapsed);
        }
    }
}
