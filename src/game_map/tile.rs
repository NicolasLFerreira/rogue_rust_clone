#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Floor,
    Wall,
    Void,
}

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub revealed: bool,
    pub visible: bool,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Self {
            tile_type,
            revealed: false,
            visible: false,
        }
    }

    pub fn empty() -> Self {
        Self {
            tile_type: TileType::Void,
            revealed: false,
            visible: false,
        }
    }

    pub fn is_walkable(&self) -> bool {
        matches!(self.tile_type, TileType::Floor)
    }

    pub fn blocks_sight(&self) -> bool {
        matches!(self.tile_type, TileType::Wall)
    }

    pub fn reveal(&mut self) {
        self.revealed = true;
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }
}
