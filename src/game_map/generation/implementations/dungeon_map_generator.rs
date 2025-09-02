use crate::game_map::generation::implementations::utils::apply_tile_map;
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
        for room in self.rooms(6).iter() {
            Self::apply_room(tile_map, *room)
        }
    }
}

// Generation
impl DungeonMapGenerator {
    fn rooms(&mut self, n: usize) -> Vec<Room> {
        let mut rooms: Vec<Room> = Vec::with_capacity(n);

        for _ in 0..n {
            loop {
                let trial_room = self.create_room();
                if !rooms.iter().any(|r| r.intersect(trial_room)) {
                    rooms.push(trial_room);
                    break;
                }
            }
        }

        rooms
    }

    fn create_room(&mut self) -> Room {
        let width = self.rng.random_range(4..12);
        let height = self.rng.random_range(4..12);
        let anchor = Point::new(
            self.rng.random_range(0..self.rect.width - width),
            self.rng.random_range(0..self.rect.height - height),
        );

        Room::new_anchor(anchor, width, height)
    }

    fn apply_room(tile_map: &mut TileMap, room: Room) {
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

        let tm = TileMap {
            tile_map: vec_t,
            rect: room,
        };

        apply_tile_map(tile_map, &tm);
    }
}

type Room = Rect;
