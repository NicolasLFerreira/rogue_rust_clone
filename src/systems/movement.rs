use crate::entities::entity_manager::EntityManager;
use crate::state::State;
use crate::game_map::tile_map::TileMap;
use crate::geometry::direction::Direction;
use crate::geometry::point::Point;
use crate::types::Id;

pub enum MoveEvent {
    Pass,
    Invalid,
    Occupied(Id, Id),
}

pub struct MovementSystem;

impl MovementSystem {
    pub fn try_move(game: &mut State, mover_id: Id, direction: Direction) -> MoveEvent {
        let new_point = match game.entity_manager.get_entity(mover_id) {
            Some(entity) => match entity.point.offset(direction.to_delta()) {
                Some(p) => p,
                None => return MoveEvent::Invalid,
            },
            None => return MoveEvent::Invalid,
        };

        if !Self::is_walkable(&game.tile_map, new_point) {
            return MoveEvent::Invalid;
        }

        if let Some(occupant_id) = Self::is_occupied(&game.entity_manager, new_point) {
            return MoveEvent::Occupied(mover_id, occupant_id);
        }

        if let Some(entity) = game.entity_manager.get_entity_mut(mover_id) {
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

    fn is_occupied(entities: &EntityManager, point: Point) -> Option<Id> {
        entities.iter().find(|e| e.point == point).map(|e| e.id())
    }
}
