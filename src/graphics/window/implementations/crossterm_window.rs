use crate::geometry::rect::Rect;
use crate::graphics::window::game_window::{GameWindow, KeyCode, WindowEvent};
use crossterm::event::{Event, KeyCode as CtKeyCode, KeyEventKind, poll, read};
use std::time::Duration;

// Empty implementation. Crossterm automatically utilises stdout which does not warrant manual window management.
// However, input is still abstracted behind the window, so here it is.
pub struct CrosstermGameWindow {
    window_size: Rect,
}

impl CrosstermGameWindow {
    pub fn new(window_size: Rect) -> Self {
        Self { window_size }
    }
}

impl GameWindow for CrosstermGameWindow {
    fn size(&self) -> Rect {
        self.window_size
    }

    fn poll_events(&self) -> Vec<WindowEvent> {
        let mut events = Vec::new();
        while poll(Duration::from_millis(0)).unwrap_or(false) {
            if let Ok(Event::Key(event)) = read() {
                if event.kind == KeyEventKind::Press {
                    events.push(WindowEvent::KeyPressed(Self::map_key_codes(event.code)))
                }
            }
        }
        events
    }

    fn is_open(&self) -> bool {
        true
    }
}

impl CrosstermGameWindow {
    fn map_key_codes(ct_key_code: CtKeyCode) -> KeyCode {
        match ct_key_code {
            CtKeyCode::Char(char) => KeyCode::Char(char),
            CtKeyCode::Up => KeyCode::Up,
            CtKeyCode::Down => KeyCode::Down,
            CtKeyCode::Left => KeyCode::Left,
            CtKeyCode::Right => KeyCode::Right,
            _ => KeyCode::Escape,
        }
    }
}
