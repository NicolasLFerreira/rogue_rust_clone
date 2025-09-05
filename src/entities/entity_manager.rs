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
    pub fn new(mut player: Entity) -> Self {
        let mut _self = Self {
            entities: HashMap::new(),
            player_id: 0,
            entity_id_counter: 0,
        };

        let player_id = _self.allocate_id();
        player.assign_id(player_id);
        _self.entities.insert(player_id, player);

        _self
    }
}

// Queries
impl EntityManager {
    pub fn allocate_id(&mut self) -> Id {
        let current = self.entity_id_counter;
        self.entity_id_counter += 1;
        current
    }

    pub fn spawn(&mut self, mut entity: Entity) {
        let entity_id = self.allocate_id();
        entity.assign_id(entity_id);
        self.entities.insert(entity_id, entity);
    }

    pub fn despawn(&mut self, id: Id) {
        self.entities.remove(&id);
    }

    pub fn player_id(&self) -> Id {
        self.player_id
    }

    pub fn player(&self) -> &Entity {
        self.entities.get(&self.player_id).unwrap()
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
