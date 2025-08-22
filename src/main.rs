mod input;
mod map;
mod render;
mod utils;

use crate::input::{Action, get_actions};
use crate::map::Map;
use crate::render::Terminal;
use crate::utils::Coord;
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

    let mut player_pos = Coord::new(5, 5);
    let map = Map::create_new(10, 10);

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
                    if player_pos.y < map.height as u16 - 1 {
                        player_pos.y += 1
                    }
                }
                Action::MoveLeft => {
                    if player_pos.x > 0 {
                        player_pos.x -= 1
                    }
                }
                Action::MoveRight => {
                    if player_pos.x < map.width as u16 - 1 {
                        player_pos.x += 1
                    }
                }
                Action::Quit => break 'game_loop,
                Action::Wait => {}
            }
        }

        // --- RENDER ---
        Terminal::clear();

        for (y, row) in map.matrix.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                queue!(stdout, MoveTo(x as u16, y as u16), Print(tile)).unwrap();
            }
        }

        queue!(
            stdout,
            MoveTo(player_pos.x, player_pos.y),
            SetForegroundColor(Color::Yellow),
            Print("@"),
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
