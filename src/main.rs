mod dungeon;
mod input;
mod render;
mod utils;

use crate::dungeon::{map::*, tile::*, utils::*};
use crate::input::{Action, get_actions};
use crate::render::Terminal;
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

    let mut player_pos = Coord::new(5, 5);
    let map = DungeonMap::new(20, 20);

    'game_loop: loop {
        let frame_start = Instant::now();

        // --- INPUT ---
        let actions = get_actions();

        // --- UPDATE ---
        for action in actions {
            match action {
                Action::MoveUp => {
                    if player_pos.y > 0 {
                        player_pos.y -= 1
                    }
                }
                Action::MoveDown => {
                    if player_pos.y < map.height - 1 {
                        player_pos.y += 1
                    }
                }
                Action::MoveLeft => {
                    if player_pos.x > 0 {
                        player_pos.x -= 1
                    }
                }
                Action::MoveRight => {
                    if player_pos.x < map.width - 1 {
                        player_pos.x += 1
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

        queue!(
            stdout,
            MoveTo(player_pos.x as u16, player_pos.y as u16),
            SetForegroundColor(Color::Yellow),
            Print("@"),
            ResetColor
        )
        .unwrap();

        queue!(
            stdout,
            MoveTo(0, map.height as u16 + 2),
            SetForegroundColor(Color::Red),
            Print(format!("Player pos: {} {}", player_pos.y, player_pos.x)),
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
