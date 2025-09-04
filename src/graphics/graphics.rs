use crate::game::Game;
use crate::geometry::point::Point;
use crate::graphics::rendering::frame::Frame;
use crate::graphics::rendering::renderer::Renderer;
use crate::graphics::theme::Theme;
use crossterm::style::Color;

// The master object for holding graphics and rendering related code
pub struct Graphics {
    theme: Box<dyn Theme>,
    renderer: Box<dyn Renderer>,
}

impl Graphics {
    pub fn new(theme: Box<dyn Theme>, renderer: Box<dyn Renderer>) -> Self {
        Graphics { theme, renderer }
    }

    pub fn renderer(&mut self) -> &mut dyn Renderer {
        &mut *self.renderer
    }

    pub fn compose_frame(&self, frame: &mut Frame, game_state: &Game) {
        // Map
        for (point, tile) in game_state.tile_map.iter_tiles() {
            frame.put(point, self.theme.tile_theme(tile))
        }

        // Entities
        for entity in game_state.entity_manager.iter() {
            if game_state.tile_map.get(entity.point).visible {
                frame.put(entity.point, self.theme.entity_theme(entity));
            }
        }

        // UI
        frame.put_str(
            Point::new(0, game_state.tile_map.rect.height),
            "q to quit",
            Color::Yellow,
            Color::Black,
        );
    }
}
