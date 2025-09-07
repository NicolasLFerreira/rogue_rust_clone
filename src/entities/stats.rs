#[derive(Clone, Debug)]
pub struct Stats {
    pub max_hp: i32,
    pub cur_hp: i32,
    pub atk: i32,
    pub def: i32,
}

impl Stats {
    pub const EMPTY: Self = Stats {
        max_hp: 0,
        cur_hp: 0,
        atk: 0,
        def: 0,
    };
}