use crate::input::action::*;

pub trait InputHandler {
    fn get_input() -> Vec<Action>;
}
