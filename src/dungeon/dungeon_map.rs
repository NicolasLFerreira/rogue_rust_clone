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
        self.rect
            .contains(point)
            .then(|| point.y * self.rect.width + point.x)
    }

    pub fn get(&self, point: Point) -> Option<&Tile> {
        self.index(point).map(|i| &self.tile_map[i])
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
}
