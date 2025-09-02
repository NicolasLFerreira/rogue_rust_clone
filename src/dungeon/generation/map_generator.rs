use crate::dungeon::dungeon_map::DungeonMap;
use crate::geometry::rect::Rect;

pub trait MapGenerator {
    fn generate_map(&self, dungeon_map: &mut DungeonMap);
}
