use crate::geometry::point::Point;
use crate::graphics::color::Color;
use crate::graphics::rendering::frame::Frame;
use crate::graphics::rendering::renderer::Renderer;
use crate::graphics::theme::Theme;
use crate::state::State;

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

    pub fn compose_frame(&mut self, state: &State, frame: &mut Frame) {
        // Map
        for (point, tile) in state.tile_map.iter_tiles() {
            frame.put(point, self.theme.tile_theme(tile))
        }

        // Entities
        for entity in state.entity_manager.iter() {
            if state.tile_map.get(entity.point).visible {
                frame.put(entity.point, self.theme.entity_theme(entity));
            }
        }

        // UI
        frame.put_str(
            Point::new(0, state.tile_map.rect.height),
            "'q' to quit",
            Color::Yellow,
            Color::Black,
        );

        frame.put_str(
            Point::new(0, state.tile_map.rect.height + 1),
            "'r' to restart",
            Color::Yellow,
            Color::Black,
        );

        let stats = state.entity_manager.player().stats.clone();
        let stats_string = &format!(
            "| hp: {}({}) - atk: {}",
            stats.cur_hp, stats.max_hp, stats.atk
        );

        frame.put_str(
            Point::new(12, state.tile_map.rect.height),
            stats_string,
            Color::Red,
            Color::Black,
        )
    }
}
