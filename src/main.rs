mod dungeon;
mod entities;
mod game;
mod input;
mod rendering;
mod types;

use crate::game::Game;
use crate::input::types::*;
use crate::rendering::crossterm_renderer::CrosstermRenderer;
use crate::rendering::frame::Frame;
use crate::rendering::renderer::Renderer;
use input::input_handler::get_input;
use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::{Duration, Instant};

const FPS: u64 = 60;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / FPS);

fn main() -> io::Result<()> {
    let (w, h) = (25, 25);
    let mut game = Game::new(w, h);
    let mut renderer = CrosstermRenderer::new(w, h);
    renderer.begin()?;

    let mut frame = Frame::new(w, h);
    let mut needs_redraw = true; // turn-based: redraw only when state changes

    'game: loop {
        let frame_start = Instant::now();

        if needs_redraw {
            game.compose(&mut frame);
            renderer.present(&frame)?;
            needs_redraw = false;
        }

        // Input

        let actions = get_input();
        let any_action = actions.len() > 0;

        for action in actions {
            match action {
                Action::Move(move_action) => game.move_player(move_action),
                Action::Meta(meta_action) => match meta_action {
                    MetaAction::Quit => break 'game,
                    MetaAction::Wait => {}
                },
            }
        }

        if any_action {
            needs_redraw = true
        }

        // Frame limiter
        let elapsed = frame_start.elapsed();
        if elapsed < FRAME_DURATION {
            sleep(FRAME_DURATION - elapsed);
        }
    }

    renderer.end()
}

fn meta_handler(action: MetaAction) {}
