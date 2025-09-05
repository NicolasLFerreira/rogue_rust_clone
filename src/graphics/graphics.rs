use crate::game_state::GameState;
use crate::geometry::point::Point;
use crate::graphics::color::Color;
use crate::graphics::rendering::frame::Frame;
use crate::graphics::rendering::renderer::Renderer;
use crate::graphics::theme::Theme;

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

    pub fn compose_frame(&self, frame: &mut Frame, game_state: &GameState) {
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
            "'q' to quit",
            Color::Yellow,
            Color::Black,
        );

        frame.put_str(
            Point::new(0, game_state.tile_map.rect.height + 1),
            "'r' to restart",
            Color::Yellow,
            Color::Black,
        );

        let stats = game_state.entity_manager.player().stats.clone();
        let stats_string = &format!(
            "| hp: {}({}) - atk: {}",
            stats.cur_hp, stats.max_hp, stats.atk
        );

        frame.put_str(
            Point::new(12, game_state.tile_map.rect.height),
            stats_string,
            Color::Red,
            Color::Black,
        )
    }
}
