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
use crate::state::State;

fn main() -> std::io::Result<()> {
    // Screen dimensions
    let sw: usize = 80; // 156
    let sh: usize = 25; // 42

    // Initialise engine (utilising crossterm at the moment)
    let window_size: Rect = Rect::new_dimensions(sw, sh);
    let graphics = Graphics::new(
        Box::new(AsciiTheme {}),
        Box::new(CrosstermRenderer::new(window_size)),
    );
    let window = Box::new(CrosstermGameWindow::new(window_size));
    let state = State::new(window_size.resize(0, -3));

    let mut engine = Engine::new(graphics, window, state);
    engine.run()
}
