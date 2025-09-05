pub trait GameWindow {
    fn size(&self) -> (usize, usize);
    fn poll_events(&self) -> Vec<WindowEvent>;
    fn is_open(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent {
    CloseRequested,
    Resized(u32, u32),

    KeyPressed(KeyCode),
    KeyReleased(KeyCode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyCode {
    Up,
    Down,
    Left,
    Right,

    Escape,
    BackSpace,
    Enter,
    Char(char),
}
