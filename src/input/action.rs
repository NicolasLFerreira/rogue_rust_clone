use crate::geometry::direction::Direction;

pub enum Action {
    Meta(MetaAction),
    Move(Direction),
}

pub enum MetaAction {
    Quit,
    Restart,
    Wait,
}
