mod dungeon;
mod entities;
mod game;
mod geometry;
mod input;
mod rendering;

use crate::game::Game;
use crate::geometry::delta::Delta;
use crate::geometry::rect::Rect;
use crate::input::action::*;
use crate::rendering::crossterm_renderer::CrosstermRenderer;
use crate::rendering::frame::Frame;
use crate::rendering::renderer::Renderer;
use input::input_handler::get_input;
use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::{Duration, Instant};

const FPS: u64 = 30;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / FPS);

fn main() -> io::Result<()> {
    // Top-level variables
    let sector_n = 3;
    let base = Delta::new(52, 13) * sector_n;
    let screen_rect: Rect = Rect::new_dimensions(base.dx as usize, base.dy as usize + 3);
    let map_rect: Rect = Rect::new_dimensions(base.dx as usize, base.dy as usize);

    // Renderer instance remains the same for the program's entire lifetime
    let mut renderer: dyn Renderer = CrosstermRenderer::new(screen_rect);

    'master: loop {
        // Game instance is reset in-between games.
        let mut game = Game::new(map_rect);

        // Rendering setup
        renderer.begin()?;
        let mut frame = Frame::new(screen_rect);
        let mut needs_redraw = true; // turn-based: redraw only when state changes

        // Main loop. Break this and the game ends
        'game: loop {
            // Keeps track of frame duration for FPS limiting
            let frame_start = Instant::now();

            // Renders changes from previous frame
            if needs_redraw {
                game.compose(&mut frame);
                renderer.present(&frame)?;
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
                    Action::Move(move_action) => game.move_player(move_action),
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

    renderer.end()
}
