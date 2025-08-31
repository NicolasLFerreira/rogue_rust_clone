use crossterm::event::{Event, KeyCode, poll, read};
use std::time::Duration;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

pub enum Action {
    // Directions
    Move(Direction),

    // Meta-actions
    Quit,
    Wait,
}

pub fn map_key_to_action(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Up | KeyCode::Char('8') => Some(Action::Move(Direction::Up)),
        KeyCode::Down | KeyCode::Char('2') => Some(Action::Move(Direction::Down)),
        KeyCode::Left | KeyCode::Char('4') => Some(Action::Move(Direction::Left)),
        KeyCode::Right | KeyCode::Char('6') => Some(Action::Move(Direction::Right)),
        KeyCode::Char('7') => Some(Action::Move(Direction::UpLeft)),
        KeyCode::Char('9') => Some(Action::Move(Direction::UpRight)),
        KeyCode::Char('1') => Some(Action::Move(Direction::DownLeft)),
        KeyCode::Char('3') => Some(Action::Move(Direction::DownRight)),
        KeyCode::Esc => Some(Action::Quit),
        KeyCode::Char(' ') => Some(Action::Wait),
        _ => None,
    }
}

pub fn get_actions() -> Vec<Action> {
    let mut actions = vec![];
    while poll(Duration::from_millis(0)).unwrap() {
        if let Event::Key(event) = read().unwrap() {
            if event.kind.is_press() {
                if let Some(action) = map_key_to_action(event.code) {
                    actions.push(action);
                }
            }
        }
    }
    actions
}
