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
        let mut tile_map: Vec<Tile> = vec![Tile::empty(); rect.area()];
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

        // Rooms
        for i in 0..3 {
            let room = self.generate_room(Rect::new_dimensions(rx, ry).translate(sdf.mul(i)));
            self.apply_room(&room)
        }
    }

    fn generate_room(&self, rect: Rect) -> DungeonMap {
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
        let mut room = DungeonMap { rect, tile_map };

        // Places door
        let door = rect.pick_edge_point();
        match room.get_mut(door) {
            Some(tile) => {
                tile.tile_type = TileType::Floor;
            }
            None => {}
        }
        room
    }

    fn apply_room(&mut self, room: &DungeonMap) {
        for (point, tile) in room.iter_tiles() {
            self.set(point, tile.clone());
        }
    }
}

// Other
impl DungeonMap {
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

    pub fn rnd_floor_point(&self) -> Point {
        loop {
            let point = self.rnd_point();
            if self.get(point).unwrap().tile_type == TileType::Floor {
                return point;
            }
        }
    }

    fn rnd_point(&self) -> Point {
        Point::new(
            rand::random_range(self.rect.x..self.rect.x + self.rect.width),
            rand::random_range(self.rect.y..self.rect.y + self.rect.height),
        )
    }
}
