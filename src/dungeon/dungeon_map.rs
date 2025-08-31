use crate::dungeon::tile::*;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;

pub struct DungeonMap {
    pub rect: Rect,
    tile_map: Vec<Tile>,
}

impl DungeonMap {
    pub fn new(rect: Rect) -> DungeonMap {
        let tile_map: Vec<Tile> = vec![Tile::empty(); rect.area()];
        DungeonMap { rect, tile_map }
    }

    pub fn generate_map(&mut self) {
        let room_rect = Rect {
            x: (self.rect.height / 3) - 3,
            y: (self.rect.width / 3) - 3,
            height: self.rect.height / 3,
            width: self.rect.width / 3,
        };
        let room = Self::generate_room(room_rect);
        self.apply_room(room);
    }

    fn generate_room(rect: Rect) -> Room {
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

struct Room {
    rect: Rect,
    tile_map: Vec<Tile>,
}
