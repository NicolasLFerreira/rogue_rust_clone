use crate::game_map::tile_map::TileMap;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;
use rand::Rng;

// Picks a point in one of the walls
pub(super) fn pick_wall_point(rect: Rect) -> Point {
    match rand::rng().random_range(0..4) {
        0 => Point::new(
            rect.x,
            rand::rng().random_range(rect.y + 1..rect.y + rect.height - 1),
        ),
        1 => Point::new(
            rect.x + rect.width - 1,
            rand::rng().random_range(rect.y + 1..rect.y + rect.height - 1),
        ),
        2 => Point::new(
            rand::rng().random_range(rect.x + 1..rect.x + rect.width - 1),
            rect.y,
        ),
        3 => Point::new(
            rand::rng().random_range(rect.x + 1..rect.x + rect.width - 1),
            rect.y + rect.height - 1,
        ),
        _ => Point::ZERO,
    }
}

pub(super) fn apply_tile_map(host: &mut TileMap, other: &TileMap) {
    for (point, tile) in other.iter_tiles() {
        host.set(point, tile.clone());
    }
}
