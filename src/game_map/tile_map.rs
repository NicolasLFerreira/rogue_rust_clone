use crate::game_map::tile::TileKind;
use crate::game_map::tile::*;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;

pub struct TileMap {
    pub rect: Rect,
    pub(super) tile_map: Vec<Tile>,
}

// Constructor
impl TileMap {
    pub fn new(rect: Rect) -> TileMap {
        TileMap {
            rect,
            tile_map: vec![Tile::empty(); rect.area()],
        }
    }
}

// Other
impl TileMap {
    fn index(&self, point: Point) -> Option<usize> {
        if self.rect.contains(point) {
            Some((point.y - self.rect.y) * self.rect.width + (point.x - self.rect.x))
        } else {
            None
        }
    }

    pub fn get(&self, point: Point) -> Tile {
        self.index(point)
            .map(|i| self.tile_map[i])
            .unwrap_or(Tile::empty())
    }

    pub fn safe_get(&self, point: Point) -> Option<Tile> {
        self.index(point).map(|i| self.tile_map[i])
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

    pub fn walkable_neighbours(&self, point: Point) -> Vec<Point> {
        point
            .neighbors()
            .iter()
            .filter(|p| self.get(**p).is_walkable())
            .cloned()
            .collect()
    }

    pub fn iter_tiles(&self) -> impl Iterator<Item = (Point, Tile)> {
        self.rect
            .iter_points()
            .filter_map(move |p| self.safe_get(p).map(|tile| (p, tile)))
    }

    pub fn rnd_floor_point(&self) -> Point {
        loop {
            let point = self.rnd_point();
            if self.safe_get(point).unwrap().kind == TileKind::Floor {
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
