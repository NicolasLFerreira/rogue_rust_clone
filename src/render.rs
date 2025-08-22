use crossterm::cursor::MoveTo;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen};
use crossterm::{
    cursor::{Hide, Show},
    execute,
    style::ResetColor,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::stdout;

pub struct Terminal;

impl Terminal {
    pub fn new() -> Self {
        enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen, Hide).unwrap();
        Terminal
    }

    pub fn clear() {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        execute!(stdout(), Show, ResetColor).unwrap();
        Terminal::clear();
        disable_raw_mode().unwrap();
    }
}
