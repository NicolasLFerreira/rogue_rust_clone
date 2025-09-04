use crate::game::Game;
use crate::game_map::tile_map::TileMap;
use crate::geometry::direction::Direction;
use crate::geometry::point::Point;

pub struct MovementSystem;

impl MovementSystem {
    pub fn try_move(game: &mut Game, entity_id: usize, direction: Direction) -> bool {
        if let Some(entity) = game.entity_manager.get_entity_mut(entity_id) {
            if let Some(new_point) = entity.point.offset(direction.to_delta()) {
                if Self::can_move_to_tile(&game.tile_map, new_point) {
                    entity.point = new_point;
                    return true;
                }
            }
        }
        false
    }

    fn can_move_to_tile(tile_map: &TileMap, point: Point) -> bool {
        tile_map
            .safe_get(point)
            .map(|tile| tile.is_walkable())
            .unwrap_or(false)
    }
}
