use crate::entities::entity::Entity;

pub struct Combat;

impl Combat {
    pub fn attack(offender: &Entity, defender: &mut Entity) -> bool {
        defender.stats.cur_hp -= offender.stats.atk;
        defender.stats.cur_hp <= 0
    }
}
