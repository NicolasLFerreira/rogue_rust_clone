pub enum Action {
    Move(MoveAction),
    Meta(MetaAction),
}

pub enum MetaAction {
    Quit,
    Wait,
}

pub enum MoveAction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}