use crate::input::input_handler::Action::Meta;
use crate::input::types::*;
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
        KeyCode::Esc => Some(Meta(MetaAction::Quit)),
        KeyCode::Char(' ') => Some(Meta(MetaAction::Wait)),

        // Move
        KeyCode::Up | KeyCode::Char('8') => Some(Action::Move(MoveAction::Up)),
        KeyCode::Down | KeyCode::Char('2') => Some(Action::Move(MoveAction::Down)),
        KeyCode::Left | KeyCode::Char('4') => Some(Action::Move(MoveAction::Left)),
        KeyCode::Right | KeyCode::Char('6') => Some(Action::Move(MoveAction::Right)),
        KeyCode::Char('7') => Some(Action::Move(MoveAction::UpLeft)),
        KeyCode::Char('9') => Some(Action::Move(MoveAction::UpRight)),
        KeyCode::Char('1') => Some(Action::Move(MoveAction::DownLeft)),
        KeyCode::Char('3') => Some(Action::Move(MoveAction::DownRight)),

        // Not-implemented
        _ => None,
    }
}
