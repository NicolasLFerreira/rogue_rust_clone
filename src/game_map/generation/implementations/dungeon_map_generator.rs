use crate::game_map::generation::implementations::utils::apply_tile_map;
use crate::game_map::generation::map_generator::MapGenerator;
use crate::game_map::tile::{Tile, TileKind};
use crate::game_map::tile_map::TileMap;
use crate::geometry::delta::Delta;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;
use rand::Rng;
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::cmp::Ordering;

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
        // Base values for regions layout and count
        let rx: usize = 3;
        let ry: usize = 3;
        let ra: usize = rx * ry;

        // Region dimensions
        let rw = self.rect.width / rx;
        let rh = self.rect.height / ry;

        // Produces random amount of rooms and takes positions
        let room_count = self.biased_room_count(ra);
        let mut available: Vec<usize> = (0..ra).collect();
        let (selected_indices, _) = available.partial_shuffle(&mut self.rng, room_count);

        // Keeps track of room locations for the corridor algorithm
        let mut rooms: Vec<Rect> = vec![];

        for &mut i in selected_indices {
            // Deconstructs row-major index
            let x = i % rx;
            let y = i / rx;

            // Room anchor and dimensions
            let region_anchor = Point::new(rw * x, rh * y);
            let region_rect = Rect::new_anchor(region_anchor, rw, rh);

            // Produces room
            let room_rect = self.create_room(region_rect);
            rooms.push(room_rect);
            let room_map = Self::generate_tile_map(room_rect);

            apply_tile_map(tile_map, &room_map);
        }

        self.corridor_algorithm(tile_map, rooms);
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
                    vec_t.push(Tile::new(TileKind::Wall))
                } else {
                    vec_t.push(Tile::new(TileKind::Floor))
                }
            }
        }

        TileMap {
            tile_map: vec_t,
            rect: room,
        }
    }

    fn biased_room_count(&mut self, max_count: usize) -> usize {
        let min_rooms = max_count.isqrt();
        let room_counts: Vec<usize> = (min_rooms..=max_count).collect();
        let mid = (min_rooms + max_count) / 2;

        let weights: Vec<usize> = room_counts
            .iter()
            .map(|&x| {
                // peak at mid-point
                let dist = (x as isize - mid as isize).abs();
                (max_count as isize + dist * 2).max(1) as usize
            })
            .collect();
        let dist = WeightedIndex::new(&weights).unwrap();
        room_counts[dist.sample(&mut self.rng)]
    }

    fn corridor_algorithm(&mut self, tile_map: &mut TileMap, rooms: Vec<Rect>) {
        let mst = self.mst(tile_map, rooms);
        for conn in mst {
            self.carve_corridor(tile_map, conn.from, conn.to);
        }
    }

    fn carve_corridor(&mut self, map: &mut TileMap, origin: Point, end: Point) {
        let mut first_wall = false;
        let mut current_point = origin;
        let mut delta = (origin - end).normalize();
        loop {
            if delta.dx != 0 && delta.dy != 0 {
                let coin_flip = self.rng.random_bool(0.5);
                if coin_flip {
                    delta.dx = 0;
                } else {
                    delta.dy = 0;
                }
            }

            if let Some(valid_point) = current_point.difference(delta) {
                current_point = valid_point;

                if let Some(tile) = map.get_mut(current_point) {
                    match tile.kind {
                        TileKind::Void => tile.convert_to_corridor(),
                        TileKind::Wall => {
                            tile.convert_to_door();
                        }
                        _ => {}
                    }
                }

                // Next
                delta = (current_point - end).normalize();
                if delta == Delta::ZERO {
                    break;
                }
            }
        }
    }

    fn mst(&mut self, tile_map: &mut TileMap, rooms: Vec<Rect>) -> Vec<Connection> {
        // Calculates the center of the rooms
        let centers: Vec<Point> = rooms.iter().map(|room| room.mid_point()).collect();

        // Calculate weighed connections
        let mut connections: Vec<Connection> = vec![];
        for (i, from) in centers.iter().enumerate() {
            for to in &centers[i + 1..] {
                connections.push(Connection {
                    from: *from,
                    to: *to,
                    weigh: from.manhattan_distance(*to),
                })
            }
        }

        // Sorts by weigh
        connections.sort_by_key(|c| c.weigh);
        let mut uf = UnionFind::new(centers.len());

        // Return value of the mst
        let mut mst: Vec<Connection> = vec![];
        for conn in connections {
            let from_idx = centers.iter().position(|p| *p == conn.from).unwrap();
            let to_idx = centers.iter().position(|p| *p == conn.to).unwrap();

            if uf.find(from_idx) != uf.find(to_idx) {
                uf.union(from_idx, to_idx);
                mst.push(conn)
            }
        }

        mst
    }
}

struct Connection {
    from: Point,
    to: Point,
    weigh: usize,
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        self.parent[ra] = rb;
    }
}
