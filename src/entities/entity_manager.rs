use crate::entities::entity::Entity;
use crate::types::Id;
use std::collections::HashMap;

pub struct EntityManager {
    entities: HashMap<usize, Entity>,
    player_id: Id,
    entity_id_counter: Id,
}

// Constructor
impl EntityManager {
    pub fn new(player: Entity) -> Self {
        let mut hashmap: HashMap<Id, Entity> = HashMap::new();
        hashmap.insert(0, player);
        Self {
            entities: hashmap,
            player_id: 0,
            entity_id_counter: 1,
        }
    }
}

// Queries
impl EntityManager {
    pub fn spawn(&mut self, entity: Entity) {
        self.entities.insert(self.entity_id_counter, entity);
        self.entity_id_counter += 1;
    }

    pub fn despawn(&mut self, id: Id) {
        self.entities.remove(&id);
    }

    pub fn player_id(&self) -> Id {
        self.player_id
    }

    pub fn get_entity(&self, id: Id) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut(&mut self, id: Id) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.values_mut()
    }
}
