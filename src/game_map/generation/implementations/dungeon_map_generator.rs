use crate::game_map::generation::implementations::utils::{apply_tile_map, pick_wall_point};
use crate::game_map::generation::map_generator::MapGenerator;
use crate::game_map::tile::{Tile, TileType};
use crate::game_map::tile_map::TileMap;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

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

        // Min 3, max 9 rooms
        let n = self.rng.random_range(3..=9);

        let mut available: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        available.shuffle::<ThreadRng>(&mut self.rng);

        let mut selected = vec![false; 9];

        for t in 0..n {
            let i = available[8 - t];
            selected[i as usize] = true;
        }

        for x in 0..3 {
            for y in 0..3 {
                if selected[y * 3 + x] {
                    let region_anchor = Point::new(rw * x, rh * y);
                    let region_rect = Rect::new_anchor(region_anchor, rw, rh);
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
}

// Generation
impl DungeonMapGenerator {
    fn create_room(&mut self, rect: Rect) -> Rect {
        let min_size = 4;

        let width = self
            .rng
            .random_range(min_size..=(rect.width - 2).max(min_size));
        let height = self
            .rng
            .random_range(min_size..=(rect.height - 2).max(min_size));

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
