use crate::dungeon::tile::*;

pub struct DungeonMap {
    pub width: usize,
    pub height: usize,
    tile_map: Vec<Tile>,
}

impl DungeonMap {
    pub fn new(width: usize, height: usize) -> DungeonMap {
        let tile_map: Vec<Tile> = (0..width * height)
            .map(|i| Tile::new(TileType::Floor))
            .collect();

        DungeonMap {
            width,
            height,
            tile_map,
        }
    }

    fn in_bound(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    // row major
    fn index(&self, x: usize, y: usize) -> Option<usize> {
        self.in_bound(x, y).then(|| y * self.width + x)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.index(x, y).map(|i| &self.tile_map[i])
    }

    pub fn set(&mut self, x: usize, y: usize, tile: Tile) -> bool {
        match self.index(x, y) {
            Some(index) => {
                self.tile_map[index] = tile;
                true
            }
            None => false,
        }
    }
}
