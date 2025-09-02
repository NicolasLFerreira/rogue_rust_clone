use crate::game_map::tile_map::TileMap;

pub trait MapGenerator {
    fn generate_map(&mut self, tile_map: &mut TileMap);
}
