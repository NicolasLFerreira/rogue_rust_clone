use crate::game_map::generation::implementations::utils::{apply_tile_map, pick_wall_point};
use crate::game_map::generation::map_generator::MapGenerator;
use crate::game_map::tile::{Tile, TileType};
use crate::game_map::tile_map::TileMap;
use crate::geometry::delta::Delta;
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
        let rw = self.rect.width / 3;
        let rh = self.rect.height / 3;
        let base_region = Rect::new(0, 0, rw, rh);

        for x in 0..3 {
            for y in 0..3 {
                let region_rect = base_region.translate(Delta::new(rw as i32 * x, rh as i32 * y));
                let room_rect = self.create_room(region_rect);
                let mut room = Self::generate_tile_map(room_rect);

                // Decides whether to place a door
                let door_point = pick_wall_point(room_rect);
                room.set(door_point, Tile::new(TileType::Floor));

                apply_tile_map(tile_map, &room);
            }
        }
    }
}

// Generation
impl DungeonMapGenerator {
    fn create_room(&mut self, rect: Rect) -> Rect {
        let width = self.rng.random_range(4..rect.width - 2);
        let height = self.rng.random_range(4..rect.height - 2);
        let anchor = Point::new(
            self.rng.random_range(rect.x..rect.x + rect.width - width),
            self.rng.random_range(rect.y..rect.y + rect.height - height),
        );

        Rect::new_anchor(anchor, width, height)
    }

    fn generate_tile_map(room: Rect) -> TileMap {
        let mut vec_t: Vec<Tile> = Vec::with_capacity(room.area());

        for y in 0..room.height {
            for x in 0..room.width {
                if x == 0 || y == 0 || x == room.width - 1 || y == room.height - 1 {
                    vec_t.push(Tile::new(TileType::Wall))
                } else {
                    vec_t.push(Tile::new(TileType::Floor))
                }
            }
        }

        TileMap {
            tile_map: vec_t,
            rect: room,
        }
    }
}
