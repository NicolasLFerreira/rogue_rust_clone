use crate::entities::entity::Entity;
use crate::entities::entity_manager::EntityManager;
use crate::game_map::tile::Tile;
use crate::game_map::tile_map::TileMap;
use crate::geometry::direction::Direction;
use crate::geometry::point::Point;
use crate::types::Id;
use std::collections::{HashMap, VecDeque};

pub enum MoveEvent {
    Pass,
    Invalid,
    Occupied(Id, Id),
}

pub struct MovementSystem;

impl MovementSystem {
    pub fn try_move_npc(
        entity_manager: &mut EntityManager,
        tile_map: &TileMap,
        mover_id: Id,
        target: Point,
    ) -> MoveEvent {
        let entity: &Entity = match entity_manager.get_entity(mover_id) {
            Some(entity) => entity,
            None => return MoveEvent::Invalid,
        };

        // // Calculates direction (this part can be replaced with an actual algorithm)
        let mhd = Self::bfs_next(tile_map, entity.point, target) - entity.point;
        let dir = mhd.to_direction();

        // Tries moving in direction
        Self::try_move_direction(entity_manager, tile_map, mover_id, dir)
    }

    pub fn try_move_direction(
        entity_manager: &mut EntityManager,
        tile_map: &TileMap,
        mover_id: Id,
        direction: Direction,
    ) -> MoveEvent {
        let new_point = match entity_manager.get_entity(mover_id) {
            Some(entity) => match entity.point.offset(direction.to_delta()) {
                Some(p) => p,
                None => return MoveEvent::Invalid,
            },
            None => return MoveEvent::Invalid,
        };

        if !Self::is_walkable(&tile_map, new_point) {
            return MoveEvent::Invalid;
        }

        if let Some(occupant_id) = Self::is_occupied(&entity_manager, new_point) {
            return MoveEvent::Occupied(mover_id, occupant_id);
        }

        if let Some(entity) = entity_manager.get_entity_mut(mover_id) {
            entity.point = new_point;
            MoveEvent::Pass
        } else {
            MoveEvent::Invalid
        }
    }

    fn is_walkable(tile_map: &TileMap, point: Point) -> bool {
        tile_map
            .safe_get(point)
            .map(|tile: Tile| tile.is_walkable())
            .unwrap_or(false)
    }

    fn is_occupied(entities: &EntityManager, point: Point) -> Option<Id> {
        entities.iter().find(|e| e.point == point).map(|e| e.id())
    }

    fn bfs_next(tile_map: &TileMap, start: Point, end: Point) -> Point {
        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut came_from: HashMap<Point, Option<Point>> = HashMap::new();
        came_from.insert(start, None);

        while let Some(current) = queue.pop_front() {
            if current == end {
                break;
            }

            for neighbour in tile_map.walkable_neighbours(current) {
                if !came_from.contains_key(&neighbour) {
                    queue.push_back(neighbour);
                    came_from.insert(neighbour, Some(current));
                }
            }
        }

        let mut path = vec![end];
        let mut current = end;
        while let Some(&Some(prev)) = came_from.get(&current) {
            path.push(prev);
            current = prev;
        }
        path.reverse();
        if let Some(next) = path.get(1) {
            *next
        } else {
            // edge case where entities are overlapping
            start
        }
    }
}
