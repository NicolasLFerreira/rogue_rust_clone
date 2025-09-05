mod entities;
mod game_state;
mod game_map;
mod geometry;
mod graphics;
mod systems;
mod types;

use crate::game_state::GameState;
use crate::geometry::rect::Rect;
use crate::graphics::graphics::Graphics;
use crate::graphics::rendering::frame::Frame;
use crate::graphics::rendering::renderers::crossterm_renderer::CrosstermRenderer;
use crate::graphics::theme::AsciiTheme;
use crate::graphics::window::game_window::GameWindow;
use crate::graphics::window::implementations::crossterm_window::CrosstermGameWindow;
use crate::systems::combat::Combat;
use crate::systems::movement::{MoveEvent, MovementSystem};
use graphics::window::input_action_mapper::*;
use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};

const FPS: u64 = 30;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / FPS);

fn main() -> io::Result<()> {
    let sw: usize = 80; // 156
    let sh: usize = 25; // 42

    let mw: usize = 80;
    let mh: usize = 22;

    // Top-level variables
    let window_rect: Rect = Rect::new_dimensions(sw, sh);
    let map_rect: Rect = Rect::new_dimensions(mw, mh);

    let game_window: Box<dyn GameWindow> = Box::new(CrosstermGameWindow::new(window_rect));

    // Renderer instance remains the same for the program's entire lifetime
    let mut graphics: Graphics = Graphics::new(
        // Assign implementations
        Box::new(AsciiTheme {}),
        Box::new(CrosstermRenderer::new(window_rect)),
    );
    let mut frame = Frame::new(window_rect);

    // Start renderer
    graphics.renderer().begin()?;

    'master: loop {
        frame.clear();
        let mut needs_redraw = true;

        // Game instance is reset in-between games.
        let mut game = GameState::new(map_rect);

        // Main loop. Break this and the game ends
        'game: loop {
            // Keeps track of frame duration for FPS limiting
            let frame_start = Instant::now();

            // Renders changes from previous frame
            if needs_redraw {
                graphics.compose_frame(&mut frame, &game);
                graphics.renderer().present(&frame)?;
                needs_redraw = false;
            }

            // Polls for actions (i.e. input)
            let window_events = game_window.poll_events();
            let actions = input_action_mapper(window_events);
            let has_actions = actions.len() > 0;

            // Pattern matches action categories to their respective handlers
            for action in actions {
                match action {
                    Action::Move(move_action) => {
                        let player_id = game.entity_manager.player_id();
                        match MovementSystem::try_move(&mut game, player_id, move_action) {
                            MoveEvent::Occupied(mover_id, occupant_id) => {
                                Combat::initiate(&mut game, mover_id, occupant_id)
                            }
                            _ => {}
                        }
                    }
                    Action::Meta(meta_action) => match meta_action {
                        MetaAction::Quit => break 'master,
                        MetaAction::Restart => break 'game,
                        MetaAction::Wait => {}
                    },
                }
            }

            // Set redraw and move entities after player actions have been calculated
            if has_actions {
                // If an action is performed, redraw the screen
                needs_redraw = true;
                game.move_entities();
            };

            // Frame limiter
            let elapsed = frame_start.elapsed();
            if elapsed < FRAME_DURATION {
                sleep(FRAME_DURATION - elapsed);
            }
        }
    }

    graphics.renderer().end()
}
