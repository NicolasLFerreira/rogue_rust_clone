use crate::geometry::point::Point;

pub enum EntityKind {
    Player,
    Enemy,
}

pub struct Stats {
    pub max_hp: i32,
    pub cur_hp: i32,
    pub atk: i32,
    pub def: i32,
}

pub struct Entity {
    pub point: Point,
    pub kind: EntityKind,
    pub stats: Stats,
}

impl Entity {
    pub fn new(point: Point, kind: EntityKind) -> Self {
        Self {
            point,
            kind,
            stats: Stats {
                max_hp: 0,
                cur_hp: 0,
                atk: 0,
                def: 0,
            },
        }
    }

    pub fn take_damage(&mut self, dmg: i32) -> bool {
        self.stats.cur_hp -= dmg;
        if self.stats.cur_hp <= 0 { true } else { false }
    }
}
