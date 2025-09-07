use crate::game::action::{Action, MetaAction, input_action_mapper};
use crate::game::state::State;
use crate::graphics::graphics::Graphics;
use crate::graphics::rendering::frame::Frame;
use crate::graphics::window::game_window::{GameWindow, WindowEvent};
use crate::systems::combat::Combat;
use crate::systems::movement::{MoveEvent, MovementSystem};
use std::thread::sleep;
use std::time::{Duration, Instant};

// FPS hardcoded at the moment
const FPS: u64 = 30;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / FPS);

// Master struct. Holds references to all major components
pub struct Engine<W: GameWindow> {
    graphics: Graphics,
    window: W,
    state: State,
    event_queue: Vec<WindowEvent>,
}

// Implementations are defined in main()
// The engine should remain agnostic to the actual running environment
impl<W: GameWindow> Engine<W> {
    pub fn new(graphics: Graphics, window: W, state: State) -> Self {
        Self {
            graphics,
            window,
            state,
            event_queue: Vec::new(),
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        self.graphics.renderer().begin()?;
        let mut frame = Frame::new(self.window.size());

        loop {
            // Duration of frame for FPS limitation
            let frame_start = Instant::now();

            // Input
            self.input();

            // Render
            self.graphics.compose_frame(&self.state, &mut frame);
            self.graphics.renderer().present(&mut frame)?;

            // Frame limiter
            let elapsed = frame_start.elapsed();
            if elapsed < FRAME_DURATION {
                sleep(FRAME_DURATION - elapsed);
            }
        }

        self.graphics.renderer().end()
    }

    fn input(&mut self) {
        self.event_queue.extend(self.window.poll_events());
        let events: Vec<WindowEvent> = self.event_queue.drain(..).map(|x| x).collect();
        for event in events {
            let action = input_action_mapper(event);
            self.handle_action(action);
            self.state.move_entities();
        }
    }

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::Move(move_action) => {
                let player_id = self.state.entity_manager.player_id();
                match MovementSystem::try_move_direction(
                    &mut self.state.entity_manager,
                    &self.state.tile_map,
                    player_id,
                    move_action,
                ) {
                    MoveEvent::Occupied(mover_id, occupant_id) => {
                        Combat::fight(&mut self.state, mover_id, occupant_id)
                    }
                    _ => {}
                }
            }
            Action::Meta(meta_action) => match meta_action {
                MetaAction::Quit => std::process::exit(0),
                MetaAction::Restart => self.reset_game_state(),
                MetaAction::Wait => {}
            },
        }
    }

    // Temporary
    fn reset_game_state(&mut self) {
        self.state = State::new(self.window.size().resize(0, -3));
    }
}
