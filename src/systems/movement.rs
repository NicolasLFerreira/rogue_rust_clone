use crate::entities::entity_manager::EntityManager;
use crate::game::Game;
use crate::game_map::tile_map::TileMap;
use crate::geometry::direction::Direction;
use crate::geometry::point::Point;

pub enum MoveEvent {
    Pass,
    Invalid,
    Occupied(Point),
}

pub struct MovementSystem;

impl MovementSystem {
    pub fn try_move(game: &mut Game, entity_id: usize, direction: Direction) -> MoveEvent {
        let new_point = match game.entity_manager.get_entity(entity_id) {
            Some(entity) => match entity.point.offset(direction.to_delta()) {
                Some(p) => p,
                None => return MoveEvent::Invalid,
            },
            None => return MoveEvent::Invalid,
        };

        if !Self::is_walkable(&game.tile_map, new_point) {
            return MoveEvent::Invalid;
        }

        if Self::is_occupied(&game.entity_manager, new_point) {
            return MoveEvent::Occupied(new_point);
        }

        if let Some(entity) = game.entity_manager.get_entity_mut(entity_id) {
            entity.point = new_point;
            MoveEvent::Pass
        } else {
            MoveEvent::Invalid
        }
    }

    fn is_walkable(tile_map: &TileMap, point: Point) -> bool {
        tile_map
            .safe_get(point)
            .map(|tile| tile.is_walkable())
            .unwrap_or(false)
    }

    fn is_occupied(entities: &EntityManager, point: Point) -> bool {
        entities.iter().any(|f| f.point == point)
    }
}
