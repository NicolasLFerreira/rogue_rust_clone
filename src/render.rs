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
        execute!(stdout(), Hide).unwrap();
        Terminal
    }

    pub fn clear() {
        execute!(
            stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )
        .unwrap();
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        execute!(stdout(), Show, ResetColor).unwrap();
        Terminal::clear();
        disable_raw_mode().unwrap();
    }
}
