mod action;
mod engine;
mod entities;
mod game_map;
mod geometry;
mod graphics;
mod state;
mod systems;
mod types;

use crate::engine::Engine;
use crate::geometry::rect::Rect;
use crate::graphics::graphics::Graphics;
use crate::graphics::rendering::renderers::crossterm_renderer::CrosstermRenderer;
use crate::graphics::theme::AsciiTheme;
use crate::graphics::window::implementations::crossterm_window::CrosstermGameWindow;
use crate::graphics::window::implementations::winit_window::WinitGameWindow;
use crate::state::State;
use winit::event_loop::{ControlFlow, EventLoop};

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 25;

fn main() -> std::io::Result<()> {
    // Screen dimensions
    let window_size: Rect = Rect::new_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT);
    let toggle = true;

    // winit
    if toggle {
        let graphics = Graphics::new(
            Box::new(AsciiTheme),
            Box::new(CrosstermRenderer::new(window_size)),
        );
        let window = WinitGameWindow::default();
        let state = State::new(window_size.resize(0, -3));

        let mut engine = Engine::<WinitGameWindow>::new(graphics, window, state);
        engine.run()?;

        Ok(())
    }
    // Crossterm
    else {
        // Initialise engine (utilising crossterm at the moment)
        let graphics = Graphics::new(
            Box::new(AsciiTheme),
            Box::new(CrosstermRenderer::new(window_size)),
        );
        let window = CrosstermGameWindow::new(window_size);
        let state = State::new(window_size.resize(0, -3));

        let mut engine = Engine::<CrosstermGameWindow>::new(graphics, window, state);
        engine.run()
    }
}
