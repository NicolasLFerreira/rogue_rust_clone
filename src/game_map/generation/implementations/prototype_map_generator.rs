use crate::game_map::generation::implementations::utils::{apply_tile_map, pick_wall_point};
use crate::game_map::generation::map_generator::MapGenerator;
use crate::game_map::tile::{Tile, TileType};
use crate::game_map::tile_map::TileMap;
use crate::geometry::delta::Delta;
use crate::geometry::rect::Rect;

pub struct PrototypeMapGenerator {
    rect: Rect,
}

// Constructor
impl PrototypeMapGenerator {
    pub fn new(rect: Rect) -> Self {
        PrototypeMapGenerator { rect }
    }
}

// Code
impl PrototypeMapGenerator {
    fn generate_region(&self, rect: Rect) -> TileMap {
        let mut tile_map = Vec::with_capacity(rect.area());
        for y in 0..rect.height {
            for x in 0..rect.width {
                let tile = if x == 0 || x == rect.width - 1 || y == 0 || y == rect.height - 1 {
                    Tile {
                        tile_type: TileType::Wall,
                        visible: true,
                        revealed: true,
                    }
                } else {
                    Tile {
                        tile_type: TileType::Floor,
                        visible: true,
                        revealed: true,
                    }
                };
                tile_map.push(tile);
            }
        }
        let mut region = TileMap { rect, tile_map };

        // Places door
        let door = pick_wall_point(rect);
        match region.get_mut(door) {
            Some(tile) => {
                tile.tile_type = TileType::Floor;
            }
            None => {}
        }
        region
    }
}

impl MapGenerator for PrototypeMapGenerator {
    fn generate_map(&self, map: &mut TileMap) {
        let rx = self.rect.width / 3;
        let ry = self.rect.height / 3;
        let sdf = Delta {
            dx: rx as i32,
            dy: ry as i32,
        };

        // Regions
        for i in 0..3 {
            let region = self.generate_region(Rect::new_dimensions(rx, ry).translate(sdf * i));
            apply_tile_map(map, &region)
        }
    }
}
