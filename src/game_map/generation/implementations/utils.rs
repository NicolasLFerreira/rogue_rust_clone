use crate::game_map::tile_map::TileMap;

pub(super) fn apply_tile_map(host: &mut TileMap, other: &TileMap) {
    for (point, tile) in other.iter_tiles() {
        host.set(point, tile.clone());
    }
}
