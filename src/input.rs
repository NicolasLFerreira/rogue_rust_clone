use crossterm::event::{Event, KeyCode, poll, read};
use std::time::Duration;

pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Quit,
    Wait,
}

pub fn map_key_to_action(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Up => Some(Action::MoveUp),
        KeyCode::Down => Some(Action::MoveDown),
        KeyCode::Left => Some(Action::MoveLeft),
        KeyCode::Right => Some(Action::MoveRight),
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
