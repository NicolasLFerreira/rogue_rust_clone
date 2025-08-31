use crate::dungeon::tile::*;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;

pub struct DungeonMap {
    pub rect: Rect,
    tile_map: Vec<Tile>,
}

impl DungeonMap {
    pub fn new(rect: Rect) -> DungeonMap {
        let tile_map: Vec<Tile> = (0..rect.area())
            .map(|i| Tile::new(TileType::Floor))
            .collect();

        DungeonMap { rect, tile_map }
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

    pub fn set(&mut self, tile: Tile, point: Point) -> bool {
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
