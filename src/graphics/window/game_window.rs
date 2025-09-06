use crate::geometry::rect::Rect;

pub trait GameWindow {
    fn size(&self) -> Rect;
    fn poll_events(&mut self) -> Vec<WindowEvent>;
    fn is_open(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent {
    KeyPressed(KeyCode),
    KeyReleased(KeyCode),
    Resized(u32, u32),
    CloseRequested,
    Redraw,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyCode {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,

    Space,
    Escape,
    BackSpace,
    Enter,
    Char(char),

    Unknown,
}
