use crate::geometry::direction::Direction;
use crate::graphics::window::input_action_mapper::Action::Meta;
use crate::graphics::window::game_window::{KeyCode, WindowEvent};

pub fn input_action_mapper(events: Vec<WindowEvent>) -> Vec<Action> {
    let mut actions = vec![];
    for event in events {
        if let Some(action) = map_key_to_action(event) {
            actions.push(action);
        }
    }
    actions
}

fn map_key_to_action(event: WindowEvent) -> Option<Action> {
    match event {
        WindowEvent::KeyPressed(key) => match key {
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
        },
        _ => None,
    }
}

pub enum Action {
    Meta(MetaAction),
    Move(Direction),
}

pub enum MetaAction {
    Quit,
    Restart,
    Wait,
}
