use crate::dungeon::tile::TileType;
use crate::dungeon::tile::*;
use crate::geometry::delta::Delta;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;
use std::ops::Mul;

pub struct DungeonMap {
    pub rect: Rect,
    tile_map: Vec<Tile>,
}

// Constructor
impl DungeonMap {
    pub fn new(rect: Rect) -> DungeonMap {
        let mut tile_map: Vec<Tile> = vec![Tile::new(TileType::Floor); rect.area()];
        tile_map[0] = Tile::new(TileType::Wall);
        DungeonMap { rect, tile_map }
    }
}

// Generation
impl DungeonMap {
    pub fn generate_map(&mut self) {
        let rx = self.rect.width / 3;
        let ry = self.rect.height / 3;
        let sdf = Delta {
            dx: rx as i32,
            dy: ry as i32,
        };

        // Room 1
        for i in 0..3 {
            let room = self.generate_room(Rect::new_dimensions(rx, ry).translate(sdf.mul(i)));
            self.apply_room(room)
        }
    }

    fn generate_room(&self, rect: Rect) -> Room {
        let mut tile_map = Vec::with_capacity(rect.area());
        for y in 0..rect.height {
            for x in 0..rect.width {
                let tile = if x == 0 || x == rect.width - 1 || y == 0 || y == rect.height - 1 {
                    Tile::new(TileType::Wall)
                } else {
                    Tile::new(TileType::Floor)
                };
                tile_map.push(tile);
            }
        }
        Room { rect, tile_map }
    }

    fn apply_room(&mut self, room: Room) {
        for y in 0..room.rect.height {
            for x in 0..room.rect.width {
                let map_point = Point::new(room.rect.x + x, room.rect.y + y);

                let tile =
                    if x == 0 || x == room.rect.width - 1 || y == 0 || y == room.rect.height - 1 {
                        Tile::new(TileType::Wall)
                    } else {
                        Tile::new(TileType::Floor)
                    };

                self.set(map_point, tile);
            }
        }
    }
}

// Other
impl DungeonMap {
    // row major
    fn index(&self, point: Point) -> Option<usize> {
        if self.rect.contains(point) {
            Some((point.y - self.rect.y) * self.rect.width + (point.x - self.rect.x))
        } else {
            None
        }
    }

    pub fn get(&self, point: Point) -> Option<&Tile> {
        self.index(point).map(|i| &self.tile_map[i])
    }

    pub fn get_mut(&mut self, point: Point) -> Option<&mut Tile> {
        self.index(point).map(|i| &mut self.tile_map[i])
    }

    pub fn set(&mut self, point: Point, tile: Tile) -> bool {
        match self.index(point) {
            Some(index) => {
                self.tile_map[index] = tile;
                true
            }
            None => false,
        }
    }

    pub fn iter_tiles(&self) -> impl Iterator<Item = (Point, &Tile)> {
        self.rect
            .iter_points()
            .filter_map(move |p| self.get(p).map(|tile| (p, tile)))
    }
}

// Helper room struct
struct Room {
    rect: Rect,
    tile_map: Vec<Tile>,
}
