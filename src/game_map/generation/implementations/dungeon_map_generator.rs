use crate::game_map::generation::implementations::utils::{Region, apply_tile_map};
use crate::game_map::generation::map_generator::MapGenerator;
use crate::game_map::tile::{Tile, TileType};
use crate::game_map::tile_map::TileMap;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;
use rand::Rng;
use rand::rngs::ThreadRng;

pub struct DungeonMapGenerator {
    rect: Rect,
    rng: ThreadRng,
}

impl DungeonMapGenerator {
    pub fn new(rect: Rect) -> Self {
        DungeonMapGenerator {
            rect,
            rng: rand::rng(),
        }
    }
}

// Trait
impl MapGenerator for DungeonMapGenerator {
    fn generate_map(&mut self, tile_map: &mut TileMap) {
        for region in self.regions(6).iter() {
            Self::apply_region(tile_map, *region)
        }
    }
}

// Generation
impl DungeonMapGenerator {
    fn regions(&mut self, n: usize) -> Vec<Region> {
        (0..n).map(|_| self.create_region()).collect()
    }

    fn create_region(&mut self) -> Region {
        let width = self.rng.random_range(4..12);
        let height = self.rng.random_range(4..12);
        let anchor = Point::new(
            self.rng.random_range(0..self.rect.width - width),
            self.rng.random_range(0..self.rect.height - height),
        );

        Region::new_anchor(anchor, width, height)
    }

    fn apply_region(tile_map: &mut TileMap, region: Region) {
        let mut vec_t: Vec<Tile> = Vec::with_capacity(region.area());

        for y in 0..region.height {
            for x in 0..region.width {
                if x == 0 || y == 0 || x == region.width - 1 || y == region.height - 1 {
                    vec_t.push(Tile::new(TileType::Wall))
                } else {
                    vec_t.push(Tile::new(TileType::Floor))
                }
            }
        }

        let tm = TileMap {
            tile_map: vec_t,
            rect: region,
        };

        apply_tile_map(tile_map, &tm);
    }
}
