use crate::geometry::rect::Rect;
use crate::graphics::window::game_window::{GameWindow, KeyCode, WindowEvent};
use std::collections::VecDeque;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent as WWindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode as WKeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct WinitGameWindow {
    screen_size: Rect,
    window: Option<Window>,
    events: VecDeque<WindowEvent>,
    open: bool,
}

impl GameWindow for WinitGameWindow {
    fn size(&self) -> Rect {
        self.screen_size
    }

    fn poll_events(&mut self) -> Vec<WindowEvent> {
        self.events.drain(..).collect()
    }

    fn is_open(&self) -> bool {
        self.open
    }
}

impl ApplicationHandler for WinitGameWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        self.open = true
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WWindowEvent) {
        match event {
            WWindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WWindowEvent::KeyboardInput { event, .. } => {
                self.events
                    .push_back(WindowEvent::KeyPressed(Self::map_winit_key(
                        event.physical_key,
                    )));
            }
            _ => {}
        }
    }
}

// Integration
impl WinitGameWindow {
    fn map_winit_key(physical_key: PhysicalKey) -> KeyCode {
        match physical_key {
            PhysicalKey::Code(code) => {
                match code {
                    // WKeyCode::KeyA => {}
                    // WKeyCode::KeyB => {}
                    // WKeyCode::KeyC => {}
                    // WKeyCode::KeyD => {}
                    // WKeyCode::KeyE => {}
                    // WKeyCode::KeyF => {}
                    // WKeyCode::KeyG => {}
                    // WKeyCode::KeyH => {}
                    // WKeyCode::KeyI => {}
                    // WKeyCode::KeyJ => {}
                    // WKeyCode::KeyK => {}
                    // WKeyCode::KeyL => {}
                    // WKeyCode::KeyM => {}
                    // WKeyCode::KeyN => {}
                    // WKeyCode::KeyO => {}
                    // WKeyCode::KeyP => {}
                    // WKeyCode::KeyQ => {}
                    // WKeyCode::KeyR => {}
                    // WKeyCode::KeyS => {}
                    // WKeyCode::KeyT => {}
                    // WKeyCode::KeyU => {}
                    // WKeyCode::KeyV => {}
                    // WKeyCode::KeyW => {}
                    // WKeyCode::KeyX => {}
                    // WKeyCode::KeyY => {}
                    // WKeyCode::KeyZ => {}
                    WKeyCode::Backspace => KeyCode::BackSpace,
                    WKeyCode::Enter => KeyCode::Enter,
                    WKeyCode::Space => KeyCode::Space,

                    WKeyCode::ArrowUp => KeyCode::Up,
                    WKeyCode::ArrowDown => KeyCode::Down,
                    WKeyCode::ArrowLeft => KeyCode::Left,
                    WKeyCode::ArrowRight => KeyCode::Right,

                    WKeyCode::Numpad1 => KeyCode::DownLeft,
                    WKeyCode::Numpad2 => KeyCode::Down,
                    WKeyCode::Numpad3 => KeyCode::DownRight,
                    WKeyCode::Numpad4 => KeyCode::Left,
                    WKeyCode::Numpad5 => KeyCode::Space,
                    WKeyCode::Numpad6 => KeyCode::Right,
                    WKeyCode::Numpad7 => KeyCode::UpLeft,
                    WKeyCode::Numpad8 => KeyCode::Up,
                    WKeyCode::Numpad9 => KeyCode::UpRight,

                    WKeyCode::Escape => KeyCode::Escape,
                    _ => KeyCode::Unknown,
                }
            }
            PhysicalKey::Unidentified(_) => KeyCode::Unknown,
        }
    }
}
