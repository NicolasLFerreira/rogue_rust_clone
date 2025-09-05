use crate::entities::entity::Stats;
use crate::state::State;
use crate::types::Id;

pub struct Combat;

impl Combat {
    pub fn initiate(game: &mut State, offender: Id, defender: Id) {
        if offender == defender {
            return;
        }

        // Immutable borrow of the offender stats otherwise borrow-checker will act up
        let offender_stats = if let Some(entity) = game.entity_manager.get_entity(offender) {
            entity.stats.clone()
        } else {
            return;
        };

        // Defender stats are mutable for in-place change
        let defender_stats = if let Some(entity) = game.entity_manager.get_entity_mut(defender) {
            &mut entity.stats
        } else {
            return;
        };

        // Perform actual attack
        if Self::attack(&offender_stats, defender_stats) {
            game.entity_manager.despawn(defender);
        }
    }

    pub fn attack(offender: &Stats, defender: &mut Stats) -> bool {
        defender.cur_hp -= offender.atk;
        defender.cur_hp <= 0
    }
}
