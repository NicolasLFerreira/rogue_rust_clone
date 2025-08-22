mod map;
mod render;
mod utils;

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

    loop {
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

        // Poll input with timeout to avoid blocking
        if poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(event) = read().unwrap() {
                if event.kind.is_press() {
                    match event.code {
                        KeyCode::Up => {
                            if player_pos.y > 0 {
                                player_pos.y -= 1
                            }
                        }
                        KeyCode::Down => {
                            if player_pos.y < map.height as u16 - 1 {
                                player_pos.y += 1
                            }
                        }
                        KeyCode::Left => {
                            if player_pos.x > 0 {
                                player_pos.x -= 1
                            }
                        }
                        KeyCode::Right => {
                            if player_pos.x < map.width as u16 - 1 {
                                player_pos.x += 1
                            }
                        }
                        KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
        }
    }
}
