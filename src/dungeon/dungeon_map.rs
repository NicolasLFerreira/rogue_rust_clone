use crate::dungeon::tile::TileType;
use crate::dungeon::tile::*;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;

pub struct DungeonMap {
    pub rect: Rect,
    pub(crate) tile_map: Vec<Tile>,
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
impl DungeonMap {}

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
