mod input;
mod map;
mod render;
mod utils;

use crate::input::Action;
use crate::map::Map;
use crate::utils::Coord;
use crossterm::event::poll;
use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, read},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{Write, stdout};
use std::time::Duration;

fn main() {
    let _terminal: render::Terminal = render::Terminal::new();

    let mut stdout = stdout();
    let mut player_pos: Coord = Coord::new(5, 5);

    let map: Map = Map::create_new(10, 10);

    'main:loop {
        // Clear screen
        render::Terminal::clear();

        // Draw map
        for (y, row) in map.matrix.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                queue!(stdout, MoveTo(x as u16, y as u16), Print(tile)).unwrap();
            }
        }

        // Draw player
        queue!(
            stdout,
            MoveTo(player_pos.x, player_pos.y),
            SetForegroundColor(Color::Yellow),
            Print("@"),
            ResetColor
        )
        .unwrap();

        stdout.flush().unwrap();

        let actions = input::get_actions();

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
                Action::Quit => {
                    break 'main;
                }
                Action::Wait => {
                    continue;
                }
            }
        }
    }
}
