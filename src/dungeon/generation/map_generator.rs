use crate::dungeon::dungeon_map::DungeonMap;

pub trait MapGenerator {
    fn generate_map(&self, dungeon_map: &mut DungeonMap);
}
