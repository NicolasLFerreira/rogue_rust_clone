mod map;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode, poll, read},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{Write, stdout};
use std::time::Duration;
use crate::map::Map;

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, Hide, Clear(ClearType::All))?;

    let map: Map = Map::create_new(10, 10);

    let (mut x, mut y) = (5, 5);

    // Draw initial grid
    for row in 0..map.height {
        for col in 0..map.width {
            queue!(stdout, MoveTo(col as u16, row as u16), Print("."))?;
        }
    }

    // Draw initial player
    queue!(stdout, MoveTo(map.height/2, map.width/2), Print("@"))?;
    stdout.flush()?;

    let mut num: i32 = 0;

    'game: loop {
        if poll(Duration::from_millis(100))? {

            if let Event::Key(event) = read()? {
                num+=1;
                queue!(stdout, MoveTo(0, 0), Print(format!("This is something : {num}")));
            }

            continue 'game;

            if let Event::Key(event) = read()? {
                // erase old player
                queue!(stdout, MoveTo(x, y), Print("."))?;

                // handle movement
                match event.code {
                    // Cardinal directions
                    KeyCode::Up | KeyCode::Char('8') => {
                        if y > 0 {
                            y -= 1
                        }
                    }
                    KeyCode::Down | KeyCode::Char('2') => {
                        if y < map.height - 1 {
                            y += 1
                        }
                    }
                    KeyCode::Left | KeyCode::Char('4') => {
                        if x > 0 {
                            x -= 1
                        }
                    }
                    KeyCode::Right | KeyCode::Char('6') => {
                        if x < map.width - 1 {
                            x += 1
                        }
                    }

                    // Diagonals via keypad (NumLock on)
                    KeyCode::Char('7') => {
                        if x > 0 {
                            x -= 1
                        };
                        if y > 0 {
                            y -= 1
                        }
                    } // up-left
                    KeyCode::Char('9') => {
                        if x < map.width - 1 {
                            x += 1
                        };
                        if y > 0 {
                            y -= 1
                        }
                    } // up-right
                    KeyCode::Char('1') => {
                        if x > 0 {
                            x -= 1
                        };
                        if y < map.height - 1 {
                            y += 1
                        }
                    } // down-left
                    KeyCode::Char('3') => {
                        if x < map.width - 1 {
                            x += 1
                        };
                        if y < map.height - 1 {
                            y += 1
                        }
                    } // down-right
                    KeyCode::Char('5') => { /* stay in place */ }

                    KeyCode::Char('q') => break 'game,
                    _ => {}
                }

                // draw new player
                queue!(stdout, MoveTo(x, y), Print("@"))?;
                stdout.flush()?;
            }
        }
    }

    // restore terminal
    execute!(stdout, Show)?;
    disable_raw_mode()?;
    Ok(())
}
