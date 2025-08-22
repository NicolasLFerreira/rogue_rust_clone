mod map;
mod render;
mod utils;

use crossterm::event::poll;
use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, read},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{Write, stdout};
use std::time::Duration;
use map::Map;

fn main() {
    let _terminal = render::Terminal::new();

    let mut stdout = stdout();
    let mut player_pos = (5, 5);

    let map: Map = Map::create_new(10, 10);

    loop {
        // Clear screen
        execute!(stdout, Clear(ClearType::All)).unwrap();

        // Draw map
        for (y, row) in map.matrix.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                execute!(stdout, MoveTo(x as u16, y as u16), Print(tile)).unwrap();
            }
        }

        // Draw player
        execute!(
            stdout,
            MoveTo(player_pos.0, player_pos.1),
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
                            if player_pos.1 > 0 {
                                player_pos.1 -= 1
                            }
                        }
                        KeyCode::Down => {
                            if player_pos.1 < map.height as u16 - 1 {
                                player_pos.1 += 1
                            }
                        }
                        KeyCode::Left => {
                            if player_pos.0 > 0 {
                                player_pos.0 -= 1
                            }
                        }
                        KeyCode::Right => {
                            if player_pos.0 < map.width as u16 - 1 {
                                player_pos.0 += 1
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
