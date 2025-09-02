use crate::game_map::tile_map::TileMap;

pub trait MapGenerator {
    fn generate_map(&self, tile_map: &mut TileMap);
}
