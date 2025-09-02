use crate::dungeon::dungeon_map::DungeonMap;
use crate::dungeon::tile::{Tile, TileType};
use crate::geometry::delta::Delta;
use crate::geometry::rect::Rect;

pub struct DungeonMapGenerator {
    rect: Rect,
}

// Constructor
impl DungeonMapGenerator {
    pub fn new(rect: Rect) -> Self {
        DungeonMapGenerator { rect }
    }
}

// Generation
impl DungeonMapGenerator {
    pub fn generate_map(&self, map: &mut DungeonMap) {
        let rx = self.rect.width / 3;
        let ry = self.rect.height / 3;
        let sdf = Delta {
            dx: rx as i32,
            dy: ry as i32,
        };

        // Rooms
        for i in 0..3 {
            let room = self.generate_room(Rect::new_dimensions(rx, ry).translate(sdf * i));
            self.apply_room(map, &room)
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

    fn apply_room(&self, map: &mut DungeonMap, room: &DungeonMap) {
        for (point, tile) in room.iter_tiles() {
            map.set(point, tile.clone());
        }
    }
}
