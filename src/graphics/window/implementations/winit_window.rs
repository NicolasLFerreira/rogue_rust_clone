use crate::geometry::rect::Rect;
use crate::graphics::window::game_window::{GameWindow, WindowEvent};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent as WWindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct WinitGameWindow {
    screen_size: Rect,
    window: Option<Window>,
}

impl GameWindow for WinitGameWindow {
    fn size(&self) -> Rect {
        self.screen_size
    }

    fn poll_events(&self) -> Vec<WindowEvent> {
        todo!()
    }

    fn is_open(&self) -> bool {
        todo!()
    }
}

impl ApplicationHandler for WinitGameWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WWindowEvent) {
        match event {
            WWindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WWindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
