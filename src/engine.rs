use crate::graphics::graphics::Graphics;
use crate::graphics::rendering::frame::Frame;
use crate::graphics::window::game_window::GameWindow;
use crate::graphics::window::input_action_mapper::{Action, MetaAction, input_action_mapper};
use crate::state::State;
use crate::systems::combat::Combat;
use crate::systems::movement::{MoveEvent, MovementSystem};
use std::thread::sleep;
use std::time::{Duration, Instant};

// FPS hardcoded at the moment
const FPS: u64 = 30;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / FPS);

// Master struct. Holds references to all major components
pub struct Engine {
    graphics: Graphics,
    window: Box<dyn GameWindow>,
    state: State,
}

// Implementations are defined in main()
// The engine should remain agnostic to the actual running environment
impl Engine {
    pub fn new(graphics: Graphics, window: Box<dyn GameWindow>, state: State) -> Self {
        Self {
            graphics,
            window,
            state,
        }
    }
}

impl Engine {
    pub fn run(&mut self) -> std::io::Result<()> {
        self.graphics.renderer().begin()?;
        let window_size = self.window.size();
        let mut frame = Frame::new(window_size);

        'master: loop {
            frame.clear();
            let mut needs_redraw = true;

            // Game instance is reset in-between games.
            self.state = State::new(self.window.size().resize(0, -3));

            // Main loop. Break this and the game ends
            'game: loop {
                // Keeps track of frame duration for FPS limiting
                let frame_start = Instant::now();

                // Renders changes from previous frame
                if needs_redraw {
                    self.graphics.compose_frame(&self.state, &mut frame);
                    self.graphics.renderer().present(&mut frame)?;
                    needs_redraw = false;
                }

                // Polls for actions (i.e. input)
                let window_events = self.window.poll_events();
                let actions = input_action_mapper(window_events);
                let has_actions = actions.len() > 0;

                // Pattern matches action categories to their respective handlers
                for action in actions {
                    match action {
                        Action::Move(move_action) => {
                            let player_id = self.state.entity_manager.player_id();
                            match MovementSystem::try_move(&mut self.state, player_id, move_action)
                            {
                                MoveEvent::Occupied(mover_id, occupant_id) => {
                                    Combat::fight(&mut self.state, mover_id, occupant_id)
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
                    self.state.move_entities();
                };

                // Frame limiter
                let elapsed = frame_start.elapsed();
                if elapsed < FRAME_DURATION {
                    sleep(FRAME_DURATION - elapsed);
                }
            }
        }

        self.graphics.renderer().end()
    }
}
