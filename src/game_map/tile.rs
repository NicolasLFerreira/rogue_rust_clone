#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileKind {
    Void,
    Floor,
    Wall,
    Door,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileCollision {
    Passable,
    Blocked,
}

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub kind: TileKind,
    pub collision: TileCollision,
    pub revealed: bool,
    pub visible: bool,
}

impl Tile {
    pub fn new(kind: TileKind) -> Self {
        let tile_property = match kind {
            TileKind::Floor | TileKind::Door => TileCollision::Passable,
            _ => TileCollision::Blocked,
        };

        Self {
            kind,
            collision: tile_property,
            revealed: true,
            visible: true,
        }
    }

    pub fn empty() -> Self {
        Self {
            kind: TileKind::Void,
            collision: TileCollision::Blocked,
            revealed: false,
            visible: false,
        }
    }

    pub fn is_walkable(&self) -> bool {
        matches!(self.kind, TileKind::Floor | TileKind::Door)
            && self.collision == TileCollision::Passable
    }

    pub fn blocks_sight(&self) -> bool {
        matches!(self.kind, TileKind::Wall)
    }

    pub fn reveal(&mut self) {
        self.revealed = true;
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }
}
