use crate::geometry::direction::Direction;
use crate::input::action::*;
use crate::input::input_handler::Action::Meta;
use crossterm::event::{Event, KeyCode, KeyEventKind, poll, read};
use std::time::Duration;

pub fn get_input() -> Vec<Action> {
    let mut actions = Vec::new();

    while poll(Duration::from_millis(0)).unwrap_or(false) {
        if let Ok(Event::Key(event)) = read() {
            if event.kind == KeyEventKind::Press {
                if let Some(action) = map_key_to_action(event.code) {
                    actions.push(action);
                }
            }
        }
    }

    actions
}

fn map_key_to_action(key: KeyCode) -> Option<Action> {
    match key {
        // Meta
        KeyCode::Char('q') => Some(Meta(MetaAction::Quit)),
        KeyCode::Char('r') => Some(Meta(MetaAction::Restart)),
        KeyCode::Char(' ') => Some(Meta(MetaAction::Wait)),

        // Move
        KeyCode::Up | KeyCode::Char('8') => Some(Action::Move(Direction::North)),
        KeyCode::Down | KeyCode::Char('2') => Some(Action::Move(Direction::South)),
        KeyCode::Left | KeyCode::Char('4') => Some(Action::Move(Direction::West)),
        KeyCode::Right | KeyCode::Char('6') => Some(Action::Move(Direction::East)),
        KeyCode::Char('7') => Some(Action::Move(Direction::NorthWest)),
        KeyCode::Char('9') => Some(Action::Move(Direction::NorthEast)),
        KeyCode::Char('1') => Some(Action::Move(Direction::SouthWest)),
        KeyCode::Char('3') => Some(Action::Move(Direction::SouthEast)),

        // Not-implemented
        _ => None,
    }
}
