use crate::geometry::direction::Direction;
use crate::graphics::window::game_window::{KeyCode, WindowEvent};

pub enum Action {
    Meta(MetaAction),
    Move(Direction),
}

pub enum MetaAction {
    Quit,
    Restart,
    Wait,
}

pub fn input_action_mapper(event: WindowEvent) -> Action {
    if let Some(action) = map_key_to_action(event) {
        action
    } else {
        Action::Meta(MetaAction::Wait)
    }
}

pub fn map_key_to_action(event: WindowEvent) -> Option<Action> {
    match event {
        WindowEvent::KeyPressed(key) => match key {
            // Meta
            KeyCode::Char('q') => Some(Action::Meta(MetaAction::Quit)),
            KeyCode::Char('r') => Some(Action::Meta(MetaAction::Restart)),
            KeyCode::Char(' ') => Some(Action::Meta(MetaAction::Wait)),

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
