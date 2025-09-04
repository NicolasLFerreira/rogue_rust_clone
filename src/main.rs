mod entities;
mod game;
mod game_map;
mod geometry;
mod graphics;
mod input;
mod systems;
mod types;

use crate::game::Game;
use crate::geometry::rect::Rect;
use crate::graphics::graphics::Graphics;
use crate::graphics::rendering::frame::Frame;
use crate::graphics::rendering::renderers::crossterm_renderer::CrosstermRenderer;
use crate::graphics::theme::AsciiTheme;
use crate::input::action::*;
use crate::systems::combat::Combat;
use crate::systems::movement::{MoveEvent, MovementSystem};
use input::input_handler::get_input;
use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};

const FPS: u64 = 30;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / FPS);

fn main() -> io::Result<()> {
    let sw: usize = 156;
    let sh: usize = 42;

    // Top-level variables
    let screen_rect: Rect = Rect::new_dimensions(sw, sh);
    let map_rect: Rect = Rect::new_dimensions(sw, sh - 3);

    // Renderer instance remains the same for the program's entire lifetime
    let mut graphics: Graphics = Graphics::new(
        Box::new(AsciiTheme {}),
        Box::new(CrosstermRenderer::new(screen_rect)),
    );
    let mut frame = Frame::new(screen_rect);

    'master: loop {
        // Game instance is reset in-between games.
        let mut game = Game::new(map_rect);

        // Rendering setup
        graphics.renderer().begin()?;
        frame.clear();
        let mut needs_redraw = true; // turn-based: redraw only when state changes

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
            let actions = get_input();
            if actions.len() > 0 {
                // If an action is performed, redraw the screen
                needs_redraw = true;
            };

            // Pattern matches action categories to their respective handlers
            for action in actions {
                match action {
                    Action::Move(move_action) => {
                        let player_id = game.entity_manager.player_id();
                        match MovementSystem::try_move(&mut game, player_id, move_action) {
                            MoveEvent::Occupied(id) => game.entity_manager.despawn(id),
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

            // Frame limiter
            let elapsed = frame_start.elapsed();
            if elapsed < FRAME_DURATION {
                sleep(FRAME_DURATION - elapsed);
            }
        }
    }

    graphics.renderer().end()
}
