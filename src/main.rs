mod dungeon;
mod entities;
mod game;
mod input;
mod rendering;
mod types;

use crate::dungeon::tile::*;
use crate::entities::entity::{Entity, EntityType};
use crate::game::Game;
use crate::input::{Action, get_actions};
use crate::rendering::render::*;
use crate::types::point::*;
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::{Write, stdout};
use std::thread::sleep;
use std::time::{Duration, Instant};

const FPS: u64 = 10;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / FPS);

fn main() {
    let _terminal: Terminal = Terminal::new();
    let mut stdout = stdout();

    let mut game: Game = Game::new();

    game.entities
        .push(Entity::new(Point::new(2, 2), EntityType::Enemy));
    game.entities
        .push(Entity::new(Point::new(7, 3), EntityType::Enemy));
    game.entities
        .push(Entity::new(Point::new(13, 8), EntityType::Enemy));

    'game_loop: loop {
        let frame_start = Instant::now();

        // --- INPUT ---
        let actions = get_actions();

        // --- UPDATE ---
        for action in actions {
            match action {
                Action::MoveUp => {
                    if game.player.point.y > 0 {
                        game.player.point.y -= 1
                    }
                }
                Action::MoveDown => {
                    if game.player.point.y < (game.dungeon_map.height - 1) as i32 {
                        game.player.point.y += 1
                    }
                }
                Action::MoveLeft => {
                    if game.player.point.x > 0 {
                        game.player.point.x -= 1
                    }
                }
                Action::MoveRight => {
                    if game.player.point.x < (game.dungeon_map.width - 1) as i32 {
                        game.player.point.x += 1
                    }
                }
                Action::Quit => break 'game_loop,
                Action::Wait => {}
            }
        }

        // --- RENDER ---
        Terminal::clear();

        for x in 0..game.dungeon_map.width {
            for y in 0..game.dungeon_map.height {
                queue!(
                    stdout,
                    MoveTo(x as u16, y as u16),
                    Print(match game.dungeon_map.get(y, x).unwrap().tile_type {
                        TileType::Floor => '.',
                        TileType::Wall => '#',
                    })
                )
                .unwrap();
            }
        }

        for entity in std::iter::once(&game.player).chain(game.entities.iter()) {
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
            MoveTo(0, game.dungeon_map.height as u16 + 2),
            SetForegroundColor(Color::Green),
            Print(format!(
                "Player pos: {} {}",
                game.player.point.y, game.player.point.x
            )),
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
