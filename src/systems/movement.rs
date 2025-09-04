use crate::game::Game;
use crate::geometry::direction::Direction;
use crate::geometry::point::Point;

pub struct MovementSystem;

impl MovementSystem {
    pub fn try_move(game: &mut Game, entity_id: usize, direction: Direction) {
        let delta = direction.to_delta();
        if let Some(new_point) = game
            .entity_manager
            .get_entity(entity_id)
            .point
            .offset(delta)
        {
            if Self::can_move_to_tile(game, new_point) {
                game.entity_manager.get_entity_mut(entity_id).point = new_point;
            }
        }
    }

    fn can_move_to_tile(game: &mut Game, point: Point) -> bool {
        game.tile_map
            .safe_get(point)
            .map(|tile| tile.is_walkable())
            .unwrap_or(false)
    }
}
